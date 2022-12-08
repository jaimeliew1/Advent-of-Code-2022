fn parse_input(filename: &str) -> Vec<u64> {
    std::fs::read_to_string(filename)
        .expect("can't find file")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn solve() -> (u64, u64) {
    let data = parse_input("input/dayXX.txt");
    (0, 0)
}

aoc2022::aoc!(solve);
