use std::{collections::HashSet, error::Error, fs::read_to_string, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input/day10.txt")?;
    let now = Instant::now();
    println!("p1: {} ({}µs)", part1(&input), now.elapsed().as_micros());
    let now = Instant::now();
    println!("p2: {} ({}µs)", part2(&input), now.elapsed().as_micros());
    Ok(())
}

const DELTAS: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

fn part1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|s| s.as_bytes().iter().map(|b| b - b'0').collect())
        .collect::<Vec<Vec<_>>>();
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    let mut frontier = vec![];
    for y in 0..h {
        for x in 0..w {
            if grid[y as usize][x as usize] == 0 {
                frontier.push(((y, x), (y, x), 1));
            }
        }
    }

    let mut trailheads = 0;
    let mut visited = HashSet::new();
    while let Some((first, last @ (y0, x0), len)) = frontier.pop() {
        visited.insert((first, last));
        if len == 10 {
            trailheads += 1;
            continue;
        }

        for (dy, dx) in DELTAS {
            let neighbor @ (y1, x1) = (y0 + dy, x0 + dx);
            if y1 < 0 || y1 == h || x1 < 0 || x1 == w || visited.contains(&(first, neighbor)) {
                continue;
            }
            let val = grid[y1 as usize][x1 as usize];
            if val != len {
                continue;
            }
            frontier.push((first, neighbor, len + 1));
        }
    }

    trailheads
}

fn part2(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|s| s.as_bytes().iter().map(|b| (b - b'0') as usize).collect())
        .collect::<Vec<Vec<_>>>();
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    let mut frontier = vec![];
    for y in 0..h {
        for x in 0..w {
            if grid[y as usize][x as usize] == 0 {
                frontier.push(vec![(y, x)]);
            }
        }
    }

    let mut visited = HashSet::new();
    let mut trails = HashSet::new();
    while let Some(path) = frontier.pop() {
        let (y0, x0) = path[path.len() - 1];
        visited.insert(path.clone());
        if path.len() == 10 {
            trails.insert(path.clone());
            continue;
        }

        for (dy, dx) in DELTAS {
            let neighbor @ (y1, x1) = (y0 + dy, x0 + dx);
            if y1 < 0 || y1 == h || x1 < 0 || x1 == w {
                continue;
            }
            let val = grid[y1 as usize][x1 as usize];
            if val != path.len() {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(neighbor);
            if !visited.contains(&new_path) {
                frontier.push(new_path);
            }
        }
    }

    trails.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 36);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 81);
    }
}
