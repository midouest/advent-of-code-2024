use std::{error::Error, fs::read_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input/day02.txt")?;
    println!("P1: {}", part1(&input)?);
    println!("P2: {}", part2(&input)?);
    Ok(())
}

fn parse(line: &str) -> Result<Vec<usize>, Box<dyn Error>> {
    let report = line
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(report)
}

fn is_safe_report(report: &[usize]) -> bool {
    let mut prev = report[0];
    let increasing = report[1] > report[0];
    let mut safe = true;
    for &level in &report[1..] {
        if level.abs_diff(prev) > 3
            || (increasing && level < prev)
            || (!increasing && level > prev)
            || level == prev
        {
            safe = false;
            break;
        }
        prev = level;
    }
    safe
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut count = 0;
    for line in input.lines() {
        let report = parse(line)?;
        if is_safe_report(&report) {
            count += 1;
        }
    }
    Ok(count)
}

fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut count = 0;
    for line in input.lines() {
        let report = parse(line)?;
        if is_safe_report(&report) {
            count += 1;
            continue;
        }

        for i in 0..report.len() {
            let mut new_report = report.clone();
            new_report.remove(i);
            if is_safe_report(&new_report) {
                count += 1;
                break;
            }
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1(EXAMPLE)?, 2);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2(EXAMPLE)?, 4);
        Ok(())
    }
}
