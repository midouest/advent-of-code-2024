use std::{error::Error, fs::read_to_string, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input/day07.txt")?;
    let now = Instant::now();
    println!("p1: {} ({}µs)", part1(&input)?, now.elapsed().as_micros());
    let now = Instant::now();
    println!("p2: {} ({}µs)", part2(&input)?, now.elapsed().as_micros());
    Ok(())
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut sum = 0;
    for line in input.lines() {
        let nums = line
            .split(&[':', ' '])
            .filter(|s| !s.is_empty())
            .map(|n| n.parse())
            .collect::<Result<Vec<usize>, _>>()?;
        let (test0, nums) = (nums[0], &nums[1..]);
        let mut frontier = vec![(test0, nums.len() - 1)];
        while let Some((test1, mut i)) = frontier.pop() {
            let n = nums[i];
            if i == 0 {
                if test1 == n {
                    sum += test0;
                    break;
                }
            } else {
                i -= 1;
                if test1 % n == 0 {
                    frontier.push((test1 / n, i));
                }
                if test1 > n {
                    frontier.push((test1 - n, i));
                }
            }
        }
    }
    Ok(sum)
}

fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    let mut sum = 0;
    for line in input.lines() {
        let nums = line
            .split(&[':', ' '])
            .filter(|s| !s.is_empty())
            .map(|n| n.parse())
            .collect::<Result<Vec<usize>, _>>()?;
        let (test0, nums) = (nums[0], &nums[1..]);
        let mut frontier = vec![(test0, nums.len() - 1)];
        while let Some((test1, mut i)) = frontier.pop() {
            let n = nums[i];
            if i == 0 {
                if test1 == n {
                    sum += test0;
                    break;
                }
            } else {
                i -= 1;
                if test1 % n == 0 {
                    frontier.push((test1 / n, i));
                }
                if test1 > n {
                    frontier.push((test1 - n, i));
                }
                if let Some(n) = unconcat(test1, n) {
                    frontier.push((n, i));
                }
            }
        }
    }
    Ok(sum)
}

fn unconcat(mut a: usize, b: usize) -> Option<usize> {
    let mut c = 0;
    let mut i = 0;
    while a > 0 && c < b {
        c += 10usize.pow(i) * (a % 10);
        a /= 10;
        i += 1;
    }
    (c == b).then_some(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1(EXAMPLE)?, 3749);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2(EXAMPLE)?, 11387);
        Ok(())
    }
}
