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
pub enum SemesterWrap<'a> {
    Some(Semester<'a>),
    None,
}

#[derive(Clone)]
pub struct Schedule<'a, 'b> {
    remaining: Vec<&'a Class>,
    pub semesters: Vec<&'b SemesterWrap<'a>>,
    own: SemesterWrap<'a>,
}

impl<'a> Semester<'a> {
    pub fn new(classes: Vec<&'a Class>) -> Self {
        Semester(classes)
    }

    pub fn credits(&self) -> u16 {
        self.0.iter().map(|class| class.credits as u16).sum()
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

pub struct Intermediate<'a, 'b>(pub Semester<'a>, pub Schedule<'a, 'b>);

impl<'a, 'b> Schedule<'a, 'b> {
    pub fn new(classes: &'a Vec<Class>) -> Self {
        let mut sched = Schedule {
            remaining: Vec::new(),
            semesters: Vec::new(),
            own: SemesterWrap::None,
        };
        sched.remaining = classes.iter().collect();
        sched
    }

    pub fn child<'c>(&'c self, semester: Semester<'a>) -> Schedule<'a, 'b>
    where
        'c: 'b,
    {
        let remaining: Vec<&'a Class> = self
            .remaining
            .clone()
            .into_iter()
            .filter(|class| !semester.0.contains(class))
            .collect();

        let mut semesters: Vec<&'b SemesterWrap<'a>> = self.semesters.clone();
        semesters.push(&self.own);

        Schedule {
            remaining,
            semesters,
            own: SemesterWrap::Some(semester),
        }
        // Intermediate(semester, sched)
    }

    pub fn generate_children<'c>(&'c self) -> Vec<Schedule<'a, 'c>> {
        self.generate_possible()
            .into_iter()
            .map(|sem| self.child(sem))
            .collect()
    }

    fn generate_possible(&self) -> Vec<Semester<'a>> {
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
        for x in sorted.iter() {
            min += 1;
            accum += x.credits;
            if accum >= 12 {
                break;
            }
        }

        (min..max)
            .into_iter()
            .map(|i| {
                let clone: Vec<&'a Class> = self.remaining.clone();
                Combinations::new(clone, i)
            })
            .flatten()
            .map(|x| x.into())
            .filter(|x: &Semester| x.is_valid())
            .collect()
    }
}

impl<'a> Display for Schedule<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Schedule {{")?;
        for (i, semester) in self.semesters.iter().enumerate() {
            if let SemesterWrap::Some(semester) = semester {
                writeln!(f, "\t{}: {} ({} credits)", i, semester, semester.credits())?;
            }
        }
        writeln!(f, "}}")
    }
}
