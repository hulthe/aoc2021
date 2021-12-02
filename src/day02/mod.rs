pub fn parse<'a>(input: &'a str) -> impl Iterator<Item=(i32, i32)> + 'a {
    input.lines()
        .flat_map(|line| line.split_once(' '))
        .map(|(dir, dist)| (dir, dist.parse::<i32>().expect("failed to parse integer")))
        .map(|(dir, dist)| match dir {
            "forward" => (dist, 0),
            "down" => (0, dist),
            "up" => (0, -dist),
            _ => panic!("invalid input"),
        })
}

pub fn part1(input: &str) -> i32 {
    let (pos, depth) = parse(input).fold((0, 0), |(pos, depth), (x, y)| (pos + x, depth + y));

    pos * depth
}

pub fn part2(input: &str) -> i32 {
    let (pos, depth, _aim) = parse(input)
        .fold((0, 0, 0), |(pos, depth, aim), (x, r)| {
            let aim = aim + r;
            (pos + x, depth + x * aim, aim)
        });

    pos * depth
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 150);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 900);
    }
}
