use itertools::Itertools;
use std::collections::HashSet;

fn parse_input(filename: &str) -> Vec<(i64, i64, i64)> {
    std::fs::read_to_string(filename)
        .expect("can't find file")
        .lines()
        .map(|s| {
            s.split(',')
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn solve() -> (usize, usize) {
    let data = parse_input("input/day18.txt");

    let points: HashSet<(i64, i64, i64)> = HashSet::from_iter(data.into_iter());
    let neighbors: Vec<(i64, i64, i64)> = vec![(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)];

    // Count faces with non-lava neighbors.
    let part1 = points
        .iter()
        .cartesian_product(&neighbors)
        .filter(|((x, y, z), (dx, dy, dz))| !points.contains(&(x + dx, y + dy, z + dz)))
        .count();

    let upperbound = points
        .iter()
        .map(|(x, y, z)| x.max(y).max(z))
        .max()
        .unwrap()
        + 2;
    let mut visited: HashSet<(i64, i64, i64)> = HashSet::new();
    let mut frontier: Vec<(i64, i64, i64)> = vec![(0, 0, 0)];
    let mut count = 0;

    // Flood-fill bounding box and count faces that touch lava.
    while let Some((x, y, z)) = frontier.pop() {
        visited.insert((x, y, z));
        for (dx, dy, dz) in neighbors.iter() {
            if x + dx >= upperbound
                || y + dy >= upperbound
                || z + dz >= upperbound
                || x + dx < -1
                || y + dy < -1
                || z + dz < -1
            {
                continue;
            } else if points.contains(&(x + dx, y + dy, z + dz)) {
                count += 1;
            } else if !visited.contains(&(x + dx, y + dy, z + dz))
                && !frontier.contains(&(x + dx, y + dy, z + dz))
            {
                frontier.push((x + dx, y + dy, z + dz));
            }
        }
    }
    (part1, count)
}

aoc2022::aoc!(solve);