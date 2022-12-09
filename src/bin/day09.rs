use std::collections::HashMap;

fn parse_input(filename: &str) -> Vec<(char, usize)> {
    std::fs::read_to_string(filename)
        .expect("can't find file")
        .lines()
        .map(|s| s.split_once(" ").unwrap())
        .map(|(a, b)| (a.chars().next().unwrap(), b.parse().unwrap()))
        .collect()
}

fn solve() -> (u64, u64) {
    let data = parse_input("input/day09.txt");

    let mut map: HashMap<(i32, i32), u64> = HashMap::new();
    let (mut x, mut y): (i32, i32) = (0, 0);
    let (mut xr, mut yr): (i32, i32) = (0, 0);

    for (dir, rep) in data.iter() {
        let (dx, dy) = match dir {
            'L' => (-1, 0),
            'R' => (1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => panic!(),
        };
        for _ in 0..*rep {
            let (_x, _y) = (x + dx, y + dy);
            match ((_x - xr).abs(), (_y - yr).abs()) {
                (2, _) => {
                    xr = (xr + _x) / 2;
                    yr = _y;
                }
                (_, 2) => {
                    xr = _x;
                    yr = (yr + _y) / 2;
                }
                _ => (),
            }
            x = _x;
            y = _y;

            // println!("{},{},{},{}", x, y, xr, yr);
            map.entry((xr, yr)).and_modify(|x| *x += 1).or_insert(1);
        }
    }
    (map.len() as u64, 0)
}

aoc2022::aoc!(solve);
