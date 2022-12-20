use good_lp::*;

fn parse_input(filename: &str) -> Vec<[i32; 6]> {
    std::fs::read_to_string(filename)
        .expect("can't find file")
        .lines()
        .map(|s| {
            let splits: Vec<_> = s.split_whitespace().collect();
            [
                splits[6].parse().unwrap(),
                splits[12].parse().unwrap(),
                splits[18].parse().unwrap(),
                splits[21].parse().unwrap(),
                splits[27].parse().unwrap(),
                splits[30].parse().unwrap(),
            ]
        })
        .collect()
}

fn max_geodes(costs: &[i32; 6], n_iters: usize) -> i32 {
    // Oof. Formulated the problem as a Linear Program. Solve using an LP solver.
    let [ore_ore, clay_ore, obs_ore, obs_clay, geode_ore, geode_obs] = *costs;
    let mut variables = variables!();
    let resources: Vec<Vec<Variable>> = (0..n_iters + 1)
        .map(|_| {
            (0..4)
                .map(|_| variables.add(variable().integer().min(0)))
                .collect()
        })
        .collect();
    let robots: Vec<Vec<Variable>> = (0..n_iters + 1)
        .map(|_| {
            (0..4)
                .map(|_| variables.add(variable().integer().min(0)))
                .collect()
        })
        .collect();
    let decisions: Vec<Vec<Variable>> = (0..n_iters + 1)
        .map(|_| {
            (0..4)
                .map(|_| variables.add(variable().integer().min(0)))
                .collect()
        })
        .collect();

    let mut problem = variables
        .maximise(resources[n_iters][3])
        .using(default_solver);

    // Initialise resources, robots, and decision at t=0.
    problem.add_constraint(constraint!(resources[0][0] == 0));
    problem.add_constraint(constraint!(resources[0][1] == 0));
    problem.add_constraint(constraint!(resources[0][2] == 0));
    problem.add_constraint(constraint!(resources[0][3] == 0));
    problem.add_constraint(constraint!(robots[0][0] == 1));
    problem.add_constraint(constraint!(robots[0][1] == 0));
    problem.add_constraint(constraint!(robots[0][2] == 0));
    problem.add_constraint(constraint!(robots[0][3] == 0));
    problem.add_constraint(constraint!(decisions[0][0] == 0));
    problem.add_constraint(constraint!(decisions[0][1] == 0));
    problem.add_constraint(constraint!(decisions[0][2] == 0));
    problem.add_constraint(constraint!(decisions[0][3] == 0));

    for i in 1..n_iters + 1 {
        // Ensure a maximum of 1 new robot is made per timestep.
        problem.add_constraint(constraint!(
            decisions[i][0] + decisions[i][1] + decisions[i][2] + decisions[i][3] <= 1
        ));
        // conservation of number of robots
        problem.add_constraint(constraint!(
            robots[i][0] == robots[i - 1][0] + decisions[i - 1][0]
        ));
        problem.add_constraint(constraint!(
            robots[i][1] == robots[i - 1][1] + decisions[i - 1][1]
        ));
        problem.add_constraint(constraint!(
            robots[i][2] == robots[i - 1][2] + decisions[i - 1][2]
        ));
        problem.add_constraint(constraint!(
            robots[i][3] == robots[i - 1][3] + decisions[i - 1][3]
        ));
        // conservation of resources
        problem.add_constraint(constraint!(
            resources[i][0]
                == resources[i - 1][0] + robots[i - 1][0]
                    - ore_ore * decisions[i - 1][0]
                    - clay_ore * decisions[i - 1][1]
                    - obs_ore * decisions[i - 1][2]
                    - geode_ore * decisions[i - 1][3]
        ));
        problem.add_constraint(constraint!(
            resources[i][1]
                == resources[i - 1][1] + robots[i - 1][1] - obs_clay * decisions[i - 1][2]
        ));
        problem.add_constraint(constraint!(
            resources[i][2]
                == resources[i - 1][2] + robots[i - 1][2] - geode_obs * decisions[i - 1][3]
        ));
        problem.add_constraint(constraint!(
            resources[i][3] == resources[i - 1][3] + robots[i - 1][3]
        ));
        // Ensure enough resources available when making decision
        problem.add_constraint(constraint!(
            resources[i][0]
                >= ore_ore * decisions[i][0]
                    + clay_ore * decisions[i][1]
                    + obs_ore * decisions[i][2]
                    + geode_ore * decisions[i][3]
        ));
        problem.add_constraint(constraint!(resources[i][1] >= obs_clay * decisions[i][2]));
        problem.add_constraint(constraint!(resources[i][2] >= geode_obs * decisions[i][3]));
    }
    let solution = problem.solve().unwrap();
    solution.value(resources[n_iters][3]) as i32
}

fn solve() -> (i32, i32) {
    let data = parse_input("input/day19.txt");
    let part1 = data
        .iter()
        .enumerate()
        .map(|(i, costs)| (i + 1) as i32 * max_geodes(costs, 24))
        .sum();
    let part2 = data
        .iter()
        .take(3)
        .map(|costs| max_geodes(costs, 32))
        .product();

    (part1, part2)
}

aoc2022::aoc!(solve);
