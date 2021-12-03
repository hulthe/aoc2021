pub fn parse(input: &str) -> (Vec<u16>, usize) {
    let bits = input.lines().next().expect("input empty?").len();
    let data = input.lines()
        .map(|line| u16::from_str_radix(line, 2).expect("failed to parse binary number"))
        .collect();

    (data, bits)
}

fn check_bit(number: u16, bit: usize) -> bool {
    let mask = 1 << bit;
    (number & mask) != 0
}

pub fn part1(input: &str) -> u32 {
    let (data, bits) = parse(input);
    let mut bit_count = vec![0i32; bits];
    for num in data {
        for i in 0..bit_count.len() {
            let bit_set = check_bit(num ,i);
            // branchless baby
            bit_count[i] += (bit_set as i32) * 2 - 1;
        }
    }

    let mut gamma = 0u32;
    for bit in bit_count.into_iter().rev() {
        gamma <<= 1;
        if bit >= 0 {
            gamma |= 1;
        }
    }

    let mut mask = 0;
    for _ in 0..bits {
        mask <<= 1;
        mask |= 1;
    }

    let epsilon = gamma ^ mask;

    gamma * epsilon
}

pub fn part2(input: &str) -> u32 {
    let (data, bits) = parse(input);

    fn decode(mut data: Vec<u16>, filter: impl Fn(usize, usize) -> bool, bits: usize) -> u32 {
        for bit in (0..bits).rev() {
            let ones: usize = data.iter().map(|&line| check_bit(line, bit) as usize).sum();
            let zeros = data.len() - ones;

            let mask = 1 << bit;

            let cmp;
            if filter(ones, zeros) {
                cmp = 1 << bit;
            } else {
                cmp = 0;
            }

            data.retain(|line| line & mask == cmp);

            if data.len() <= 1 {
                break;
            }
        }

        assert_eq!(data.len(), 1, "failed not find a single value in the data");

        data[0] as u32
    }

    let oxygen = decode(data.clone(), |ones, zeros| ones >= zeros, bits);
    let scrubber = decode(data, |ones, zeros| ones < zeros, bits);
    oxygen * scrubber
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 198);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 230);
    }
}
