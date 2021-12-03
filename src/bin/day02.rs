use aoc21::load_data;
use itertools::Itertools;

enum Dir {
    Up,
    Down,
    Forward,
}
struct Move(Dir, i32);

fn parse_move(line: &str) -> Move {
    let (dir, amt) = line.trim().split_whitespace().collect_tuple().unwrap();
    let amt: i32 = amt.parse().unwrap();
    let dir = match dir.as_ref() {
        "up" => Dir::Up,
        "down" => Dir::Down,
        "forward" => Dir::Forward,
        _ => panic!("Bad direction: {}", dir),
    };
    Move(dir, amt)
}

fn xy_position(moves: &Vec<Move>) -> i32 {
    let (x, y) = moves.iter().fold((0, 0), |pos, mv| {
        let (x, y) = pos;
        match mv {
            Move(Dir::Up, d) => (x - d, y),
            Move(Dir::Down, d) => (x + d, y),
            Move(Dir::Forward, d) => (x, y + d),
        }
    });
    x * y
}

fn xya_position(moves: &Vec<Move>) -> i32 {
    let (x, y, _) = moves.iter().fold((0, 0, 0), |pos, mv| {
        let (x, y, a) = pos;
        match mv {
            Move(Dir::Up, d) => (x, y, a - d),
            Move(Dir::Down, d) => (x, y, a + d),
            Move(Dir::Forward, d) => (x + d, y + d * a, a),
        }
    });
    x * y
}

fn main() {
    let moves: Vec<Move> = load_data(2).lines().map(parse_move).collect();
    println!("[p1] x * y = {}", xy_position(&moves));
    println!("[p2] x * y = {}", xya_position(&moves));
}

#[cfg(test)]
mod tests {
    use super::Dir::*;
    use super::*;

    fn test_data() -> Vec<Move> {
        vec![
            Move(Forward, 5),
            Move(Down, 5),
            Move(Forward, 8),
            Move(Up, 3),
            Move(Down, 8),
            Move(Forward, 2),
        ]
    }

    #[test]
    fn test_xy_position() {
        assert_eq!(xy_position(&test_data()), 150);
    }
    #[test]
    fn test_xya_position() {
        assert_eq!(xya_position(&test_data()), 900);
    }
}
