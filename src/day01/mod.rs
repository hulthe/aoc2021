pub fn parse(input: &str) -> Vec<i32> {
    input.lines()
        .map(|line| line.parse().expect("invalid input"))
        .collect()
}

fn solver<const W: usize>(input: &str) -> usize {
    parse(input)
        .windows(W)
        .filter(|win| win[0] < win[W - 1])
        .count()
}

pub fn part1(input: &str) -> usize {
    solver::<2>(input)
}

pub fn part2(input: &str) -> usize {
    solver::<4>(input)
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
