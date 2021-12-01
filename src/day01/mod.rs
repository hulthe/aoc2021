pub fn parse(input: &str) -> Vec<i32> {
    input.lines()
        .map(|line| line.parse().expect("invalid input"))
        .collect()
}

pub fn part1(input: &str) -> usize {
    let measurements = parse(input);
    measurements
        .windows(2)
        .filter(|window| window.is_sorted())
        .count()
}

pub fn part2(input: &str) -> usize {
    let measurements = parse(input);
    let windows = measurements
        .windows(3)
        .map(|win| win.iter().sum::<i32>());

    windows.clone().zip(windows.skip(1))
        .filter(|(prev, next)| next > prev)
        .count()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 7);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 5);
    }
}
