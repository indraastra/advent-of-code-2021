use std::fs;
use std::str::FromStr;

pub fn load_data(day: u32) -> String {
    let filename = format!("data/day{:02}.txt", day);
    println!("Reading file {}", filename);
    fs::read_to_string(filename).expect("Error reading file")
}

pub fn parse_numbers<T: FromStr>(line: &str, delim: char) -> Result<Vec<T>, T::Err> {
    line.trim().split(delim).map(|n| n.parse()).collect()
}
pub fn parse_numbers_csv<T: FromStr>(line: &str) -> Result<Vec<T>, T::Err> {
    parse_numbers(line, ',')
}
