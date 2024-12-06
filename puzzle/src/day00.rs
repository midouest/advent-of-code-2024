use std::{error::Error, fs::read_to_string, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input/day00.txt")?;
    let now = Instant::now();
    println!("p1: {} ({}µs)", part1(&input)?, now.elapsed().as_micros());
    let now = Instant::now();
    println!("p2: {} ({}µs)", part2(&input)?, now.elapsed().as_micros());
    Ok(())
}

fn part1(_input: &str) -> Result<usize, Box<dyn Error>> {
    todo!();
}

fn part2(_input: &str) -> Result<usize, Box<dyn Error>> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1(EXAMPLE)?, 0);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2(EXAMPLE)?, 0);
        Ok(())
    }
}
