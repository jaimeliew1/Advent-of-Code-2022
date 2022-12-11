use std::collections::HashSet;

fn parse_input(filename: &str) -> Vec<(char, usize)> {
    std::fs::read_to_string(filename)
        .expect("can't find file")
        .lines()
        .map(|s| s.split_once(" ").unwrap())
        .map(|(a, b)| (a.chars().next().unwrap(), b.parse().unwrap()))
        .collect()
}

fn rope_tail_visits(path: &Vec<(char, usize)>, n: usize) -> u64 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); n];

    for (dir, rep) in path.iter() {
        let (dx, dy) = match dir {
            'L' => (-1, 0),
            'R' => (1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => panic!(),
        };

        for _ in 0..*rep {
            rope[0] = (rope[0].0 + dx, rope[0].1 + dy);
            for i in 1..n {
                let (x_prev, y_prev) = rope[i - 1];
                match ((x_prev - rope[i].0).abs(), (y_prev - rope[i].1).abs()) {
                    (2, 2) => {
                        rope[i].0 = (rope[i].0 + x_prev) / 2;
                        rope[i].1 = (rope[i].1 + y_prev) / 2;
                    }
                    (2, _) => {
                        rope[i].0 = (rope[i].0 + x_prev) / 2;
                        rope[i].1 = y_prev;
                    }
                    (_, 2) => {
                        rope[i].0 = x_prev;
                        rope[i].1 = (rope[i].1 + y_prev) / 2;
                    }
                    _ => (),
                }
            }
            visited.insert(*rope.last().unwrap());
        }
    }

    visited.len() as u64
}

fn solve() -> (u64, u64) {
    let data = parse_input("input/day09.txt");
    (rope_tail_visits(&data, 2), rope_tail_visits(&data, 10))
}

aoc2022::aoc!(solve);
