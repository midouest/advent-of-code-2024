use std::{collections::HashMap, error::Error, fs::read_to_string, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input/day11.txt")?;
    let now = Instant::now();
    println!("p1: {} ({}µs)", part1(&input)?, now.elapsed().as_micros());
    let now = Instant::now();
    println!("p2: {} ({}µs)", part2(&input)?, now.elapsed().as_micros());
    Ok(())
}

fn parse(input: &str) -> Result<HashMap<usize, usize>, Box<dyn Error>> {
    Ok(input
        .split_ascii_whitespace()
        .map(|s| s.parse().map(|n| (n, 1)))
        .collect::<Result<_, _>>()?)
}

fn next_states(stones: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut next = HashMap::new();
    for (&stone, &count) in stones {
        if stone == 0 {
            *next.entry(1).or_default() += count;
        } else {
            let digits = stone.ilog10() + 1;
            if digits % 2 == 0 {
                let div = 10usize.pow(digits / 2);
                let (left, right) = (stone / div, stone % div);
                *next.entry(left).or_default() += count;
                *next.entry(right).or_default() += count;
            } else {
                *next.entry(2024 * stone).or_default() += count;
            }
        }
    }
    next
}

fn count_stones(mut stones: HashMap<usize, usize>, n: usize) -> usize {
    for _ in 0..n {
        stones = next_states(&stones);
    }
    stones.values().sum()
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let stones = parse(input)?;
    Ok(count_stones(stones, 25))
}

fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    let stones = parse(input)?;
    Ok(count_stones(stones, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "125 17";

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1(EXAMPLE)?, 55312);
        Ok(())
    }
}
