use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub type Lines = Vec<String>;

pub fn read_lines<P>(filename: P) -> Result<Lines>
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
