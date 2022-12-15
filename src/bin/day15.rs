use itertools::Itertools;

const Y_TARGET: i64 = 2000000;

fn parse_input(filename: &str) -> Vec<(i64, i64, i64, i64)> {
    std::fs::read_to_string(filename)
        .expect("can't find file")
        .lines()
        .map(|s| {
            s.split('=')
                .skip(1)
                .map(|s| {
                    s.chars()
                        .take_while(|x| x.is_numeric() || *x == '-')
                        .collect::<String>()
                        .parse()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn contained_in(x: i64, y: i64, x0: i64, y0: i64, x1: i64, y1: i64) -> bool {
    (x - x0).abs() + (y - y0).abs() <= (x1 - x0).abs() + (y1 - y0).abs() && (x, y) != (x1, y1)
}
fn solve() -> (usize, i64) {
    let data = parse_input("input/day15.txt");

    let part1 = (-1000000..6000000)
        .filter(|x| {
            data.iter()
                .any(|(x0, y0, x1, y1)| contained_in(*x, Y_TARGET, *x0, *y0, *x1, *y1))
        })
        .count();

    let mut candidates: Vec<(i64, i64)> = Vec::new();
    for (x0, y0, x1, y1) in data.iter() {
        let D = (x1 - x0).abs() + (y1 - y0).abs();
        let cand1: Vec<(i64, i64)> = (-(D + 1)..D + 1)
            .map(|dx| (x0 + dx, y0 + (D + 1) - dx))
            .collect();
        let cand2: Vec<(i64, i64)> = (-(D + 1)..D + 1)
            .map(|dx| (x0 + dx, y0 - (D + 1) + dx))
            .collect();
        candidates.extend(cand1.iter());
        candidates.extend(cand2.iter());
    }
    candidates = candidates
        .into_iter()
        .filter(|(x, _)| *x >= 0 && *x <= 4000000)
        .filter(|(_, y)| *y >= 0 && *y <= 4000000)
        .collect();
    let part2 = *candidates
        .iter()
        .find(|(x, y)| {
            !data
                .iter()
                .any(|(x0, y0, x1, y1)| contained_in(*x, *y, *x0, *y0, *x1, *y1))
        })
        .unwrap();

    (part1, part2.0 * 4000000 + part2.1)
}

aoc2022::aoc!(solve);
