

fn parse_input(filename: &str) -> Vec<i32> {
    let contents = std::fs::read_to_string(filename).expect("can't find file");
    let data: Vec<i32> = contents.lines().map(|s| s.parse().unwrap()).collect();
    data
}

fn solve() -> (u64, u64) {
    let data = parse_input("input/dayXX.txt");
    (0, 0)
}

aoc2022::aoc!(solve);
