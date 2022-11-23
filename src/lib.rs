#[macro_export]
macro_rules! aoc {
    ($solver:expr) => {
        fn main() {
            let now = std::time::Instant::now();
            let (ans1, ans2) = $solver();
            let time = now.elapsed().as_micros();
            println!("Part 1: {}", ans1);
            println!("Part 2: {}", ans2);
            println!("time elapsed: {}μs", time);
        }
    };
}
