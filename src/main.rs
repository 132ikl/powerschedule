#![feature(iterator_try_collect)]
mod data;

use bitvec::prelude::*;
use csv::Reader;

use data::{Class, Error};

fn main() -> Result<(), data::Error> {
    let mut rdr = Reader::from_path("input.csv").unwrap();
    let classes: Vec<Class> = rdr
        .deserialize()
        .try_collect()
        .map_err(|_| Error::ConvertError("Failed to deserialize classes".to_string()))?;

    let mut power: Vec<Vec<&Class>> = Vec::new();
    let range = 2u32.pow(classes.len() as u32);

    for i in 0..range {
        let bits = i.view_bits::<Lsb0>();
        let out: Vec<&Class> = bits.iter_ones().map(|n| &classes[n]).collect();
        power.push(out);
    }

    power.iter().for_each(|set| {
        let names: Vec<String> = set.iter().map(|class| class.name()).collect();
        println!("{:?}", names);
    });

    Ok(())
}
