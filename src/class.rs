use crate::data::Error;

use serde::{de::Error as DeError, Deserialize, Deserializer};

#[derive(Debug, Deserialize, PartialEq)]
pub struct Class {
    subject: String,
    number: u16,
    pub credits: u8,
    requisites: String,
}

impl PartialOrd for Class {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.credits.cmp(&other.credits))
    }
}

impl Ord for Class {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.credits.cmp(&other.credits)
    }
}

impl Eq for Class {}

impl Class {
    pub fn name(&self) -> String {
        format!("{}{}", self.subject, self.number)
    }
}
