use itertools::Itertools;
use std::collections::HashSet;

fn parse_input(filename: &str) -> Vec<Vec<char>> {
    std::fs::read_to_string(filename)
        .expect("can't find file")
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn part1(data: &Vec<Vec<char>>) -> u64 {
    let mut priorities = 0;
    for sack in data.iter() {
        let (sack1, sack2) = sack.split_at(sack.len() / 2);
        let set1: HashSet<&char> = HashSet::from_iter(sack1.iter());
        let set2: HashSet<&char> = HashSet::from_iter(sack2.iter());
        let intersection: char = **set1.intersection(&set2).next().unwrap();
        priorities += match intersection.is_uppercase() {
            true => intersection as u64 - 65 + 27,
            false => intersection as u64 - 96,
        }
    }
    priorities
}
fn part2(data: &Vec<Vec<char>>) -> u64 {
    let mut three_iter = data.iter().tuples();
    let mut ans = 0;
    while let Some((a, b, c)) = three_iter.next() {
        let set1: HashSet<&char> = HashSet::from_iter(a.iter());
        let set2: HashSet<&char> = HashSet::from_iter(b.iter());
        let set3: HashSet<&char> = HashSet::from_iter(c.iter());
        let intersection: HashSet<&char> = set1.intersection(&set2).map(|x| *x).collect();
        let intersection: char = **intersection.intersection(&set3).next().unwrap();
        ans += match intersection.is_uppercase() {
            true => intersection as u64 - 65 + 27,
            false => intersection as u64 - 96,
        }
    }
    ans
}
fn solve() -> (u64, u64) {
    let data = parse_input("input/day03.txt");

    (part1(&data), part2(&data))
}

aoc2022::aoc!(solve);
