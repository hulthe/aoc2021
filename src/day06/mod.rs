type FishCount = u64;
type School = [FishCount; 9];

pub fn parse(input: &str) -> School {
    let mut school = [0; 9];

    input.trim()
        .split(',')
        .map(|timer| timer.parse().unwrap())
        .for_each(|timer: usize| school[timer] += 1);

    school
}

fn simulate<const DAYS: usize>(mut fish: School) -> u64 {
    for _day in 0..DAYS {
        fish.rotate_left(1);
        fish[6] += fish[8]; // yesss... breed my pretties!
    }

    fish.into_iter().sum()
}

pub fn part1(input: &str) -> u64 {
    let fish = parse(input);
    simulate::<80>(fish)
}

pub fn part2(input: &str) -> u64 {
    let fish = parse(input);
    simulate::<256>(fish)
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 5934);
    }
}
