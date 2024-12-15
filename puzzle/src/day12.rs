use std::{collections::HashSet, error::Error, fs::read_to_string, iter::zip, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input/day12.txt")?;
    let now = Instant::now();
    println!("p1: {} ({}µs)", part1(&input), now.elapsed().as_micros());
    let now = Instant::now();
    println!("p2: {} ({}µs)", part2(&input), now.elapsed().as_micros());
    Ok(())
}

const DELTAS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn part1(input: &str) -> usize {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    let mut frontier0 = vec![(0i32, 0i32)];
    let mut visited = HashSet::new();

    let mut sum = 0;
    while let Some(coord0 @ (y0, x0)) = frontier0.pop() {
        if visited.contains(&coord0) {
            continue;
        }

        let mut frontier1 = vec![coord0];
        let p0 = grid[y0 as usize][x0 as usize];
        let mut area = 0;
        let mut perimeter = 0;
        while let Some(coord1 @ (y1, x1)) = frontier1.pop() {
            if visited.contains(&coord1) {
                continue;
            }
            visited.insert(coord1);
            area += 1;
            for (dy, dx) in DELTAS {
                let neighbor @ (y2, x2) = (y1 + dy, x1 + dx);
                if y2 < 0 || y2 == h || x2 < 0 || x2 == w {
                    perimeter += 1;
                    continue;
                }
                let p1 = grid[y2 as usize][x2 as usize];
                if p1 == p0 {
                    frontier1.push(neighbor);
                } else {
                    perimeter += 1;
                    frontier0.push(neighbor);
                }
            }
        }
        sum += area * perimeter;
    }

    sum
}

fn part2(input: &str) -> usize {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;

    let mut frontier0 = vec![(0i32, 0i32)];
    let mut visited = HashSet::new();

    let mut sum = 0;
    while let Some(coord0 @ (y0, x0)) = frontier0.pop() {
        if visited.contains(&coord0) {
            continue;
        }

        let mut perimeters: [Vec<(i32, i32)>; 4] = Default::default();
        let mut frontier1 = vec![coord0];
        let p0 = grid[y0 as usize][x0 as usize];
        let mut area = 0;
        while let Some(coord1 @ (y1, x1)) = frontier1.pop() {
            if visited.contains(&coord1) {
                continue;
            }
            visited.insert(coord1);
            area += 1;
            for (i, (dy, dx)) in DELTAS.into_iter().enumerate() {
                let neighbor @ (y2, x2) = (y1 + dy, x1 + dx);
                if y2 < 0 || y2 == h || x2 < 0 || x2 == w {
                    perimeters[i].push(coord1);
                    continue;
                }
                let p1 = grid[y2 as usize][x2 as usize];
                if p1 == p0 {
                    frontier1.push(neighbor);
                } else {
                    perimeters[i].push(coord1);
                    frontier0.push(neighbor);
                }
            }
        }

        let mut sides = 0;
        for ((_, dx), ref mut perimeter) in zip(DELTAS, perimeters) {
            if dx == 0 {
                perimeter.sort_by(|(y0, x0), (y1, x1)| y0.cmp(y1).then_with(|| x0.cmp(x1)));
            } else {
                perimeter.sort_by(|(y0, x0), (y1, x1)| x0.cmp(x1).then_with(|| y0.cmp(y1)));
            }
            sides += 1;
            let (mut y0, mut x0) = perimeter[0];
            for &(y1, x1) in perimeter[1..].into_iter() {
                if y1.abs_diff(y0) + x1.abs_diff(x0) != 1 {
                    sides += 1;
                }
                (y0, x0) = (y1, x1);
            }
        }
        sum += area * sides;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 1930);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 1206);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            part2(
                "AAAA
BBCD
BBCC
EEEC"
            ),
            80
        );
    }

    #[test]
    fn part2_example2() {
        assert_eq!(
            part2(
                "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"
            ),
            236
        );
    }

    #[test]
    fn part2_example3() {
        assert_eq!(
            part2(
                "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
"
            ),
            368
        );
    }
}
