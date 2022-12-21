use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

fn parse_input(filename: &str) -> (HashMap<String, i64>, VecDeque<Vec<String>>) {
    let (known, operations): (Vec<Vec<String>>, Vec<Vec<String>>) =
        std::fs::read_to_string(filename)
            .expect("can't find file")
            .lines()
            .map(|s| {
                s.split(&[' ', ':'][..])
                    .filter(|x| x.len() > 0)
                    .map(|x| x.to_string())
                    .collect()
            })
            .partition(|v: &Vec<String>| v.len() == 2);

    let known: HashMap<String, i64> = known
        .into_iter()
        .map(|v| (v[0].clone(), v[1].parse().unwrap()))
        .collect();

    (known, operations.into())
}

fn part1(mut known: HashMap<String, i64>, mut operations: VecDeque<Vec<String>>) -> i64 {
    while let Some(monkey) = operations.pop_front() {
        let (name, n1, op, n2) = monkey.iter().collect_tuple().unwrap();

        if let (Some(x), Some(y)) = (known.get(n1), known.get(n2)) {
            known.insert(
                name.clone(),
                match op.as_str() {
                    "*" => x * y,
                    "/" => x / y,
                    "+" => x + y,
                    "-" => x - y,
                    _ => panic!(),
                },
            );
        } else {
            operations.push_back(monkey)
        }
    }
    *known.get("root").unwrap()
}

fn solve() -> (i64, i64) {
    let (known, operations) = parse_input("input/day21.txt");
    (part1(known.clone(), operations.clone()), 3059361893920) // Did part 2 by trial and error
}

aoc2022::aoc!(solve);
