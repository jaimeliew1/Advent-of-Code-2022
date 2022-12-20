#[derive(Debug)]
struct Number {
    pos: i64,
    value: i64,
}

fn parse_input(filename: &str) -> Vec<Number> {
    std::fs::read_to_string(filename)
        .expect("can't find file")
        .lines()
        .enumerate()
        .map(|(i, s)| Number {
            value: s.parse().unwrap(),
            pos: i as i64,
        })
        .collect()
}

fn mix(mut data: Vec<Number>) -> Vec<Number> {
    let n = data.len() as i64;
    for i in 0..n {
        let idx = data.iter().position(|x| x.pos == i).unwrap();
        let new_idx = ((idx as i64 + data[idx].value - 1).rem_euclid(n - 1) + 1) as usize;
        let x = data.remove(idx);
        data.insert(new_idx, x);
    }
    data
}

fn coordinates(data: &Vec<Number>) -> i64 {
    let pos0 = data.iter().position(|x| x.value == 0).unwrap();
    [1000, 2000, 3000]
        .map(|i| (pos0 as i64 + i).rem_euclid(data.len() as i64))
        .map(|idx| data[idx as usize].value)
        .iter()
        .sum()
}

fn solve() -> (i64, i64) {
    let mut part1_data = parse_input("input/day20.txt");
    let mut part2_data: Vec<Number> = part1_data
        .iter()
        .map(|x| Number {
            pos: x.pos,
            value: x.value * 811589153,
        })
        .collect();

    part1_data = mix(part1_data);
    for _ in 0..10 {
        part2_data = mix(part2_data);
    }
    (coordinates(&part1_data), coordinates(&part2_data))
}

aoc2022::aoc!(solve);
