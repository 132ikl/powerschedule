use std::fmt;
use std::{fmt::Display, rc::Rc};

use crate::requirements::TestRequisite;
use crate::{class::Class, requirements::RequisiteName};

use combinations::Combinations;

pub struct Semester(pub Vec<Rc<Class>>, pub Term);

#[derive(Debug, Copy, Clone)]
pub enum TermSeason {
    Spring,
    #[allow(unused)]
    Summer,
    Fall,
}

#[derive(Debug, Copy, Clone)]
pub struct Term {
    pub season: TermSeason,
    pub year: u16,
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
        self.0
            .iter()
            .fold(0, |acc, class| acc + (class.credits as u16))
    }

    pub fn is_valid(&self) -> bool {
        let credits = self.credits();
        if credits < 12 || credits > 20 {
            return false;
        }

        if !self.0.iter().all(|class| class.offered(&self.1)) {
            return false;
        }

        return true;
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

    pub fn is_valid(&self) -> bool {
        if !self
            .semesters
            .iter()
            .map(|x| x.0.clone())
            .flatten()
            .all(|class| class.requisites_met(self))
        {
            return false;
        };

        return true;
    }

    #[allow(unused)]
    pub fn is_complete(&self) -> bool {
        !self.remaining.iter().any(|class| class.required)
    }

    pub fn child(&self, semester: Rc<Semester>) -> Option<Schedule> {
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
            true => Some(new),
            false => None,
        }
    }

    pub fn generate_possible(&self) -> Vec<Rc<Semester>> {
        let mut sorted = self.remaining.clone();
        sorted.sort_unstable_by_key(|x| x.credits);

        let mut accum = 0;
        let mut max = 0;
        for val in sorted.iter() {
            max += 1;
            accum += val.credits;
            if accum >= 20 {
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
            if accum >= 12 {
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
        candidates.push((self.remaining.clone(), term).into());

        candidates
            .into_iter()
            .filter(|x: &Semester| x.is_valid())
            .map(|x| Rc::new(x))
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
            writeln!(
                f,
                "{} {}: {} ({} credits)",
                semester.1.season,
                semester.1.year,
                semester,
                semester.credits()
            )?;
        }
        writeln!(
            f,
            "Remaining: {}",
            self.remaining
                .iter()
                .map(|x| x.name())
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        writeln!(f)
    }
}
