use std::fs;

fn parse_input(filename: &str) -> Vec<i32> {
    let contents = fs::read_to_string(filename).expect("can't find file");
    let data: Vec<i32> = contents.lines().map(|s| s.parse().unwrap()).collect();
    data
}

fn part1(data: &Vec<i32>) -> u32 {
    0
}

fn part2(data: &Vec<i32>) -> u32 {
    0
}

fn main() {
    let now = std::time::Instant::now();

    let data = parse_input("input/day00.txt");

    let ans_part1 = part1(&data);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&data);
    println!("part2: {}", ans_part2);

    let time = now.elapsed().as_micros();
    println!("Time: {}μs", time);
}
