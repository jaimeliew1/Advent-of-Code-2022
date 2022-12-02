use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref BEATS: HashMap<char, char> = vec![('A', 'C'), ('B', 'A'), ('C', 'B')]
        .into_iter()
        .collect();
    static ref BEATENBY: HashMap<char, char> = vec![('A', 'B'), ('B', 'C'), ('C', 'A')]
        .into_iter()
        .collect();
    static ref SCORE: HashMap<char, u64> = vec![('A', 1), ('B', 2), ('C', 3)].into_iter().collect();
}

fn parse_input(filename: &str) -> Vec<(char, char)> {
    let contents = std::fs::read_to_string(filename).expect("can't find file");
    contents
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .map(|chars| {
            (
                chars[0],
                match chars[2] {
                    'X' => 'A',
                    'Y' => 'B',
                    'Z' => 'C',
                    _ => panic!(),
                },
            )
        })
        .collect()
}

fn solve() -> (u64, u64) {
    let data = parse_input("input/day02.txt");
    let part1: u64 = data
        .iter()
        .map(|(left, right)| match left {
            x if BEATS[x] == *right => 0 + SCORE[right],
            x if BEATS[right] == *x => 6 + SCORE[right],
            x if x == right => 3 + SCORE[right],
            _ => panic!(),
        })
        .sum();
    let part2: u64 = data
        .iter()
        .map(|(left, right)| match right {
            'A' => 0 + SCORE[&BEATS[left]],
            'B' => 3 + SCORE[left],
            'C' => 6 + SCORE[&BEATENBY[left]],
            _ => panic!(),
        })
        .sum();
    (part1, part2)
}

aoc2022::aoc!(solve);
