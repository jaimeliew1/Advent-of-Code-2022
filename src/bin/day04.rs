fn parse_input(filename: &str) -> Vec<Vec<u64>> {
    std::fs::read_to_string(filename)
        .expect("can't find file")
        .lines()
        .map(|s| {
            s.split(&['-', ','][..])
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

fn solve() -> (u64, u64) {
    let data = parse_input("input/day04.txt");

    // Find completely overlapping ranges.
    let part1 = data
        .iter()
        .filter(|x| ((x[0] <= x[2]) & (x[1] >= x[3])) | ((x[2] <= x[0]) & (x[3] >= x[1])))
        .count();

    // find ranges with any overlap.
    let part2 = data
        .iter()
        .filter(|x| !((x[1] < x[2]) | (x[0] > x[3])))
        .count();
    (part1 as u64, part2 as u64)
}

aoc2022::aoc!(solve);
