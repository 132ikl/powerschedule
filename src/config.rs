use std::collections::BTreeMap;

use serde::Deserialize;

use crate::data::Term;

const fn true_fn() -> bool {
    true // thank you serde very cool
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub min_credits: u8,
    pub max_credits: u8,
    pub semesters: u8,
    pub starting_term: Term,
    #[serde(default)]
    pub groups: BTreeMap<String, u8>,
    #[serde(default = "true_fn")]
    pub show_incomplete: bool,
}
