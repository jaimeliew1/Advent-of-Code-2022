use itertools::Itertools;

#[derive(Debug, Clone)]
enum Op {
    Old,
    Add(u64),
    Mul(u64),
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    Op: Op,
    DivBy: u64,
    TrueMonk: usize,
    FalseMonk: usize,
    Activity: u64,
}

fn parse_input(filename: &str) -> Vec<Monkey> {
    let mut out: Vec<Monkey> = Vec::new();
    let content = std::fs::read_to_string(filename).expect("can't find file");
    let monkey_strings: Vec<Vec<&str>> =
        content.split("\n\n").map(|p| p.lines().collect()).collect();

    for lines in monkey_strings.iter() {
        let monkey = Monkey {
            items: lines[1]
                .strip_prefix("  Starting items: ")
                .unwrap()
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect(),
            Op: match lines[2]
                .strip_prefix("  Operation: new = old ")
                .unwrap()
                .split_once(' ')
                .unwrap()
            {
                ("+", x) => Op::Add(x.parse().unwrap()),
                ("*", "old") => Op::Old,
                ("*", x) => Op::Mul(x.parse().unwrap()),
                _ => panic!(),
            },
            DivBy: lines[3].rsplit_once(' ').unwrap().1.parse().unwrap(),
            TrueMonk: lines[4].rsplit_once(' ').unwrap().1.parse().unwrap(),
            FalseMonk: lines[5].rsplit_once(' ').unwrap().1.parse().unwrap(),
            Activity: 0,
        };
        out.push(monkey);
    }

    out
}

fn monkeybusiness(mut monkeys: Vec<Monkey>, cycles: u64, denom: u64) -> u64 {
    let supermodulo: u64 = monkeys.iter().map(|monkey| monkey.DivBy).product();

    for _ in 0..cycles {
        for i in 0..monkeys.len() {
            let (truemonkidx, falsemonkidx) = (monkeys[i].TrueMonk, monkeys[i].FalseMonk);
            while let Some(item) = monkeys[i].items.pop() {
                monkeys[i].Activity += 1;
                let worry = match monkeys[i].Op {
                    Op::Old => (item * item) / denom % supermodulo,
                    Op::Add(x) => (item + x) / denom % supermodulo,
                    Op::Mul(x) => (item * x) / denom % supermodulo,
                };
                if worry % monkeys[i].DivBy == 0 {
                    monkeys[truemonkidx].items.push(worry);
                } else {
                    monkeys[falsemonkidx].items.push(worry);
                }
            }
        }
    }
    monkeys
        .iter()
        .map(|m| m.Activity)
        .sorted()
        .rev()
        .take(2)
        .product()
}
fn solve() -> (u64, u64) {
    let data = parse_input("input/day11.txt");
    (
        monkeybusiness(data.clone(), 20, 3),
        monkeybusiness(data.clone(), 10000, 1),
    )
}

aoc2022::aoc!(solve);
