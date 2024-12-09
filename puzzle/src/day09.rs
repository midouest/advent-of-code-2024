use std::{error::Error, fs::read_to_string, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input/day09.txt")?;
    let now = Instant::now();
    println!("p1: {} ({}µs)", part1(&input), now.elapsed().as_micros());
    let now = Instant::now();
    println!("p2: {} ({}µs)", part2(&input), now.elapsed().as_micros());
    Ok(())
}

const OFFSET: u8 = 48;

fn part1(input: &str) -> usize {
    let bytes = input.trim_ascii_end().as_bytes();
    let mut head = 0;
    let mut tail = bytes.len() - 1;
    let mut count = 0;
    let mut sum = 0;
    let mut partial_len = 0;
    let mut partial_id = 0;
    while head < tail {
        let len = bytes[head] - OFFSET;
        let id = head / 2;
        for _ in 0..len {
            sum += count * id;
            count += 1;
        }
        head += 1;
        let mut empty_len = bytes[head] - OFFSET;
        while empty_len > 0 {
            if partial_len == 0 {
                partial_len = bytes[tail] - OFFSET;
                partial_id = tail / 2;
            }
            let len = empty_len.min(partial_len);
            for _ in 0..len {
                sum += count * partial_id;
                count += 1;
            }
            empty_len -= len;
            partial_len -= len;
            if partial_len == 0 {
                tail -= 2;
            }
        }
        head += 1;
    }
    for _ in 0..partial_len {
        sum += count * partial_id;
        count += 1;
    }
    sum
}

enum Node {
    Empty { len: u8 },
    File { len: u8, id: usize },
}

fn part2(input: &str) -> usize {
    let mut disk = input
        .trim_ascii_end()
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(i, &b)| {
            let len = b - OFFSET;
            if i % 2 == 0 {
                Node::File { len, id: i / 2 }
            } else {
                Node::Empty { len }
            }
        })
        .collect::<Vec<_>>();

    let mut i = disk.len() - 1;
    while i > 0 {
        if let Node::File { len: a, id } = disk[i] {
            for j in 0..i {
                match disk[j] {
                    Node::Empty { len: b } if b >= a => {
                        disk[i] = Node::Empty { len: a };
                        disk[j] = Node::File { len: a, id };
                        if b > a {
                            disk.insert(j + 1, Node::Empty { len: b - a });
                        }
                        break;
                    }
                    _ => (),
                }
            }
        }
        i -= 1;
    }

    let mut sum = 0usize;
    let mut count = 0usize;
    for node in disk {
        match node {
            Node::Empty { len } => {
                count += len as usize;
            }
            Node::File { len, id } => {
                for _ in 0..len {
                    sum += count * id;
                    count += 1;
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 2858);
    }
}
