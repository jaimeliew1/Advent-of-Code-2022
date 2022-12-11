fn parse_input(filename: &str) -> (Vec<Vec<char>>, Vec<Vec<u64>>) {
    let content = std::fs::read_to_string(filename).expect("can't find file");
    let (chunk1, chunk2) = content.split_once("\n\n").unwrap();

    // Parse crate stack into a vector of character vectors.
    let mut stack: Vec<Vec<char>> = Vec::new();
    for i in 0..9 {
        let x: Vec<char> = chunk1
            .lines()
            .rev()
            .skip(1)
            .map(|l| l.chars().skip(1 + (i * 4) as usize).next().unwrap())
            .filter(|&x| x != ' ')
            .collect();

        stack.push(x);
    }

    // Parse instructions as Vector of integer triplets.
    let instructions: Vec<Vec<u64>> = chunk2
        .lines()
        .map(|l| {
            l.split_whitespace()
                .skip(1)
                .step_by(2)
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();

    (stack, instructions)
}

fn solve() -> (String, String) {
    let (stack, instructions) = parse_input("input/day05.txt");

    // Part1
    let mut stack1 = stack.clone();
    for instr in instructions.iter() {
        for _ in 0..instr[0] {
            let val = stack1[instr[1] as usize - 1].pop().unwrap();
            stack1[instr[2] as usize - 1].push(val);
        }
    }
    let part1: String = stack1.iter().map(|v| v.last().unwrap()).collect();

    // Part2
    let mut stack2 = stack.clone();
    for instr in instructions.iter() {
        let mut temp: Vec<char> = Vec::new();
        for _ in 0..instr[0] {
            temp.push(stack2[instr[1] as usize - 1].pop().unwrap());
        }
        for val in temp.iter().rev() {
            stack2[instr[2] as usize - 1].push(*val);
        }
    }
    let part2: String = stack2.iter().map(|v| v.last().unwrap()).collect();
    (part1, part2)
}

aoc2022::aoc!(solve);
