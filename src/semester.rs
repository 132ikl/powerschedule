use std::ops::{Index, IndexMut};

use crate::class::{ClassIdx, ClassList};

#[derive(Clone)]
pub struct Semester(pub Vec<ClassIdx>);

impl Semester {
    pub fn new(classes: Vec<ClassIdx>) -> Self {
        Semester(classes)
    }

    pub fn credits(&self, classlist: &ClassList) -> u16 {
        self.0
            .iter()
            .map(|&idx| classlist[idx].credits as u16)
            .sum()
    }

    pub fn is_valid(&self, classlist: &ClassList) -> bool {
        let credits = self.credits(classlist);
        if credits < 12 || credits > 20 {
            return false;
        }

        return true;
    }
}

impl From<Vec<ClassIdx>> for Semester {
    fn from(value: Vec<ClassIdx>) -> Self {
        Semester::new(value)
    }
}

impl FromIterator<ClassIdx> for Semester {
    fn from_iter<T: IntoIterator<Item = ClassIdx>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

// impl<'a> Display for Semester<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let names: Vec<String> = self.0.iter().map(|class| class.name()).collect();
//         write!(f, "{}", names.join(", "))
//     }
// }

#[derive(Clone)]
pub struct SemesterList(Vec<Semester>);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemesterIdx(usize);

impl SemesterList {
    pub fn new(semesters: Vec<Semester>) -> Self {
        Self(semesters)
    }
}

impl Index<SemesterIdx> for SemesterList {
    type Output = Semester;

    fn index(&self, index: SemesterIdx) -> &Self::Output {
        &self.0[index.0]
    }
}

impl IndexMut<SemesterIdx> for SemesterList {
    fn index_mut(&mut self, index: SemesterIdx) -> &mut Self::Output {
        &mut self.0[index.0]
    }
}
