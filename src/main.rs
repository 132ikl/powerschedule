#![feature(iterator_try_collect)]
#![feature(iter_collect_into)]
#![feature(get_many_mut)]
mod class;
mod data;

use csv::Reader;

use class::Class;
use data::{Error, Schedule, Semester};

// fn step_schedule<'a, 'b>(
//     schedule: Schedule<'a, 'b>,
//     sems: &'b mut Vec<Semester<'a>>,
//     scheds: &mut Vec<Schedule<'a, 'b>>,
// ) {
// }

fn main() -> Result<(), data::Error> {
    let mut rdr = Reader::from_path("input.csv").unwrap();

    let classes: Vec<Class> = rdr
        .deserialize()
        .try_collect()
        .map_err(|_| Error::ConvertError("Failed to deserialize classes".to_string()))?;

    // let mut old: Vec<Schedule> = Vec::new();
    let mut schedules: Vec<Schedule> = vec![Schedule::new(&classes)];
    // let mut semesters: Vec<Semester> = Vec::new();
    // let mut schedules: Vec<Schedule> = Vec::new();
    //

    let tail = 0;
    let schedule = &schedules[tail];
    for i in schedule.generate_children() {
        schedules.push(i);
    }

    // for i in 0..5 {
    //     let mut new: Vec<Schedule> = Vec::new();
    //     for sched in  {
    //         sched
    //             .generate_children()
    //             .into_iter()
    //             .for_each(|x| new.push(x));
    //     }
    //     to_process = new;
    // }

    // for ((sched, sems), scheds) in to_process
    //     .into_iter()
    //     .zip(semesters.iter_mut())
    //     .zip(schedules.iter_mut())
    // {
    //     step_schedule(sched, sems, scheds)
    // }
    // to_process = schedules.into_iter().flatten().collect();

    Ok(())
}
