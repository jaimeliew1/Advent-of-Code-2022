use itertools::Itertools;

fn solve() -> (u64, u64) {
    let data: Vec<char> = std::fs::read_to_string("input/day06.txt").unwrap().chars().collect();
    let part1 = data.windows(4).position(|x| x.iter().all_unique()).unwrap() + 4;
    let part2 = data.windows(14).position(|x| x.iter().all_unique()).unwrap() + 14;

    (part1 as u64, part2 as u64)
}

aoc2022::aoc!(solve);
