use std::fs;

use fst::Result;
use fst::{MapBuilder, SetBuilder};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use uuid::Uuid;
fn main() -> Result<()> {
    // dickens
    let mut terms = vec![];
    let file = File::open("dickens.txt")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            terms.push(line.to_string());
        }
    }

    // uuid
    //let my_uuid = Uuid::new_v4();
    //println!("{}", my_uuid.to_simple());
    //let mut terms = vec![];
    //for _ in 0..256_000 {
    //terms.push(Uuid::new_v4().to_simple().to_string());
    //}

    terms.sort();
    terms.dedup();
    let total_size = terms.iter().map(|uuid| uuid.len()).sum::<usize>();
    dbg!(total_size);
    dbg!(terms.len());

    // This is where we'll write our map to.
    let map_wtr = io::BufWriter::new(File::create("map.fst")?);
    let set_wtr = io::BufWriter::new(File::create("fst.fst")?);

    // Create a builder that can be used to insert new key-value pairs.
    let mut mapbuild = MapBuilder::new(map_wtr)?;
    let mut setbuild = SetBuilder::new(set_wtr)?;
    for (id, term) in terms.iter().enumerate() {
        mapbuild.insert(term, id as u64).unwrap();
        setbuild.insert(term).unwrap();
    }

    mapbuild.finish()?;
    setbuild.finish()?;
    Ok(())
}
