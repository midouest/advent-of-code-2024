use std::{collections::HashMap, error::Error, fs::read_to_string, iter::zip};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input/day01.txt")?;
    println!("P1: {}", part1(&input)?);
    println!("P2: {}", part2(&input)?);
    Ok(())
}

fn parse(input: &str) -> Result<(Vec<usize>, Vec<usize>), Box<dyn Error>> {
    let (mut left, mut right) = (vec![], vec![]);
    for line in input.trim().lines() {
        let nums = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;
        left.push(nums[0]);
        right.push(nums[1]);
    }
    Ok((left, right))
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let (mut left, mut right) = parse(input)?;
    left.sort();
    right.sort();
    let total_distance = zip(left, right).map(|(a, b)| a.abs_diff(b)).sum();
    Ok(total_distance)
}

fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    let (left, right) = parse(input)?;
    let mut counts = HashMap::new();
    for n in right {
        *counts.entry(n).or_insert(0) += 1
    }
    let similarity_score = left
        .into_iter()
        .map(|n| counts.get(&n).copied().unwrap_or(0) * n)
        .sum();
    Ok(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1(EXAMPLE)?, 11);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2(EXAMPLE)?, 31);
        Ok(())
    }
}
