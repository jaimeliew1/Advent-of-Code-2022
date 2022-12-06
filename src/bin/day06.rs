use std::collections::HashSet;

fn parse_input(filename: &str) -> Vec<char> {
    std::fs::read_to_string(filename)
        .expect("can't find file")
        .chars()
        .collect()
}

fn first_all_unique_occurence(data: &[char], n: usize) -> usize {
    data.windows(n)
        .map(|x| HashSet::<char>::from_iter(x.to_owned()).len() == n)
        .position(|x| x)
        .unwrap()
        + n
}
fn solve() -> (u64, u64) {
    let data = parse_input("input/day06.txt");
    let part1 = first_all_unique_occurence(&data, 4);
    let part2 = first_all_unique_occurence(&data, 14);

    (part1 as u64, part2 as u64)
}

aoc2022::aoc!(solve);
