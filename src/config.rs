use std::collections::BTreeMap;

use serde::Deserialize;

use crate::data::Term;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub min_credits: u8,
    pub max_credits: u8,
    pub semesters: u8,
    pub starting_term: Term,
    pub groups: BTreeMap<String, u8>,
}
