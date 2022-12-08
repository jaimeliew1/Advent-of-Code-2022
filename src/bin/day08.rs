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

    let mut edge_view: Vec<Vec<bool>> = data
        .iter()
        .map(|l| l.iter().map(|_| false).collect())
        .collect();
    let mut trees_visible: Vec<Vec<u64>> =
        data.iter().map(|l| l.iter().map(|_| 1).collect()).collect();

    for (i, row) in data.iter().enumerate() {
        for (j, height) in row.iter().enumerate() {
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
                let treerow: Vec<&u64> = (1..)
                    .map(|c| {
                        data.get((i as i32 + c * dy) as usize)
                            .and_then(|row| row.get((j as i32 + c * dx) as usize))
                    })
                    .while_some()
                    .collect();
                // part 1
                edge_view[i][j] |= treerow.iter().all(|&&h| h < *height);
                // part 2
                let mut score = 0;
                for h in treerow.iter() {
                    if *h < height {
                        score += 1
                    } else {
                        score += 1;
                        break;
                    }
                }
                trees_visible[i][j] *= score;
            }
        }
    }

    (
        edge_view.iter().flatten().filter(|&&x| x).count() as u64,
        *trees_visible.iter().flatten().max().unwrap(),
    )
}

aoc2022::aoc!(solve);
