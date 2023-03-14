use std::{fmt::Display, rc::Rc};

use crate::class::Class;

use bitvec::{order::Lsb0, view::BitView};
use combinations::Combinations;

#[derive(Debug)]
pub enum Error {
    ConvertError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct Semester(pub Vec<Rc<Class>>);

#[derive(Clone)]
pub struct Schedule {
    remaining: Vec<Rc<Class>>,
    semesters: Vec<Rc<Semester>>,
}

impl Semester {
    pub fn new(classes: Vec<Rc<Class>>) -> Self {
        Semester(classes)
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

        return true;
    }
}

impl From<Vec<Rc<Class>>> for Semester {
    fn from(value: Vec<Rc<Class>>) -> Self {
        Semester::new(value)
    }
}

impl FromIterator<Rc<Class>> for Semester {
    fn from_iter<T: IntoIterator<Item = Rc<Class>>>(iter: T) -> Self {
        let mut semester = Semester::new(Vec::new());
        for class in iter {
            semester.0.push(class);
        }
        semester
    }
}

impl Display for Semester {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names: Vec<String> = self.0.iter().map(|class| class.name()).collect();
        write!(f, "{}", names.join(", "))
    }
}

impl Schedule {
    pub fn new(classes: &Vec<Rc<Class>>) -> Self {
        let mut sched = Schedule {
            remaining: Vec::new(),
            semesters: Vec::new(),
        };
        sched.remaining = classes.clone();
        sched
    }

    pub fn child(&self, semester: Rc<Semester>) -> Schedule {
        let remaining: Vec<Rc<Class>> = self
            .remaining
            .clone()
            .into_iter()
            .filter(|class| !semester.0.contains(class))
            .collect();

        let mut semesters: Vec<Rc<Semester>> = self.semesters.clone();
        semesters.push(semester.clone());

        Schedule {
            remaining,
            semesters,
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

        (min..max)
            .into_iter()
            .map(|i| Combinations::new(self.remaining.clone(), i))
            .flatten()
            .map(|x| x.into())
            .filter(|x: &Semester| x.is_valid())
            .map(|x| Rc::new(x))
            .collect()
    }
}

impl Display for Schedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Schedule {{")?;
        for (i, semester) in self.semesters.iter().enumerate() {
            writeln!(f, "\t{}: {} ({} credits)", i, semester, semester.credits())?;
        }
        writeln!(f, "}}")
    }
}
