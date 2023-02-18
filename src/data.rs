use std::fmt::Display;

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
pub struct Semester<'a>(pub Vec<&'a Class>);

#[derive(Clone)]
pub struct Schedule<'a> {
    remaining: Vec<&'a Class>,
    semesters: Vec<&'a Semester<'a>>,
}

impl<'a> Semester<'a> {
    pub fn new(classes: Vec<&'a Class>) -> Self {
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

impl<'a> From<Vec<&'a Class>> for Semester<'a> {
    fn from(value: Vec<&'a Class>) -> Self {
        Semester::new(value)
    }
}

impl<'a> FromIterator<&'a Class> for Semester<'a> {
    fn from_iter<T: IntoIterator<Item = &'a Class>>(iter: T) -> Self {
        let mut semester = Semester::new(Vec::new());
        for class in iter {
            semester.0.push(class);
        }
        semester
    }
}

impl<'a> Display for Semester<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names: Vec<String> = self.0.iter().map(|class| class.name()).collect();
        write!(f, "{}", names.join(", "))
    }
}

impl<'a> Schedule<'a> {
    pub fn new(classes: &'a Vec<Class>) -> Self {
        let mut sched = Schedule {
            remaining: Vec::new(),
            semesters: Vec::new(),
        };
        sched.remaining = classes.iter().collect();
        sched
    }

    pub fn child<'b>(&self, semester: &'b Semester) -> Schedule<'b>
    where
        'a: 'b,
    {
        let remaining: Vec<&Class> = self
            .remaining
            .clone()
            .into_iter()
            .filter(|class| !semester.0.contains(class))
            .collect();

        let mut semesters: Vec<&'b Semester<'b>> = self.semesters.clone();
        semesters.push(semester);

        Schedule {
            remaining,
            semesters,
        }
    }

    pub fn generate_possible(&self) -> Vec<Semester> {
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
            .collect()
    }
}

impl<'a> Display for Schedule<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Schedule {{")?;
        for (i, semester) in self.semesters.iter().enumerate() {
            writeln!(f, "\t{}: {} ({} credits)", i, semester, semester.credits())?;
        }
        writeln!(f, "}}")
    }
}
