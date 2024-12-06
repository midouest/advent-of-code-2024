use rayon::prelude::*;
use std::{collections::HashSet, error::Error, fs::read_to_string, str::FromStr, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input/day06.txt")?;
    let now = Instant::now();
    println!("p1: {} ({}µs)", part1(&input)?, now.elapsed().as_micros());
    let now = Instant::now();
    println!("p2: {} ({}ms)", part2(&input)?, now.elapsed().as_millis());
    Ok(())
}

#[derive(Clone)]
struct Sim {
    h: i32,
    w: i32,
    map: HashSet<(i32, i32)>,
    init: (i32, i32),
}

impl FromStr for Sim {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        let h = lines.len() as i32;
        let w = lines[0].len() as i32;
        let mut map = HashSet::new();
        let mut init = None;
        for (y, line) in lines.into_iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    map.insert((y as i32, x as i32));
                } else if c == '^' {
                    init = Some((y as i32, x as i32));
                }
            }
        }
        init.map(|init| Self { h, w, map, init }).ok_or("No guard")
    }
}

impl Sim {
    fn iter<'a>(&'a self) -> SimIterator<'a> {
        SimIterator {
            sim: self,
            pos: self.init,
            dir: 0,
        }
    }
}

struct SimIterator<'a> {
    sim: &'a Sim,
    pos: (i32, i32),
    dir: usize,
}

const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn next_dir_idx(dir: usize) -> usize {
    (dir + 1) % DIRS.len()
}

impl<'a> Iterator for SimIterator<'a> {
    type Item = ((i32, i32), usize);

    fn next(&mut self) -> Option<Self::Item> {
        let next = (self.pos, self.dir);
        let (y0, x0) = self.pos;
        if y0 < 0 || x0 < 0 || y0 == self.sim.h || x0 == self.sim.w {
            return None;
        }
        let (dy, dx) = DIRS[self.dir];
        let pos = (y0 + dy, x0 + dx);
        if self.sim.map.contains(&pos) {
            self.dir = next_dir_idx(self.dir);
        } else {
            self.pos = pos;
        }
        Some(next)
    }
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let sim: Sim = input.parse()?;
    let vis: HashSet<_> = sim.iter().map(|(pos, _)| pos).collect();
    Ok(vis.len())
}

fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    let sim0: Sim = input.parse()?;
    let loops = sim0
        .iter()
        .par_bridge()
        .flat_map(|(p0, _)| {
            let mut sim1 = sim0.clone();
            sim1.map.insert(p0);
            let mut vis = HashSet::new();
            for state in sim1.iter() {
                if vis.contains(&state) {
                    return Some(p0);
                }
                vis.insert(state);
            }
            None
        })
        .collect::<HashSet<_>>();
    Ok(loops.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1(EXAMPLE)?, 41);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2(EXAMPLE)?, 6);
        Ok(())
    }
}
