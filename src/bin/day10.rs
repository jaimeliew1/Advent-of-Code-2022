fn parse_input(filename: &str) -> Vec<Vec<String>> {
    std::fs::read_to_string(filename)
        .expect("can't find file")
        .lines()
        .map(|s| s.split_whitespace().map(|s| s.to_string()).collect())
        .collect()
}

fn solve() -> (i64, String) {
    let data = parse_input("input/day10.txt");

    let mut state: Vec<i64> = vec![1, 1];
    for op in data.iter() {
        match op.len() {
            1 => state.push(*state.last().unwrap()),
            2 => {
                let newval = state.last().unwrap() + op[1].parse::<i64>().unwrap();
                state.append(&mut vec![*state.last().unwrap(), newval]);
            }
            _ => panic!(),
        }
    }
    let part1 = state
        .iter()
        .enumerate()
        .skip(20)
        .step_by(40)
        .fold(0, |acc, (i, s)| acc + i as i64 * s);

    let mut part2: String = String::new();
    for (i, s) in state.iter().skip(1).enumerate() {
        if i % 40 == 0 {
            part2.push('\n');
        }
        if (i as i64 % 40 - s).abs() < 2 {
            part2.push('#')
        } else {
            part2.push(' ')
        }
    }
    (part1, part2)
}

aoc2022::aoc!(solve);
