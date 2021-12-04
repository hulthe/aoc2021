use std::collections::{HashMap, HashSet};

type Number = u32;
type Pos = (usize, usize);

pub struct Bingo {
    pub numbers: Vec<Number>,
    pub boards: Vec<Board>,
}

#[derive(Clone)]
pub struct Board {
    pub numbers: HashMap<Number, Pos>,
}

pub fn parse(input: &str) -> Bingo {
    let mut lines = input.split("\n\n");

    let numbers = lines.next().expect("input empty");
    let numbers = numbers
        .split(',')
        .map(|n| n.parse().expect("invalid input"))
        .collect();

    let boards = lines
        .map(|numbers| {
            numbers
                .split_whitespace()
                .enumerate()
                .map(|(i, n)| (i, n.parse().unwrap()))
                .map(|(i, n)| (n, (i % 5, i / 5)))
                .collect()
        })
        .inspect(|numbers: &HashMap<_, _>| debug_assert_eq!(numbers.len(), 25))
        .map(|numbers| Board { numbers })
        .collect();

    Bingo { numbers, boards }
}

fn row((x, y): Pos) -> impl Iterator<Item = Pos> {
    (0..5).filter(move |&rx| rx != x).map(move |rx| (rx, y))
}

fn col((x, y): Pos) -> impl Iterator<Item = Pos> {
    (0..5).filter(move |&cy| cy != y).map(move |cy| (x, cy))
}

fn check_if_board_won(placed: &HashSet<Pos>, check: &[Pos]) -> bool {
    check
        .iter()
        .any(|&p| row(p).all(|p| placed.contains(&p)) || col(p).all(|p| placed.contains(&p)))
}

fn solver(
    bingo: Bingo,
    mut return_condition: impl FnMut(usize) -> bool,
) -> (Board, HashSet<Pos>, Number) {
    let mut placed: Vec<HashSet<Pos>> = vec![HashSet::new(); bingo.boards.len()];
    let mut boards = bingo.boards;

    for &num in &bingo.numbers {
        let mut won = HashSet::new();

        for i in 0..boards.len() {
            if let Some(&pos) = boards[i].numbers.get(&num) {
                placed[i].insert(pos);

                if check_if_board_won(&placed[i], &[pos]) {
                    if return_condition(placed.len()) {
                        return (boards.remove(i), placed.remove(i), num);
                    }

                    won.insert(i);
                }
            }
        }

        if !won.is_empty() {
            fn drain_i<T, I: Iterator<Item = T>, F: Fn(usize) -> bool>(i: I, f: F) -> Vec<T> {
                i.enumerate()
                    .filter(|&(i, _)| f(i))
                    .map(|(_, t)| t)
                    .collect()
            }
            placed = drain_i(placed.into_iter(), |i| !won.contains(&i));
            boards = drain_i(boards.into_iter(), |i| !won.contains(&i));

            won.clear();
        }
    }

    panic!("no board won :(");
}

fn score_board(board: &Board, placed: &HashSet<Pos>, final_num: Number) -> u32 {
    let unmarked_sum: u32 = board
        .numbers
        .iter()
        .filter(|(_, pos)| !placed.contains(pos))
        .map(|(&num, _)| num)
        .sum();

    unmarked_sum * final_num
}

pub fn part1(input: &str) -> u32 {
    let bingo = parse(input);
    let (board, placed, final_num) = solver(bingo, |_| true);
    score_board(&board, &placed, final_num)
}

pub fn part2(input: &str) -> u32 {
    let bingo = parse(input);
    let (board, placed, final_num) = solver(bingo, |l| l == 1);
    score_board(&board, &placed, final_num)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 4512);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 1924);
    }
}
