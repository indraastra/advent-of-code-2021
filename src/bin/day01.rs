use aoc21::load_data;
use itertools::izip;

fn increases(measurements: &[u32]) -> usize {
    // Version 1:
    // let mut prev = None;
    // let mut count = 0;
    // for depth in measurements {
    //     if let Some(prev_depth) = prev {
    //         count += if depth > prev_depth { 1 } else { 0 };
    //     }
    //     prev = Some(depth)
    // }
    // count
    izip!(measurements, &measurements[1..]).filter(|(p, n)| n > p).count()

}

fn increasing_windows(measurements: &[u32]) -> usize {
    // How can this be passed directly to increases() without the collect()?
    let windows: Vec<u32> = izip!(measurements, &measurements[1..], &measurements[2..])
        .map(|(x, y, z)| x + y + z)
        .collect();
    increases(&windows)
}

fn main() {
    let contents = load_data(1);

    let measurements: Vec<u32> = contents
        .lines()
        .map(|line| {
            line.trim()
                .parse()
                .unwrap_or_else(|_| panic!("Failed to parse '{}' as a number", line))
        })
        .collect();

    println!(
        "[p1] Number of depth increases: {}",
        increases(&measurements)
    );
    println!(
        "[p2] Number of depth increases with windows: {}",
        increasing_windows(&measurements)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increases() {
        assert_eq!(increases(&[1, 2, 3]), 2);
        assert_eq!(increases(&[2, 2, 2]), 0);
        assert_eq!(increases(&[3, 2, 1]), 0);
        assert_eq!(increases(&[0, 2, 2, 1, 2, 2, 4]), 3);
        assert_eq!(increases(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]), 7);
    }

    #[test]
    fn test_increasing_windows() {
        assert_eq!(increasing_windows(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]), 5);
    }
}

