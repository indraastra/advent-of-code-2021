use aoc21::load_data;

fn bits_to_u32(bits: &str) -> u32 {
    u32::from_str_radix(bits, 2).unwrap()
}

fn nth_char(s: &str, idx: usize) -> char {
    s.as_bytes()[idx] as char
}

fn power_consumption(numbers: &Vec<&str>) -> u32 {
    let nbits = numbers[0].len();
    println!("Numbers: {}, Bits: {}", numbers.len(), nbits);

    let mut counts = vec![0; nbits];
    for bits in numbers {
        for (i, b) in bits.chars().enumerate() {
            counts[i] += b.to_digit(2).unwrap();
        }
    }
    println!("Counts: {:?}", counts);

    let mid = (numbers.len() / 2) as u32;
    let gamma: String = counts
        .iter()
        .map(|&x| if x > mid { '1' } else { '0' })
        .collect();
    let epsilon: String = counts
        .iter()
        .map(|&x| if x > mid { '0' } else { '1' })
        .collect();

    let gamma = bits_to_u32(&gamma);
    let epsilon = bits_to_u32(&epsilon);
    println!("Gamma: {}; Epsilon: {}", gamma, epsilon);
    gamma * epsilon
}

fn winners_at<'a>(
    candidates: &Vec<&'a str>,
    win_fn: fn(u32, u32) -> char,
    idx: usize,
) -> Vec<&'a str> {
    let mut counts = (0, 0);
    for &c in candidates {
        match nth_char(c, idx) {
            '0' => counts.0 += 1,
            '1' => counts.1 += 1,
            _ => unreachable!(),
        }
    }
    let winner = win_fn(counts.0, counts.1);
    candidates
        .iter()
        .filter(|c| nth_char(c, idx) == winner)
        .copied()
        .collect()
}

fn winner<'a>(candidates: &Vec<&'a str>, win_fn: fn(u32, u32) -> char) -> &'a str {
    let mut winners = candidates.to_owned();
    for i in 0..candidates.len() {
        winners = winners_at(&winners, win_fn, i);
        if winners.len() == 1 {
            break;
        }
    }
    winners.first().unwrap()
}

fn o2_rating(candidates: &Vec<&str>) -> u32 {
    let rating = winner(candidates, |zeros, ones| {
        // Winner wins, 1 to tie-break.
        if zeros > ones {
            '0'
        } else if zeros < ones {
            '1'
        } else {
            '1'
        }
    });
    bits_to_u32(rating)
}

fn co2_rating(numbers: &Vec<&str>) -> u32 {
    let rating = winner(numbers, |zeros, ones| {
        // Loser wins, 0 to tie-break.
        if zeros > ones {
            '1'
        } else if zeros < ones {
            '0'
        } else {
            '0'
        }
    });
    bits_to_u32(rating)
}

fn main() {
    let report: String = load_data(3);
    let numbers: Vec<&str> = report.lines().collect();
    println!("[p1] power consumption: {}", power_consumption(&numbers));
    println!(
        "[p2] rating: {}",
        o2_rating(&numbers) * co2_rating(&numbers)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<&'static str> {
        vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
    }

    #[test]
    fn test_o2_rating() {
        assert_eq!(o2_rating(&test_data()), 23);
    }

    #[test]
    fn test_co2_rating() {
        assert_eq!(co2_rating(&test_data()), 10);
    }
}
