#![feature(iter_zip)]

use std::{error::Error, iter::zip, u32};

use aoc21::{all_true, load_data, parse_numbers_csv};

const BOARD_DIM: usize = 5;
const BOARD_SIZE: usize = BOARD_DIM * BOARD_DIM;

#[derive(Debug)]
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

fn any_are_all<'a, BoolIter, BoolIterIter>(bool_chunks: BoolIterIter) -> bool
where
    BoolIter: IntoIterator<Item = &'a bool>,
    BoolIterIter: IntoIterator<Item = BoolIter>,
{
    bool_chunks.into_iter().map(all_true).any(|x| x)
}

fn is_winning(board: &Board) -> bool {
    // Check for a winning row.
    any_are_all(board.marked.chunks(BOARD_DIM)) ||
    // Check for a winning column.
    any_are_all((0..BOARD_DIM).map(|s| board.marked.iter().skip(s).step_by(BOARD_DIM)))
}

fn unmarked_sum(board: &Board) -> u32 {
    zip(board.marked, board.grid)
        .filter_map(|(marked, n)| if marked { None } else { Some(n) })
        .sum()
}

fn winning_score(board: &Board, last_n: u32) -> u32 {
    unmarked_sum(board) * last_n
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

    let mut first_score = None;
    let mut last_score = None;

    for number in numbers {
        for board in boards.iter_mut() {
            mark_number(board, number);
        }
        boards.retain(|board| {
            let winning = is_winning(board);
            if winning {
                match first_score {
                    None => first_score = Some(winning_score(board, number)),
                    Some(_) => last_score = Some(winning_score(board, number)),
                }
            }
            !winning
        });
        if boards.len() == 0 {
            break; 
        }
    }

    println!("[p1] First winning score: {}", first_score.unwrap());
    println!("[p2] Last winning score: {}", last_score.unwrap());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numbers() {
        assert_eq!(
            parse_numbers("7,4,9,5,11,17,23,2"),
            Ok(vec![7, 4, 9, 5, 11, 17, 23, 2])
        );
    }

    fn test_boards() -> Vec<&'static str> {
        vec![
            "22 13 17 11  0
              8  2 23  4 24
             21  9 14 16  7
              6 10  3 18  5
              1 12 20 15 19",
            " 3 15  0  2 22
              9 18 13 17  5
             19  8  7 25 23
             20 11 10 24  4
             14 21 16 12  6",
            "14 21 17 24  4
             10 16 15  9 19
             18  8 23 26 20
             22 11 13  6  5
              2  0 12  3  7",
        ]
    }
    #[test]
    fn test_parse_boards() -> Result<(), String> {
        let boards = parse_boards(test_boards())?;
        assert_eq!(boards.len(), 3);
        Ok(())
    }

    #[test]
    fn test_winning_row() -> Result<(), String> {
        let mut board = parse_board(
            "14 21 17 24  4
             10 16 15  9 19
             18  8 23 26 20
             22 11 13  6  5
              2  0 12  3  7",
        )?;
        for n in vec![14, 21, 17, 24, 4] {
            mark_number(&mut board, n);
        }
        assert_eq!(is_winning(&board), true);
        Ok(())
    }

    #[test]
    fn test_winning_col() -> Result<(), String> {
        let mut board = parse_board(
            "14 21 17 24  4
             10 16 15  9 19
             18  8 23 26 20
             22 11 13  6  5
              2  0 12  3  7",
        )?;
        for n in vec![14, 10, 18, 22, 2] {
            mark_number(&mut board, n);
        }
        assert_eq!(is_winning(&board), true);
        Ok(())
    }

    #[test]
    fn test_winning_score() -> Result<(), String> {
        let mut board = parse_board(
            "14 21 17 24  4
             10 16 15  9 19
             18  8 23 26 20
             22 11 13  6  5
              2  0 12  3  7",
        )?;
        let numbers = parse_numbers("7,4,9,5,11,17,23,2,0,14,21,24")?;
        let last_n = numbers.last().unwrap();
        for n in &numbers {
            mark_number(&mut board, *n);
        }

        assert_eq!(is_winning(&board), true);
        assert_eq!(unmarked_sum(&board), 188);
        assert_eq!(winning_score(&board, *last_n), 4512);
        Ok(())
    }
}
