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

    pub fn generate_possible(&self, classlist: &ClassList) -> Vec<Semester> {
        let mut sorted = self.remaining.clone();
        sorted.sort_unstable_by_key(|&idx| classlist[idx].credits);

        let mut accum = 0;
        let mut max = 0;
        for &idx in &sorted {
            max += 1;
            accum += classlist[idx].credits;
            if accum >= 20 {
                break;
            }
        }

        let mut accum = 0;
        sorted.reverse();
        let mut min = 0;
        for &idx in &sorted {
            min += 1;
            accum += classlist[idx].credits;
            if accum >= 12 {
                break;
            }
        }

        (min..max)
            .into_iter()
            .map(|i| Combinations::new(self.remaining.clone(), i))
            .flatten()
            .map(|x| x.into())
            .filter(|x: &Semester| x.is_valid(classlist))
            .collect()
    }

    pub fn print<'a>(
        &'a self,
        classlist: &'a ClassList,
        semesterlist: &'a SemesterList,
    ) -> ScheduleDisplay<'a> {
        ScheduleDisplay {
            schedule: self,
            classlist,
            semesterlist,
        }
    }
}

pub struct ScheduleDisplay<'a> {
    schedule: &'a Schedule,
    classlist: &'a ClassList,
    semesterlist: &'a SemesterList,
}

impl<'a> std::fmt::Display for ScheduleDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Schedule {{")?;
        for (i, &semesteridx) in self.schedule.semesters.iter().enumerate() {
            let semester = &self.semesterlist[semesteridx];
            let semesterprint = semester.print(self.classlist);
            let credits = semester.credits(self.classlist);
            writeln!(f, "\t{i}: {semesterprint} ({credits} credits)",)?;
        }
        writeln!(f, "}}")
    }
}
