#![feature(iterator_try_collect)]
#![feature(iter_collect_into)]
#![feature(get_many_mut)]
mod class;
mod config;
mod data;
mod requirements;

use std::{fs, rc::Rc};

use config::Config;
use csv::ReaderBuilder;

use class::Class;
use data::{Schedule, ScheduleError};
use itertools::Itertools;

fn step_schedules<'a>(
    input: Vec<Schedule>,
    config: &Config,
) -> Vec<Result<Schedule, ScheduleError>> {
    input
        .into_iter()
        .map(|sched| {
            sched
                .generate_possible(config)
                .into_iter()
                .map(|sem_result| match sem_result {
                    Ok(sem) => sched.child(sem),
                    Err(err) => Err(err),
                })
                .collect::<Vec<Result<Schedule, ScheduleError>>>()
        })
        .flatten()
        .collect::<Vec<Result<Schedule, ScheduleError>>>()
}

fn split_result_vec<T, E>(results: Vec<Result<T, E>>) -> (Vec<T>, Vec<E>) {
    let mut successes = vec![];
    let mut errors = vec![];

    for result in results {
        match result {
            Ok(value) => successes.push(value),
            Err(error) => errors.push(error),
        }
    }

    (successes, errors)
}

fn main() {
    let mut rdr = ReaderBuilder::new()
        .comment(Some(b'#'))
        .from_path("input.csv")
        .unwrap();

    let classes_taken: Vec<String> = fs::read_to_string("taken.txt")
        .unwrap()
        .lines()
        .filter(|line| !line.starts_with('#'))
        .map(|line| line.to_owned())
        .collect();

    let config_str = fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(&config_str).unwrap();

    let classes: Vec<Rc<Class>> = rdr
        .deserialize()
        .map(|x| Rc::new(x.unwrap()))
        .collect::<Vec<Rc<Class>>>();

    let mut scheds: Vec<Schedule> = vec![Schedule::new(
        &classes,
        Rc::new(classes_taken),
        config.starting_term,
    )];
    let mut errors: Vec<ScheduleError> = Vec::new();

    for _ in 0..(config.semesters) {
        let (scheds_split, errors_split) = split_result_vec(step_schedules(scheds, &config));
        scheds = scheds_split;
        errors.extend(errors_split);
    }

    scheds.iter().for_each(|x| println!("{}", x));
    println!("Errors:");
    let err_counts = errors.iter().counts();
    err_counts
        .iter()
        .sorted_by(|a, b| (b.1).cmp(a.1))
        .for_each(|(&err, count)| println!("{count}: {err}"));
}
