use itertools::Itertools;
fn parse_input(filename: &str) -> Vec<Vec<u64>> {
    let contents = std::fs::read_to_string(filename).expect("can't find file");
    contents
        .split("\n\n")
        .map(|par| par.lines().map(|l| l.parse().unwrap()).collect())
        .collect()
}

fn solve() -> (u64, u64) {
    let data = parse_input("input/day01.txt");
    let top_three: Vec<u64> = data
        .iter()
        .map(|chunk| chunk.iter().sum())
        .sorted()
        .rev()
        .take(3)
        .collect();

    (top_three[0], top_three.iter().sum())
}

aoc2022::aoc!(solve);
