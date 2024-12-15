use std::{error::Error, fs::read_to_string, str::FromStr, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input/day13.txt")?;
    let now = Instant::now();
    println!("p1: {} ({}µs)", part1(&input)?, now.elapsed().as_micros());
    let now = Instant::now();
    println!("p2: {} ({}µs)", part2(&input)?, now.elapsed().as_micros());
    Ok(())
}

fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let machines = parse(input)?;
    Ok(machines.iter().flat_map(Machine::cost).sum())
}

fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let machines = parse(input)?;
    Ok(machines
        .iter()
        .map(Machine::corrected)
        .flat_map(|m| m.cost())
        .sum())
}

fn parse(input: &str) -> Result<Vec<Machine>, Box<dyn Error>> {
    let machines = input
        .split("\n\n")
        .map(Machine::from_str)
        .collect::<Result<_, _>>()?;
    Ok(machines)
}

impl FromStr for Machine {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let xys = s
            .lines()
            .map(|line| {
                line.split_once(": ")
                    .ok_or("missing colon delimiter")
                    .and_then(|(_, rest)| rest.split_once(", ").ok_or("missing comma delimiter"))
                    .and_then(|(x, y)| {
                        x[2..]
                            .parse::<i64>()
                            .map_err(|_| "invalid x integer")
                            .and_then(|x| {
                                y[2..]
                                    .parse::<i64>()
                                    .map_err(|_| "invalid y integer")
                                    .map(|y| (x, y))
                            })
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Machine::new(
            xys[0].0, xys[0].1, xys[1].0, xys[1].1, xys[2].0, xys[2].1,
        ))
    }
}

struct Machine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

impl Machine {
    fn new(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> Self {
        Self {
            ax,
            ay,
            bx,
            by,
            px,
            py,
        }
    }

    fn corrected(&self) -> Self {
        Self {
            px: self.px + 10000000000000,
            py: self.py + 10000000000000,
            ..*self
        }
    }

    fn cost(&self) -> Option<i64> {
        let nb_num = self.ax * self.py - self.px * self.ay;
        let nb_den = self.ax * self.by - self.ay * self.bx;
        if nb_num % nb_den != 0 {
            return None;
        }
        let nb = nb_num / nb_den;
        let na_num = self.px - nb * self.bx;
        let na_den = self.ax;
        if na_num % na_den != 0 {
            return None;
        }
        let na = na_num / na_den;
        Some(3 * na + nb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1(EXAMPLE)?, 480);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2(EXAMPLE)?, 875318608908);
        Ok(())
    }
}
