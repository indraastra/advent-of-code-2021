use std::fs;

pub fn load_data(day: u32) -> String {
    let filename = format!("data/day{:02}.txt", day);
    println!("Reading file {}", filename);
    fs::read_to_string(filename).expect("Error reading file")
}
