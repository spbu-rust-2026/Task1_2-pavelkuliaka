use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let lines_iter = io::stdin().lock().lines().map(|line| line.unwrap());
    let mut sum: u128 = 0;
    for line in lines_iter {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "-1" {
            break;
        }
        if let Ok(value) = line.parse::<u128>()
            && value != 0
        {
            sum += value;
            continue;
        }
        println!("NaN");
        return Ok(());
    }
    println!("{sum}");
    Ok(())
}
