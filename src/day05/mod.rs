use std::cmp::max;

type Pos = (usize, usize);

#[derive(Debug, Clone, Copy)]
pub struct Line {
    start: Pos,
    end: Pos,
}

pub fn parse(input: &str) -> Vec<Line> {
    fn parse_pos(s: &str) -> Pos {
        let (x, y) = s.split_once(',').unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    }
    input.lines()
        .map(|line| line.split_once(" -> ").expect("invalid input"))
        .map(|(start, end)| [start, end].map(parse_pos))
        .map(|[start, end]| Line { start, end })
        .collect()
}

fn count_overlaps(lines: &[Line]) -> usize {
    let mut points = [[0u16; 1000]; 1000];
    for line in lines {
        for p in line.points() {
            points[p.0][p.1] += 1;
        }
    }

    points.into_iter().flat_map(|col| col.into_iter()).filter(|&count| count > 1).count()
}

pub fn part1(input: &str) -> usize {
    let mut lines = parse(input);

    lines.retain(|line| line.start.0 == line.end.0 || line.start.1 == line.end.1);

    count_overlaps(&lines)
}

pub fn part2(input: &str) -> usize {
    let lines = parse(input);
    count_overlaps(&lines)
}

impl Line {
    fn points(&self) -> impl Iterator<Item=Pos> {
        LineIter::new(self.start, self.end)
    }
}

/// An iterator over all the points in a line
struct LineIter {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
    steps: usize,
}

impl LineIter {
    pub fn new(start: Pos, end: Pos) -> Self {
        let dir = |s, e: usize| e.cmp(&s) as isize;

        let (x1, y1) = (start.0 as isize, start.1 as isize);
        let (x2, y2) = (end.0 as isize, end.1 as isize);
        let diff_x = (x1 - x2).abs();
        let diff_y = (y1 - y2).abs();
        Self {
            x: x1,
            y: y1,
            dx: dir(start.0, end.0),
            dy: dir(start.1, end.1),
            steps: max(diff_x, diff_y) as usize + 1,
        }
    }
}

impl Iterator for LineIter {
    type Item = Pos;
    fn next(&mut self) -> Option<Self::Item> {
        if self.steps == 0 {
            None
        } else {
            self.steps -= 1;

            let p = (self.x as usize, self.y as usize);
            self.x += self.dx;
            self.y += self.dy;
            Some(p)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 5);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 12);
    }
}
