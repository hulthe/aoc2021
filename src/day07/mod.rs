use std::cmp::min;

/// Parse to a sorted list numbers
pub fn parse(input: &str) -> Vec<i64> {
    let mut crabs: Vec<_> = input
        .trim()
        .split(',')
        .map(|n| n.parse().expect("invalid crab"))
        .collect();
    crabs.sort_unstable();
    crabs
}

pub fn part1(input: &str) -> i64 {
    let crabs = parse(input);

    let min_crab = crabs.iter().copied().min().unwrap();
    let max_crab = crabs.iter().copied().max().unwrap();
    let mid_crab = (min_crab + max_crab) / 2;

    // applying the fuel function over the span of crabs yields a polynomial
    // let's solve it
    let (x1, y1) = (min_crab as f64, fuel_to_move_to(&crabs, min_crab) as f64);
    let (x2, y2) = (mid_crab as f64, fuel_to_move_to(&crabs, mid_crab) as f64);
    let (x3, y3) = (max_crab as f64, fuel_to_move_to(&crabs, max_crab) as f64);
    let a =
        (x1 * (y3 - y2) + x2 * (y1 - y3) + x3 * (y2 - y1)) / ((x1 - x2) * (x1 - x3) * (x2 - x3));
    let b = (y2 - y1) / (x2 - x1) - a * (x1 + x2);
    let function_min = (-b / 2.0 / a) as i64;

    fn fuel_to_move_to(crabs: &[i64], to: i64) -> i64 {
        crabs.iter().copied().map(|from| (from - to).abs()).sum()
    }

    /// take elements from the iterator while they are decreasing, then return the smallest
    fn scan(i: impl Iterator<Item = i64>, crabs: &[i64]) -> i64 {
        let mut i = i.map(|pos| fuel_to_move_to(&crabs, pos));
        let mut last = i.next().unwrap();

        for fuel in i {
            if fuel < last {
                last = fuel;
            } else {
                break;
            }
        }

        last
    }

    // the polynomial is slightly inaccurate, so we look at the nearby values to find the smallest
    let left = (1..).map(|i| function_min - i);
    let right = function_min..;
    min(scan(left, &crabs), scan(right, &crabs))
}

pub fn part2(input: &str) -> i64 {
    let crabs = parse(input);

    let crab_average = crabs.iter().copied().sum::<i64>() / crabs.len() as i64;

    fn fuel_to_move_to(crabs: &[i64], to: i64) -> i64 {
        crabs
            .iter()
            .copied()
            .map(|from| (from - to).abs())
            .map(|steps| (0..=steps).sum::<i64>())
            .sum()
    }

    // account for rounding errors
    [crab_average, crab_average + 1]
        .map(|pos| fuel_to_move_to(&crabs, pos))
        .into_iter()
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 37);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 168);
    }
}
