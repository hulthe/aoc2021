use hashers::fx_hash::FxHasher;
use std::hash::BuildHasherDefault;
type HashMap<K, V> = std::collections::HashMap<K, V, BuildHasherDefault<FxHasher>>;

type Height = u8;
type Map<T> = Vec<Vec<T>>;
type Coord = (usize, usize);

pub fn parse(input: &str) -> Map<Height> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|c| c.to_digit(10))
                .map(|d| d as Height)
                .collect()
        })
        .collect()
}

fn neighbors_pos(x: usize, y: usize, map: &Map<Height>) -> impl Iterator<Item = Coord> {
    let mx = map[0].len();
    let my = map.len();

    [
        (x.wrapping_sub(1), y),
        (x + 1, y),
        (x, y.wrapping_sub(1)),
        (x, y + 1),
    ]
    .into_iter()
    .filter(move |&(x, _)| x < mx)
    .filter(move |&(_, y)| y < my)
}

fn neighbors<'a>(x: usize, y: usize, map: &'a Map<Height>) -> impl Iterator<Item = Height> + 'a {
    neighbors_pos(x, y, map).map(|(x, y)| map[y][x])
}

fn all_coords(map: &Map<Height>) -> impl Iterator<Item = Coord> {
    let mx = map[0].len();
    let my = map.len();

    (0..my).flat_map(move |y| (0..mx).map(move |x| (x, y)))
}

pub fn part1(input: &str) -> u64 {
    let map = parse(input);

    all_coords(&map)
        .filter_map(|(x, y)| {
            let point = map[y][x];
            neighbors(x, y, &map)
                .all(|neighbor| neighbor > point)
                .then(|| point)
        })
        .map(|point| (point + 1) as u64)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    #[derive(Clone, Copy)]
    enum Flow {
        LowPoint,
        Into(Coord),
        NotABasin,
    }

    let map = parse(input);
    let mx = map[0].len();
    let my = map.len();

    let mut flow_map: Map<Option<Flow>> = vec![vec![None; mx]; my];
    let low_points: Vec<Coord> = all_coords(&map)
        .filter(|&(x, y)| {
            let point = map[y][x];
            neighbors(x, y, &map).all(|neighbor| neighbor > point)
        })
        .collect();

    fn determine_flow(
        (x, y): Coord,
        map: &Map<Height>,
        flow_map: &mut Map<Option<Flow>>,
        low_points: &Vec<Coord>,
    ) -> Flow {
        if let Some(flow) = flow_map[y][x] {
            return flow;
        }

        // I'll have a #9 large, extra dip.
        if map[y][x] == 9 {
            flow_map[y][x] = Some(Flow::NotABasin);
            return Flow::NotABasin;
        }

        let point = map[y][x];
        let lowest_neighbor = neighbors_pos(x, y, map)
            .filter(|&(x, y)| map[y][x] < point)
            .min_by_key(|&(x, y)| map[y][x]);

        let flow = match lowest_neighbor {
            Some(neighbor) => match determine_flow(neighbor, map, flow_map, low_points) {
                Flow::Into(lowest) => Flow::Into(lowest),
                Flow::LowPoint => Flow::Into(neighbor),
                Flow::NotABasin => Flow::LowPoint,
            },
            None => Flow::LowPoint,
        };

        flow_map[y][x] = Some(flow);
        flow
    }

    let mut basins: HashMap<Coord, u64> = HashMap::default();
    for coord in all_coords(&map) {
        let flow = determine_flow(coord, &map, &mut flow_map, &low_points);

        let lowest = match flow {
            Flow::LowPoint => coord,
            Flow::Into(lowest) => lowest,
            Flow::NotABasin => continue,
        };
        *basins.entry(lowest).or_default() += 1;
    }

    let mut basins: Vec<u64> = basins.values().copied().collect();
    basins.sort_unstable_by(|a, b| b.cmp(a));
    basins[0..3].iter().copied().product()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 15);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 1134);
    }
}
