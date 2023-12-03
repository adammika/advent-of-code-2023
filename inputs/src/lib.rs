use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub type Lines = Vec<String>;

pub fn day_one_lines() -> Result<Lines> {
    read_lines("day-1.txt")
}

pub fn day_two_lines() -> Result<Lines> {
    read_lines("day-2.txt")
}

fn read_lines<P>(filename: P) -> Result<Lines>
where
    P: AsRef<Path>,
{
    let dir = env!("CARGO_MANIFEST_DIR");
    let path = Path::new(dir);
    let file_path = path.join(filename);

    let file = File::open(file_path)?;
    BufReader::new(file)
        .lines()
        .collect::<Result<Vec<String>, io::Error>>()
        .map_err(Into::into)
}
