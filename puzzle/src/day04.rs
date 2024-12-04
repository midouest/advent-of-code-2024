use std::{error::Error, fs::read_to_string, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input/day04.txt")?;
    let now = Instant::now();
    println!("p1: {} ({}us)", part1(&input), now.elapsed().as_micros());
    let now = Instant::now();
    println!("p2: {} ({}us)", part2(&input), now.elapsed().as_micros());
    Ok(())
}

const DELTAS: [(i32, i32); 8] = [
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
];

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

fn part1(input: &str) -> usize {
    let search = input
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let (height, width) = (search.len() as i32, search[0].len() as i32);

    let found = (0..height)
        .flat_map(|y| (0..width).map(move |x| (y, x)))
        .map(|(y0, x0)| {
            DELTAS
                .iter()
                .filter(|(dy, dx)| {
                    (*dy >= 0 || y0 >= 3)
                        && (*dy <= 0 || y0 < height - 3)
                        && (*dx >= 0 || x0 >= 3)
                        && (*dx <= 0 || x0 < width - 3)
                })
                .filter(|(dy, dx)| {
                    XMAS.iter()
                        .scan((y0, x0), |(y, x), c| {
                            let next = (*y, *x, *c);
                            (*y, *x) = (*y + dy, *x + dx);
                            Some(next)
                        })
                        .all(|(y, x, c)| search[y as usize][x as usize] == c)
                })
                .count()
        })
        .sum::<usize>();

    found
}

fn part2(input: &str) -> usize {
    let search = input
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let (height, width) = (search.len(), search[0].len());

    let mut count = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if search[y][x] == 'A'
                && ((search[y - 1][x - 1] == 'M' && search[y + 1][x + 1] == 'S')
                    || (search[y - 1][x - 1] == 'S' && search[y + 1][x + 1] == 'M'))
                && ((search[y - 1][x + 1] == 'M' && search[y + 1][x - 1] == 'S')
                    || (search[y - 1][x + 1] == 'S' && search[y + 1][x - 1] == 'M'))
            {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 9);
    }
}
