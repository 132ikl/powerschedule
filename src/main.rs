#![feature(iterator_try_collect)]
#![feature(iter_collect_into)]
#![feature(get_many_mut)]

mod class;
mod error;
mod schedule;
mod semester;

use csv::Reader;

use class::{Class, ClassList};
use error::{Error, Result};
use schedule::Schedule;
use semester::Semester;

// fn step_schedules<'a, 'b>(
//     input: &'a Vec<Schedule<'a>>,
//     sems: &'b mut Vec<(&'b Schedule<'b>, Semester<'b>)>,
// ) -> Vec<Schedule<'b>>
// where
//     'a: 'b,
// {
//     let mut scheds: Vec<Schedule> = Vec::new();
//     let tail = sems.len();
//     for sched in input.iter() {
//         for sem in sched.generate_possible().into_iter() {
//             sems.push((&sched, sem));
//         }
//     }
//     for (sched, sem) in sems.iter().skip(tail) {
//         scheds.push(sched.child(sem));
//     }
//     scheds
// }

fn main() -> Result<()> {
    let mut rdr = Reader::from_path("input.csv").unwrap();

    let classes: Vec<Class> = rdr
        .deserialize()
        .try_collect()
        .map_err(|_| Error::ConvertError("Failed to deserialize classes".to_string()))?;

    // dbg!(&classes);

    let classes = ClassList::new(classes);

    // let mut scheds: Vec<Vec<Schedule>> = Vec::new();
    // let mut sems: Vec<Vec<(&Schedule, Semester)>> = Vec::new();
    let top: Vec<Schedule> = vec![Schedule::new(classes.all())];

    let x = top[0].generate_possible(&classes);

    for (i, x) in x.into_iter().enumerate() {}

    // (0..8).for_each(|_| sems.push(Vec::new()));
    // let [a, b, c, d, e, f, g, h] = sems.get_many_mut([0, 1, 2, 3, 4, 5, 6, 7]).unwrap();
    // scheds.push(vec![Schedule::new(&classes)]);

    // let one = step_schedules(&top, a);
    // let two = step_schedules(&one, b);
    // let three = step_schedules(&two, c);
    // let four = step_schedules(&three, d);
    // let five = step_schedules(&four, e);
    // let six = step_schedules(&five, f);
    // let seven = step_schedules(&six, g);
    // let eight = step_schedules(&seven, h);
    // five.iter().for_each(|a| println!("{}", a));

    // Ok(())
    Ok(())
}
