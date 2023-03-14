#![feature(iterator_try_collect)]
#![feature(iter_collect_into)]
#![feature(get_many_mut)]
mod class;
mod data;
mod requirements;

use std::rc::Rc;

use csv::Reader;

use class::Class;
use data::Schedule;

fn step_schedules<'a>(input: Vec<Schedule>) -> Vec<Schedule> {
    input
        .into_iter()
        .map(|sched| {
            sched
                .generate_possible()
                .into_iter()
                .map(|sem| sched.child(sem).into())
                .collect::<Vec<Schedule>>()
        })
        .flatten()
        .collect::<Vec<Schedule>>()
}

fn main() -> Result<(), data::Error> {
    let mut rdr = Reader::from_path("input.csv").unwrap();

    let classes: Vec<Rc<Class>> = rdr
        .deserialize()
        .map(|x| Rc::new(x.unwrap()))
        .collect::<Vec<Rc<Class>>>();

    let mut scheds: Vec<Schedule> = vec![Schedule::new(&classes)];

    for _ in 0..4 {
        scheds = step_schedules(scheds);
    }

    scheds.iter().for_each(|x| println!("{}", x));

    Ok(())
}
