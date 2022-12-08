use itertools::Itertools;

fn parse_input(filename: &str) -> Vec<Vec<u64>> {
    std::fs::read_to_string(filename)
        .expect("can't find file")
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u64).collect())
        .collect()
}

fn solve() -> (u64, u64) {
    let data = parse_input("input/day08.txt");

    let (mut edge_view_count, mut best_score) = (0, 0);
    for (i, j) in (0..data.len()).cartesian_product(0..data[0].len()) {
        let height = data[i][j];
        let (mut edge_view, mut score) = (false, 1);
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let treerow: Vec<&u64> = (1..)
                .map(|c| {
                    data.get((i as i32 + c * dy) as usize)
                        .and_then(|row| row.get((j as i32 + c * dx) as usize))
                })
                .while_some()
                .collect();

            edge_view |= treerow.iter().all(|&&h| h < height);
            let mut s = 0;
            for h in treerow.iter() {
                if **h < height {
                    s += 1
                } else {
                    s += 1;
                    break;
                }
            }
            score *= s;
        }
        best_score = best_score.max(score);
        edge_view_count += edge_view as u64;
    }

    (edge_view_count, best_score)
}

aoc2022::aoc!(solve);
