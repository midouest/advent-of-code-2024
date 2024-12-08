use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::read_to_string,
    time::Instant,
};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input/day08.txt")?;
    let now = Instant::now();
    println!("p1: {} ({}µs)", part1(&input), now.elapsed().as_micros());
    let now = Instant::now();
    println!("p2: {} ({}µs)", part2(&input), now.elapsed().as_micros());
    Ok(())
}

fn parse(input: &str) -> (i32, i32, HashMap<u8, Vec<(i32, i32)>>) {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let mut antennas = HashMap::new();
    for y in 0..height {
        for x in 0..width {
            let a = grid[y as usize][x as usize];
            if a != b'.' {
                antennas.entry(a).or_insert(vec![]).push((y, x));
            }
        }
    }
    (height, width, antennas)
}

fn part1(input: &str) -> usize {
    let (height, width, antennas) = parse(input);
    let mut antinodes = HashSet::new();
    for coords in antennas.values() {
        for (i, &(y0, x0)) in coords.iter().enumerate() {
            for &(y1, x1) in coords[i + 1..].iter() {
                let dy = y1 - y0;
                let dx = x1 - x0;
                for (y2, x2) in [(y1 + dy, x1 + dx), (y0 - dy, x0 - dx)] {
                    if y2 >= 0 && y2 < height && x2 >= 0 && x2 < width {
                        antinodes.insert((y2, x2));
                    }
                }
            }
        }
    }
    antinodes.len()
}

fn part2(input: &str) -> usize {
    let (height, width, antennas) = parse(input);
    let mut antinodes = HashSet::new();
    for coords in antennas.values() {
        for (i, &(y0, x0)) in coords.iter().enumerate() {
            for &(y1, x1) in coords[i + 1..].iter() {
                let dy = y1 - y0;
                let dx = x1 - x0;
                let (mut y2, mut x2) = (y0, x0);
                while y2 >= 0 && y2 < height && x2 >= 0 && x2 < width {
                    antinodes.insert((y2, x2));
                    y2 -= dy;
                    x2 -= dx;
                }
                (y2, x2) = (y1, x1);
                while y2 >= 0 && y2 < height && x2 >= 0 && x2 < width {
                    antinodes.insert((y2, x2));
                    y2 += dy;
                    x2 += dx;
                }
            }
        }
    }
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 34);
    }
}
