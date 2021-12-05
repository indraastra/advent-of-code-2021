use std::{num::ParseIntError, str::FromStr};

use aoc21::{load_data, parse_numbers_csv};

const BOARD_DIM: u32 = 5;

struct Board;

impl FromStr for Board {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<u32> = s.trim().split_whitespace().map(|n| n.parse()).collect();
        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point { x: x_fromstr, y: y_fromstr })
    }


}

fn parse_boards<'a>(boards: impl IntoIterator<Item = &'a str>) -> Vec<Board> {
    let mut boards: Vec<Board> = vec![];
    boards
}

fn main() {
    let mut bingo = load_data(4).split("\n\n");
    let numbers = bingo
        .next()
        .ok_or("Failed to find line with numbers")
        .and_then(parse_numbers_csv)
        .unwrap();
    let boards = parse_boards(bingo);
}
