use std::error::Error;
use std::process::Command;

fn run_solution(day: u32) -> Result<(), Box<dyn Error>> {
    let day = format!("day{:0>2}", day);
    println!("{}", day);
    let cmd = Command::new("cargo")
        .args(&["run", "--release", "--bin", &day])
        .output()?;
    if cmd.status.success() {
        let output = String::from_utf8(cmd.stdout)?;
        println!("Problem: {}\n{}", day, output);
    } else {
        println!("Problem {} failed to run.", day);
        println!("{}", String::from_utf8(cmd.stderr)?);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    match std::env::args()
        .nth(1)
        .and_then(|x| Some(x.parse::<u32>()))
        .transpose()?
    {
        Some(x) => run_solution(x)?,
        _ => {
            for i in 1..25 {
                run_solution(i)?;
            }
        }
    };
    Ok(())
}
