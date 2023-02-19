#![feature(iterator_try_collect)]
#![feature(iter_collect_into)]
#![feature(get_many_mut)]

mod class;
mod error;
mod schedule;
mod semester;

use std::thread::current;

use csv::Reader;

use class::{Class, ClassList};
use error::{Error, Result};
use schedule::Schedule;
use semester::{Semester, SemesterList};

fn step_schedules(
    classlist: &ClassList,
    semesterlist: &mut SemesterList,
    schedules: &[Schedule],
) -> Vec<Schedule> {
    let mut new_schedules = vec![];

    for schedule in schedules {
        new_schedules.extend(schedule.generate_possible(classlist, semesterlist));
    }

    new_schedules
}

fn main() -> Result<()> {
    let mut rdr = Reader::from_path("input.csv").unwrap();

    let classes: Vec<Class> = rdr
        .deserialize()
        .try_collect()
        .map_err(|_| Error::ConvertError("Failed to deserialize classes".to_string()))?;

    // dbg!(&classes);

    let classlist = ClassList::new(classes);

    let mut semesterlist = SemesterList::new();
    let mut current_depth: Vec<Schedule> = vec![Schedule::new_toplevel(&classlist)];

    let max_depth = 8;

    for depth in 0..max_depth {
        println!("Depth {depth}:");
        current_depth = step_schedules(&classlist, &mut semesterlist, &current_depth);

        dbg!(current_depth.len());
        // for (i, s) in current_depth.iter().enumerate() {
        // println!("  {i} {}", s.print(&classlist, &semesterlist));
        // }
        println!();
    }

    Ok(())
}
