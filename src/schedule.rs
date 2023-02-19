use combinations::Combinations;

use crate::class::{ClassIdx, ClassList};
use crate::semester::{Semester, SemesterIdx, SemesterList};

#[derive(Clone)]
pub struct Schedule {
    remaining: Vec<ClassIdx>,
    semesters: Vec<SemesterIdx>,
}

impl Schedule {
    pub fn new(classes: Vec<ClassIdx>) -> Self {
        Schedule {
            remaining: classes,
            semesters: Vec::new(),
        }
    }

    pub fn child(&self, semester: SemesterIdx, semesters: &SemesterList) -> Schedule {
        let remaining: Vec<ClassIdx> = self
            .remaining
            .clone()
            .into_iter()
            .filter(|class| !semesters[semester].0.contains(class))
            .collect();

        let mut semesters = self.semesters.clone();
        semesters.push(semester);

        Schedule {
            remaining,
            semesters,
        }
    }

    pub fn generate_possible(&self, classes: &ClassList) -> Vec<Semester> {
        let mut sorted = self.remaining.clone();
        sorted.sort_unstable_by_key(|&x| classes[x].credits);

        let mut accum = 0;
        let mut max = 0;
        for &val in &sorted {
            max += 1;
            accum += classes[val].credits;
            if accum >= 20 {
                break;
            }
        }

        let mut accum = 0;
        sorted.reverse();
        let mut min = 0;
        for x in sorted {
            min += 1;
            accum += classes[x].credits;
            if accum >= 12 {
                break;
            }
        }

        (min..max)
            .into_iter()
            .map(|i| Combinations::new(self.remaining.clone(), i))
            .flatten()
            .map(|x| x.into())
            .filter(|x: &Semester| x.is_valid(classes))
            .collect()
    }
}

// impl<'a> Display for Schedule<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         writeln!(f, "Schedule {{")?;
//         for (i, semester) in self.semesters.iter().enumerate() {
//             writeln!(f, "\t{}: {} ({} credits)", i, semester, semester.credits())?;
//         }
//         writeln!(f, "}}")
//     }
// }
