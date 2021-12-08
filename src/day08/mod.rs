use std::collections::HashMap;

pub struct Entry {
    patterns: [Seg; 10],
    output: [Seg; 4],
}

type Seg = [bool; 7];

#[repr(usize)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum Signal {
    #[default]
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
}
use Signal::*;

impl From<usize> for Signal {
    fn from(i: usize) -> Self {
        [A, B, C, D, E, F, G][i]
    }
}

const LEN_SEG_1: usize = 2;
const LEN_SEG_4: usize = 4;
const LEN_SEG_7: usize = 3;
const LEN_SEG_8: usize = 7;

fn active_segments(seg: &Seg) -> usize {
    seg.iter().filter(|&signal| *signal).count()
}

fn parse_segment(input: &str) -> Seg {
    let mut seg = Seg::default();

    for c in input.chars() {
        let signal = match c {
            'a' => A,
            'b' => B,
            'c' => C,
            'd' => D,
            'e' => E,
            'f' => F,
            'g' => G,
            _ => panic!("unexpected segment char: {}", c),
        };
        seg[signal as usize] = true;
    }

    seg
}

pub fn parse(input: &str) -> Vec<Entry> {
    fn parse_segments<const N: usize>(segs: &str) -> [Seg; N]
    where
        [Seg; N]: Default,
    {
        collect_array(segs.split_whitespace().map(parse_segment))
    }

    input
        .lines()
        .flat_map(|entry| entry.split_once(" | "))
        .map(|(patterns, output)| Entry {
            patterns: parse_segments(patterns),
            output: parse_segments(output),
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let entries = parse(input);
    entries
        .into_iter()
        .flat_map(|entry| entry.output.into_iter())
        .map(|output| active_segments(&output))
        .filter(|num_signals| [LEN_SEG_1, LEN_SEG_7, LEN_SEG_4, LEN_SEG_8].contains(num_signals))
        .count()
}

pub fn part2(input: &str) -> usize {
    let entries = parse(input);

    let seg_numbers = HashMap::from([
        (parse_segment("abcefg"), 0),
        (parse_segment("cf"), 1),
        (parse_segment("acdeg"), 2),
        (parse_segment("acdfg"), 3),
        (parse_segment("bcdf"), 4),
        (parse_segment("abdfg"), 5),
        (parse_segment("abdefg"), 6),
        (parse_segment("acf"), 7),
        (parse_segment("abcdefg"), 8),
        (parse_segment("abcdfg"), 9),
    ]);

    entries
        .into_iter()
        .map(|entry| {
            let seg_xor = |seg1: Seg, seg2: Seg| seg1.zip(seg2).map(|(a, b)| a != b);

            #[track_caller]
            fn sigs<const N: usize>(seg: Seg) -> [Signal; N] {
                let sigs = seg
                    .into_iter()
                    .enumerate()
                    .filter(|&(_, active)| active)
                    .map(|(i, _)| i.into());
                collect_array(sigs)
            }

            fn find_segs_w_len<const N: usize>(entry: &Entry, len: usize) -> [Seg; N] {
                let valid = entry
                    .patterns
                    .iter()
                    .copied()
                    .filter(|entry| active_segments(entry) == len);
                collect_array(valid)
            }

            let [seg_1] = find_segs_w_len(&entry, LEN_SEG_1);
            let [seg_4] = find_segs_w_len(&entry, LEN_SEG_4);
            let [seg_7] = find_segs_w_len(&entry, LEN_SEG_7);
            let [seg_8] = find_segs_w_len(&entry, LEN_SEG_8);
            let segs_069: [Seg; 3] = find_segs_w_len(&entry, 6);

            let [repr_a] = sigs(seg_xor(seg_1, seg_7));

            let repr_cf: [Signal; 2] = sigs(seg_1);
            let (seg_6, repr_c, repr_f) = segs_069
                .into_iter()
                .find_map(|seg| {
                    let contains_c_and_f = repr_cf.into_iter().all(|c_or_f| seg[c_or_f as usize]);

                    if contains_c_and_f {
                        None
                    } else {
                        // find out which one is c and which one is f
                        Some(if seg[repr_cf[0] as usize] {
                            let [repr_f, repr_c] = repr_cf;
                            (seg, repr_c, repr_f)
                        } else {
                            let [repr_c, repr_f] = repr_cf;
                            (seg, repr_c, repr_f)
                        })
                    }
                })
                .expect("failed to determine seg 6");

            let repr_bd: [Signal; 2] = sigs(seg_xor(seg_1, seg_4));
            let segs_09: [Seg; 2] = collect_array(segs_069.into_iter().filter(|&seg| seg != seg_6));

            let (seg_0, repr_d, repr_b) = segs_09
                .into_iter()
                .find_map(|seg| {
                    let contains_b_and_d = repr_bd.into_iter().all(|b_or_d| seg[b_or_d as usize]);

                    if contains_b_and_d {
                        None
                    } else {
                        // find out which one is b and which one is d
                        Some(if seg[repr_bd[0] as usize] {
                            let [repr_b, repr_d] = repr_bd;
                            (seg, repr_d, repr_b)
                        } else {
                            let [repr_d, repr_b] = repr_bd;
                            (seg, repr_d, repr_b)
                        })
                    }
                })
                .expect("failed to determine seg 0");

            let [seg_9] = collect_array(segs_09.into_iter().filter(|&seg| seg != seg_0));

            let [repr_e] = sigs(seg_xor(seg_9, seg_8));

            let mut signal_map: [Signal; 7] = [G; 7];
            signal_map[repr_a as usize] = A;
            signal_map[repr_b as usize] = B;
            signal_map[repr_c as usize] = C;
            signal_map[repr_d as usize] = D;
            signal_map[repr_e as usize] = E;
            signal_map[repr_f as usize] = F;
            //signal_map[repr_g as usize] = G;

            entry
                .output
                .iter()
                .rev()
                .map(|seg| {
                    let mut unjarbled_seg = Seg::default();
                    for (i, &active) in seg.into_iter().enumerate() {
                        if active {
                            let signal = signal_map[i] as usize;
                            debug_assert!(!unjarbled_seg[signal]);
                            unjarbled_seg[signal_map[i] as usize] = true;
                        }
                    }
                    unjarbled_seg
                })
                .map(|seg| seg_numbers[&seg])
                .enumerate()
                .map(|(i, digit)| digit * 10usize.pow(i as u32))
                .sum::<usize>()
        })
        .sum()
}

#[track_caller]
fn collect_array<T: Default + Copy, const N: usize>(mut iter: impl Iterator<Item = T>) -> [T; N] {
    let mut out = [T::default(); N];
    for i in 0..N {
        out[i] = iter
            .next()
            .expect("tried to collect a too short iterator into a too long array");
    }
    assert!(
        iter.next().is_none(),
        "tried to collect a too long iterator into a too short array"
    );
    out
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 26);
    }

    #[test]
    pub fn test_part2() {
        let small =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        assert_eq!(part2(small), 5353);

        let large = include_str!("test-input");
        assert_eq!(part2(large), 61229);
    }
}
