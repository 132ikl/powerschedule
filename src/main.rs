#![feature(iterator_try_collect)]
#![feature(iter_collect_into)]
#![feature(get_many_mut)]
mod class;
mod data;
mod requirements;

use std::rc::Rc;

use csv::Reader;

use class::Class;
use data::{Schedule, Term, TermSeason};

fn step_schedules<'a>(input: Vec<Schedule>) -> Vec<Schedule> {
    input
        .into_iter()
        .map(|sched| {
            sched
                .generate_possible()
                .into_iter()
                .filter_map(|sem| sched.child(sem))
                .collect::<Vec<Schedule>>()
        })
        .flatten()
        .collect::<Vec<Schedule>>()
}

fn main() {
    let mut rdr = Reader::from_path("input.csv").unwrap();

    let classes_taken: Vec<String> = vec![
        "PHY 183".to_string(),
        "PHY 184".to_string(),
        "ECE 201".to_string(),
        "ECE 390".to_string(),
        "CSE 331".to_string(),
        "MTH 132".to_string(),
        "MTH 133".to_string(),
        "MTH 234".to_string(),
        "CSE 231".to_string(),
        "CSE 232".to_string(),
        "CSE 260".to_string(),
    ];

    let classes: Vec<Rc<Class>> = rdr
        .deserialize()
        .map(|x| Rc::new(x.unwrap()))
        .collect::<Vec<Rc<Class>>>();

    let mut scheds: Vec<Schedule> = vec![Schedule::new(
        &classes,
        Rc::new(classes_taken),
        Term::new(TermSeason::Fall, 2023),
    )];

    for _ in 0..4 {
        scheds = step_schedules(scheds);
    }

    scheds
        .iter()
        .filter(|sched| sched.is_complete(&classes))
        .for_each(|x| println!("{}", x));
}
