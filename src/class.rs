use crate::data::{Schedule, Term};
use crate::requirements::{parse, EvalExpression, Expression};

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Class {
    subject: String,
    number: u16,
    pub credits: u8,
    pub required: bool,
    groups: String,
    pub semesters: String,
    requisites: String,
    #[serde(skip)]
    parsed_reqs: Option<Expression>,
}

impl PartialOrd for Class {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.name().cmp(&other.name()))
    }
}

impl Ord for Class {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name().cmp(&other.name())
    }
}

impl Eq for Class {}

impl Class {
    pub fn name(&self) -> String {
        format!("{} {}", self.subject, self.number)
    }

    pub fn requisites_met(&self, schedule: &Schedule) -> bool {
        parse(&self.requisites).eval(schedule)
    }

    pub fn offered(&self, term: &Term) -> bool {
        self.semesters.split("|").any(|sem| term.matches(sem))
    }

    pub fn groups(&self) -> Vec<&str> {
        if self.groups.is_empty() {
            return Vec::new();
        }
        self.groups.split("|").collect()
    }
}
