use std::{collections::HashSet, error::Error, fs::read_to_string, str::FromStr, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input/day14.txt")?;
    let now = Instant::now();
    println!("p1: {} ({}µs)", part1(&input)?, now.elapsed().as_micros());
    let now = Instant::now();
    println!("p2: {} ({}µs)", part2(&input)?, now.elapsed().as_micros());
    Ok(())
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let robots = parse(input)?;
    Ok(calc_safety_factor(&robots, 101, 103))
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let (w, h) = (101, 103);
    let robots = parse(input)?;
    let cycle = robots[0].find_cycle(w, h);

    let (i, _) = (0..cycle)
        .map(|i| (i, calc_regularity(&robots, w, h, i)))
        .max_by(|(_, r0), (_, r1)| r0.cmp(r1))
        .ok_or("Not found")?;
    Ok(i)
}

fn parse(input: &str) -> Result<Vec<Robot>, Box<dyn Error>> {
    Ok(input.lines().map(|s| s.parse()).collect::<Result<_, _>>()?)
}

fn calc_safety_factor(robots: &[Robot], w: i32, h: i32) -> usize {
    let (qx, qy) = (w / 2, h / 2);

    let mut quadrants = [0usize; 4];
    for robot in robots {
        let (x, y) = robot.run(w, h, 100);
        if x < qx {
            if y < qy {
                quadrants[0] += 1;
            } else if y > qy {
                quadrants[1] += 1;
            }
        } else if x > qx {
            if y < qy {
                quadrants[2] += 1;
            } else if y > qy {
                quadrants[3] += 1;
            }
        }
    }

    quadrants.iter().product()
}

fn calc_regularity(robots: &[Robot], w: i32, h: i32, i: i32) -> usize {
    let mut visited = HashSet::new();
    for robot in robots {
        let pos = robot.run(w, h, i);
        visited.insert(pos);
    }
    let visited: Vec<_> = visited.into_iter().collect();
    let mut regular = 0;
    for i in 0..visited.len() {
        let (x0, y0) = visited[i];
        for j in i..visited.len() {
            let (x1, y1) = visited[j];
            let (dx, dy) = (x1.abs_diff(x0), y1.abs_diff(y0));
            if dx == 1 && dy == 0 || dx == 0 && dy == 1 || dx == 1 && dy == 1 {
                regular += 1;
            }
        }
    }
    regular
}

struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Robot {
    fn run(&self, w: i32, h: i32, i: i32) -> (i32, i32) {
        let mut x = (self.x + self.dx * i) % w;
        if x < 0 {
            x += w;
        }
        let mut y = (self.y + self.dy * i) % h;
        if y < 0 {
            y += h;
        }
        (x, y)
    }

    fn find_cycle(&self, w: i32, h: i32) -> i32 {
        let mut visited = HashSet::new();
        let mut i = 0;
        loop {
            let pos = self.run(w, h, i);
            if visited.contains(&pos) {
                break i;
            }
            visited.insert(pos);
            i += 1;
        }
    }
}

impl FromStr for Robot {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s
            .split(['p', '=', ',', ' ', 'v'])
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "invalid integer")?;
        Ok(Robot::from(v))
    }
}

impl From<Vec<i32>> for Robot {
    fn from(v: Vec<i32>) -> Self {
        Self {
            x: v[0],
            y: v[1],
            dx: v[2],
            dy: v[3],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        let robots = parse(EXAMPLE)?;
        assert_eq!(calc_safety_factor(&robots, 11, 7), 12);
        Ok(())
    }
}
