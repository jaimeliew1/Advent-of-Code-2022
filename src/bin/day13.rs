use itertools::{EitherOrBoth, Itertools};
use json::JsonValue;
use std::cmp::Ordering;

fn parse_input(filename: &str) -> Vec<JsonValue> {
    // Parse the distress signals as JSON.
    std::fs::read_to_string(filename)
        .expect("can't find file")
        .split_whitespace()
        .map(|s| json::parse(s).unwrap())
        .collect()
}

fn distress_cmp(a: &JsonValue, b: &JsonValue) -> Ordering {
    // Compares pair of distress signals and returns total ordering (lt, gt, or eq).
    let mut iter = a.members().zip_longest(b.members());
    while let Some(pair) = iter.next() {
        let thiscomp = match pair {
            EitherOrBoth::Both(JsonValue::Array(x), JsonValue::Array(y)) => {
                distress_cmp(&JsonValue::Array(x.clone()), &JsonValue::Array(y.clone()))
            }
            EitherOrBoth::Both(JsonValue::Number(x), JsonValue::Array(y)) => distress_cmp(
                &JsonValue::Array(vec![JsonValue::Number(*x)]),
                &JsonValue::Array(y.clone()),
            ),
            EitherOrBoth::Both(JsonValue::Array(x), JsonValue::Number(y)) => distress_cmp(
                &JsonValue::Array(x.clone()),
                &JsonValue::Array(vec![JsonValue::Number(*y)]),
            ),
            EitherOrBoth::Both(JsonValue::Number(x), JsonValue::Number(y)) => {
                Into::<f32>::into(*x).total_cmp(&Into::<f32>::into(*y))
            }
            EitherOrBoth::Left(_) => Ordering::Greater,
            EitherOrBoth::Right(_) => Ordering::Less,
            _ => panic!(),
        };
        if thiscomp.is_ne() {
            return thiscomp;
        }
    }
    Ordering::Equal
}
fn solve() -> (usize, usize) {
    let mut data = parse_input("input/day13.txt");
    // Part 1. Compare pairs of distress signals, and add index if first is less than second.
    let part1 = data
        .iter()
        .tuples()
        .enumerate()
        .filter(|(_, (a, b))| distress_cmp(a, b).is_lt())
        .fold(0, |acc, (i, _)| acc + i + 1);

    // Part 2. Add [[2]] and [[6]] to distress vector and sort.
    data.push(json::parse("[[2]]").unwrap());
    data.push(json::parse("[[6]]").unwrap());
    data.sort_unstable_by(|a, b| distress_cmp(a, b));

    let idx_2 = data.iter().position(|x| x.dump() == "[[2]]").unwrap();
    let idx_6 = data.iter().position(|x| x.dump() == "[[6]]").unwrap();

    (part1, (idx_2 + 1) * (idx_6 + 1))
}

aoc2022::aoc!(solve);
