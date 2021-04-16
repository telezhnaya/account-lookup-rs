use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;

pub(crate) fn human(yocto: u128) -> u128 {
    yocto / 1000000000000000000000000
}

pub(crate) fn to_seconds(nanoseconds: u64) -> u64 {
    nanoseconds / (1000 * 1000 * 1000)
}

pub(crate) fn to_days(nanoseconds: u64) -> u64 {
    to_seconds(nanoseconds) / 86400
}

pub(crate) fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
    where P: AsRef<Path>, {
    let file = File::open(filename).expect("File does not exist");
    io::BufReader::new(file).lines()
}
