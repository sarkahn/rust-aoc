use std::fs::File;
use std::io::{prelude::*, BufReader, Error, ErrorKind};
use std::path::Path;

fn lines_to_vec(filename: impl AsRef<Path>) -> Result<Vec<u32>, Error> {
    let file = File::open(filename).expect("Unable to open file");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() -> Result<(), Error> {
    let lines = lines_to_vec("data/input.txt").expect("Error reading file");

    for a in lines.iter() {
        for b in lines.iter() {
            if a + b == 2020 {
                println!("{} * {} == {}", a, b, a * b);
            }
        }
    }

    Ok(())
}



