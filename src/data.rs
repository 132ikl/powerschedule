use std::collections::BTreeMap;
use std::fmt;
use std::{fmt::Display, rc::Rc};

use crate::config::Config;
use crate::requirements::TestRequisite;
use crate::{class::Class, requirements::RequisiteName};

use combinations::Combinations;
use serde::Deserialize;
use thiserror::Error;
use yansi::{Paint, Painted};

pub struct Semester(pub Vec<Rc<Class>>, pub Term);

#[derive(Error, Debug, PartialEq, Eq, Hash)]
pub enum ScheduleError {
    #[error("Too few credits")]
    TooFewCredits,
    #[error("Too many credits")]
    TooManyCredits,
    #[error("Not available in term {0}")]
    NotAvailable(String),
    #[error("Requisites for class un-met")]
    RequisitesUnmet,
    #[error("Doesn't fulfill required courses")]
    RequirementsUnmet,
    #[error("Did not meet credit requirement for group")]
    GroupsUnmet,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum TermSeason {
    Spring,
    #[allow(unused)]
    Summer,
    Fall,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct Term {
    pub season: TermSeason,
    pub year: u16,
}

impl Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?} {}", self.season, self.year))
    }
}

impl Term {
    pub fn new(season: TermSeason, year: u16) -> Self {
        Self { season, year }
    }

    pub fn next(&self) -> Self {
        match self.season {
            TermSeason::Spring => Term::new(TermSeason::Fall, self.year),
            TermSeason::Fall => Term::new(TermSeason::Spring, self.year + 1),
            TermSeason::Summer => unimplemented!(),
        }
    }

    pub fn matches(&self, other: &str) -> bool {
        let name = self.season.to_string();
        if other == name {
            return true;
        };
        if other.starts_with(&name) {
            if other.ends_with("Odd") {
                return !(self.year % 2 == 0);
            }
            if other.ends_with("Even") {
                return self.year % 2 == 0;
            }
        }
        return false;
    }
}

impl Display for TermSeason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Schedule {
    remaining: Vec<Rc<Class>>,
    semesters: Vec<Rc<Semester>>,
    taken: Rc<Vec<String>>,
    first_term: Term,
}

impl Semester {
    pub fn new(classes: Vec<Rc<Class>>, term: Term) -> Self {
        Semester(classes, term)
    }

    pub fn credits(&self) -> u16 {
        self.0.iter().map(|class| class.credits as u16).sum()
    }

    pub fn verify(self, config: &Config) -> Result<Rc<Self>, ScheduleError> {
        let credits = self.credits();
        if credits < config.min_credits.into() {
            return Err(ScheduleError::TooFewCredits);
        }
        if credits > config.max_credits.into() {
            return Err(ScheduleError::TooManyCredits);
        }

        if !self.0.iter().all(|class| class.offered(&self.1)) {
            return Err(ScheduleError::NotAvailable(self.1.to_string()));
        }

        return Ok(Rc::new(self));
    }
}

impl From<(Vec<Rc<Class>>, Term)> for Semester {
    fn from((value, term): (Vec<Rc<Class>>, Term)) -> Self {
        Semester::new(value, term)
    }
}

impl Display for Semester {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names: Vec<String> = self.0.iter().map(|class| class.name()).collect();
        write!(f, "{}", names.join(", "))
    }
}

impl Schedule {
    pub fn new(classes: &Vec<Rc<Class>>, taken: Rc<Vec<String>>, term: Term) -> Self {
        let mut sched = Schedule {
            remaining: Vec::new(),
            semesters: Vec::new(),
            taken: taken.clone(),
            first_term: term,
        };
        sched.remaining = classes.clone();
        sched
    }

    pub fn is_valid(&self) -> Result<(), ScheduleError> {
        let mut classes = self.semesters.iter().map(|x| x.0.clone()).flatten();
        if !classes.all(|class| class.requisites_met(self)) {
            return Err(ScheduleError::RequisitesUnmet);
        };

        return Ok(());
    }

    pub fn total_credits(&self) -> u16 {
        self.semesters
            .iter()
            .map(|x| x.0.clone())
            .flatten()
            .map(|x| x.credits as u16)
            .sum()
    }

    pub fn meets_group_credits(&self, config: &Config) -> bool {
        let classes = self.semesters.iter().map(|x| x.0.clone()).flatten();
        let mut groups: BTreeMap<String, u8> = config.groups.clone();
        groups.values_mut().for_each(|v| *v = 0);

        // this is definitely not efficient lol
        for class in classes {
            for group in class.groups() {
                let group_val = groups
                    .get_mut(group)
                    .expect("Class has a group which is not present in config");
                *group_val += class.credits;
            }
        }
        // safety: will keys always be in same order? we're cloning, so it seems like yes, but not actually doing anything to ensure that
        groups
            .iter()
            .zip(&config.groups)
            .all(|(this, other)| this.1 >= other.1)
    }

    pub fn is_complete(&self, config: &Config) -> Result<(), ScheduleError> {
        if self.remaining.iter().any(|class| class.required) {
            return Err(ScheduleError::RequirementsUnmet);
        };
        if !self.meets_group_credits(&config) {
            return Err(ScheduleError::GroupsUnmet);
        }
        Ok(())
    }

    pub fn completeness_display(&self, config: &Config) -> Painted<&str> {
        match self.is_complete(&config) {
            Ok(_) => "Yes".green(),
            Err(ScheduleError::RequirementsUnmet) => "No, requirements unmet".red(),
            Err(ScheduleError::GroupsUnmet) => "No, group credit requirement unmet".red(),
            Err(_) => panic!("Unknown completeness error"),
        }
    }

    pub fn child(&self, semester: Rc<Semester>) -> Result<Schedule, ScheduleError> {
        let remaining: Vec<Rc<Class>> = self
            .remaining
            .clone()
            .into_iter()
            .filter(|class| !semester.0.contains(class))
            .collect();

        let mut semesters: Vec<Rc<Semester>> = self.semesters.clone();
        semesters.push(semester.clone());

        let new = Schedule {
            remaining,
            semesters,
            taken: self.taken.clone(),
            first_term: self.first_term,
        };

        match new.is_valid() {
            Ok(_) => Ok(new),
            Err(err) => Err(err),
        }
    }

    pub fn generate_possible(&self, config: &Config) -> Vec<Result<Rc<Semester>, ScheduleError>> {
        let mut sorted = self.remaining.clone();
        sorted.sort_unstable_by_key(|x| x.credits);

        let mut accum = 0;
        let mut max = 0;
        for val in sorted.iter() {
            max += 1;
            accum += val.credits;
            if accum >= config.max_credits {
                break;
            }
        }
        max = std::cmp::min(max, self.remaining.len() - 1);

        accum = 0;
        sorted.reverse();
        let mut min = 0;
        for x in sorted {
            min += 1;
            accum += x.credits;
            if accum >= config.min_credits {
                break;
            }
        }

        let term = match self.semesters.last() {
            Some(sem) => sem.1.next(),
            None => self.first_term,
        };

        // try subsets of remaining classes and all remaining classes
        let mut candidates: Vec<Semester> = (min..=max)
            .into_iter()
            .map(|i| Combinations::new(self.remaining.clone(), i))
            .flatten()
            .map(|x| (x, term).into())
            .collect();
        let remaining_credits: u16 = self.remaining.iter().map(|x| x.credits as u16).sum();
        if remaining_credits <= config.max_credits.into() {
            candidates.push((self.remaining.clone(), term).into());
        };

        candidates
            .into_iter()
            .map(|x: Semester| x.verify(config))
            .collect()
    }
}

impl TestRequisite for Schedule {
    fn has_prerequisite(&self, requisite: &RequisiteName) -> bool {
        if self.taken.contains(requisite) {
            return true;
        }

        if let Some((_, sems)) = self.semesters.split_last() {
            sems.iter()
                .map(|x| x.0.clone())
                .flatten()
                .any(|class| requisite == &class.name())
        } else {
            false
        }
    }

    fn has_corequisite(&self, requisite: &RequisiteName) -> bool {
        if self.taken.contains(requisite) {
            return true;
        }

        self.semesters
            .iter()
            .map(|x| x.0.clone())
            .flatten()
            .any(|class| requisite == &class.name())
    }
}

impl Display for Schedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for semester in self.semesters.iter() {
            write!(
                f,
                "{} {}{} ",
                semester.1.season.bold().blue(),
                semester.1.year.bold().blue(),
                ":".bold().blue()
            )?;
            write!(f, "{} ", semester.green())?;
            writeln!(
                f,
                "{}({} credits)",
                "".bright_black().linger(),
                semester.credits()
            )?;
        }
        write!(
            f,
            "{} {}",
            "Remaining:".bold().white().dim(),
            self.remaining
                .iter()
                .map(|x| x.name())
                .collect::<Vec<String>>()
                .join(", ")
                .white()
                .dim()
        )
    }
}
