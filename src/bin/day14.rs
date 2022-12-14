use itertools::Itertools;

const SOURCEX: usize = 500;
const SOURCEY: usize = 0;

fn parse_input(filename: &str) -> Vec<Vec<(usize, usize)>> {
    std::fs::read_to_string(filename)
        .expect("can't find file")
        .lines()
        .map(|s| {
            s.split(" -> ")
                .map(|xy| xy.split_once(',').unwrap())
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .collect()
        })
        .collect()
}

fn next_sand(map: &[[char; 1000]; 200], y_max: usize) -> Option<(usize, usize)> {
    let (mut x, mut y): (usize, usize) = (SOURCEX, SOURCEY);
    loop {
        if y == y_max {
            return None;
        }
        if map[y + 1][x] == ' ' {
            y += 1;
        } else if map[y + 1][x - 1] == ' ' {
            y += 1;
            x -= 1;
        } else if map[y + 1][x + 1] == ' ' {
            y += 1;
            x += 1;
        } else {
            return Some((x, y));
        }
    }
}

fn count_sand(map: &[[char; 1000]; 200], y_max: usize) -> usize {
    let mut map: [[char; 1000]; 200] = map.clone();
    while let Some((x, y)) = next_sand(&map, y_max) {
        map[y][x] = 'o';
        if (x, y) == (SOURCEX, SOURCEY) {
            break;
        }
    }
    map.iter().flatten().filter(|c| **c == 'o').count()
}
fn solve() -> (usize, usize) {
    let mut data = parse_input("input/day14.txt");
    let y_max = *data.iter().flatten().map(|(_, y)| y).max().unwrap();
    data.push(vec![(0, y_max + 2), (1000 - 1, y_max + 2)]);

    // Populate the map from input file.
    let mut map: [[char; 1000]; 200] = [[' '; 1000]; 200];
    for mut path_pair in data.iter().map(|p| p.iter().tuple_windows()) {
        while let Some(((x1, y1), (x2, y2))) = path_pair.next() {
            if x1 == x2 {
                (*y1.min(y2)..=*y1.max(y2)).for_each(|y| map[y][*x1] = '#');
            } else if y1 == y2 {
                (*x1.min(x2)..=*x1.max(x2)).for_each(|x| map[*y1][x] = '#');
            }
        }
    }

    (count_sand(&map, y_max + 1), count_sand(&map, y_max + 2))
}

aoc2022::aoc!(solve);
