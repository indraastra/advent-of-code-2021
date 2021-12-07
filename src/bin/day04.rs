use std::fmt;
use std::{error::Error, num::ParseIntError, str::FromStr, u32};

use aoc21::{all_true, any_true, load_data, parse_numbers_csv};

const BOARD_DIM: usize = 5;
const BOARD_SIZE: usize = BOARD_DIM * BOARD_DIM;

struct Board {
    grid: [u32; BOARD_SIZE],
    marked: [bool; BOARD_SIZE],
}

fn parse_board(numbers: &str) -> Result<Board, String> {
    let grid = numbers
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u32>().map_err(|e| e.to_string()))
        .collect::<Result<Vec<u32>, _>>()?;
    if grid.len() != BOARD_SIZE {
        Err(format!(
            "Grid was not the right size!: actual {} vs. expected {}",
            grid.len(),
            BOARD_SIZE
        ))
    } else {
        Ok(Board {
            grid: grid.try_into().unwrap(),
            marked: [false; BOARD_SIZE],
        })
    }
}

fn parse_numbers(numbers: &str) -> Result<Vec<u32>, String> {
    parse_numbers_csv::<u32>(numbers).map_err(|e| e.to_string())
}

fn parse_boards<'a>(boards: impl IntoIterator<Item = &'a str>) -> Result<Vec<Board>, String> {
    boards.into_iter().map(|ns| parse_board(ns)).collect()
}

fn mark_number(board: &mut Board, n: u32) {
    for (idx, &val) in board.grid.iter().enumerate() {
        if val == n {
            board.marked[idx] = true;
        }
    }
}

fn is_winning(board: &Board) -> bool {
    // Check for a winning row.
    (board.marked.windows(BOARD_SIZE).map(all_true).any(|x| x) ||
    // Check for a winning column.
     false)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_data(4);
    let mut bingo_parts = input.split("\n\n");

    let numbers: Vec<u32> = bingo_parts
        .next()
        .ok_or("Failed to read numbers from input".to_string())
        .and_then(parse_numbers)?;
    let mut boards = parse_boards(bingo_parts)?;
    println!(
        "Read {} numbers, {} boards from input.",
        numbers.len(),
        boards.len()
    );

    for number in numbers {
        for mut board in &boards {
            mark_number(&mut board, number);
        }
    }

    Ok(())
}
