use std::{error::Error, fs::read_to_string};

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input/day03.txt")?;
    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)")?;
    let sum = re
        .captures_iter(input)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            let a = a.parse::<usize>()?;
            let b = b.parse::<usize>()?;
            Ok(a * b)
        })
        .sum::<Result<usize, Box<dyn Error>>>()?;
    Ok(sum)
}

fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    let re = Regex::new(r"(?:(do(?:n't)?)\(\))|(?:(mul)\((\d+),(\d+)\))")?;
    let mut enabled = true;
    let mut sum = 0;
    for c in re.captures_iter(input) {
        let mut matches = c.iter().skip(1).filter_map(|m| m.map(|m| m.as_str()));
        match matches.next() {
            Some("do") => enabled = true,
            Some("don't") => enabled = false,
            Some("mul") if enabled => {
                sum += matches
                    .flat_map(|s| s.parse::<usize>().ok())
                    .product::<usize>();
            }
            _ => (),
        };
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1(EXAMPLE1)?, 161);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2(EXAMPLE2)?, 48);
        Ok(())
    }
}
