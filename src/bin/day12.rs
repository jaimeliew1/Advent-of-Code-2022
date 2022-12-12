use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn parse_input(filename: &str) -> Vec<Vec<char>> {
    std::fs::read_to_string(filename)
        .expect("can't find file")
        .lines()
        .map(|s| s.chars().collect())
        .collect()
}

fn dijkstra(map: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> Option<u64> {
    let mut frontier: HashMap<(usize, usize), usize> = HashMap::from([(start, 0)]);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let (mut x, mut y) = start;

    while !visited.contains(&end) {
        for (xnew, ynew) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            match map.get(ynew).and_then(|row| row.get(xnew)) {
                // Ignore already visited coordinates.
                Some(_) if visited.contains(&(xnew, ynew)) => continue,
                // Ignore out-of-bound coordinates.
                None => continue,
                // Ignore too steep and uppercase destinations.
                Some(h)
                    if h.is_ascii_lowercase()
                        && map[y][x].is_ascii_lowercase()
                        && *h as u64 > map[y][x] as u64 + 1 => continue,
                // If valid, add to frontier and record min steps to destination.
                Some(_) => {
                    let this_distance = frontier[&(x, y)];
                    frontier
                        .entry((xnew, ynew))
                        .and_modify(|e| *e = *e.min(&mut (this_distance + 1)))
                        .or_insert(this_distance + 1);
                }
            }
        }
        if (x, y) == end {
            return Some(frontier[&(x, y)] as u64);
        }
        frontier.remove(&(x, y));
        visited.insert((x, y));
        // Evaluate next best candidate on the frontier.
        if let Some((&(_x, _y), _)) =  frontier.iter().min_by_key(|(_, v)| **v) {
            x = _x;
            y = _y;
        }
        else {return None}
        
    }
    unreachable!()
}
fn solve() -> (u64, u64) {
    let data = parse_input("input/day12.txt");
    let part1 = dijkstra(&data, (0, 20), (139, 20)).unwrap();

    let a_list: Vec<(usize, usize)> = (0..data[0].len())
        .cartesian_product(0..data.len())
        .filter(|&(x, y)| data[y][x] == 'a')
        .collect();
    let part2 = a_list
        .iter()
        .filter_map(|&start| dijkstra(&data, start, (139, 20)))
        .min()
        .unwrap();
    (part1, part2)
}

aoc2022::aoc!(solve);
