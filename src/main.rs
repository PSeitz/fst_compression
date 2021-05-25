use fst::Result;
use fst::{MapBuilder, SetBuilder};
use uuid::Uuid;

fn main() -> Result<()> {
    let my_uuid = Uuid::new_v4();
    println!("{}", my_uuid.to_simple());

    let mut uuids = vec![];

    for _ in 0..256_000 {
        uuids.push(Uuid::new_v4().to_simple().to_string());
    }

    let total_size = uuids.iter().map(|uuid| uuid.len()).sum::<usize>();
    dbg!(total_size);
    uuids.sort();

    use std::fs::File;
    use std::io;

    // This is where we'll write our map to.
    let map_wtr = io::BufWriter::new(File::create("map.fst")?);
    let set_wtr = io::BufWriter::new(File::create("fst.fst")?);

    // Create a builder that can be used to insert new key-value pairs.
    let mut mapbuild = MapBuilder::new(map_wtr)?;
    let mut setbuild = SetBuilder::new(set_wtr)?;
    for (id, term) in uuids.iter().enumerate() {
        mapbuild.insert(term, id as u64).unwrap();
        setbuild.insert(term).unwrap();
    }

    mapbuild.finish()?;
    setbuild.finish()?;
    Ok(())
}
