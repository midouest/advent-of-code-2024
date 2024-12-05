use std::{collections::HashMap, error::Error, fs::read_to_string, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input/day05.txt")?;
    let now = Instant::now();
    println!("p1: {} ({}us)", part1(&input)?, now.elapsed().as_micros());
    let now = Instant::now();
    println!("p2: {} ({}us)", part2(&input)?, now.elapsed().as_micros());
    Ok(())
}

fn parse(input: &str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let sections = input.split("\n\n").collect::<Vec<_>>();
    let rules = sections[0]
        .lines()
        .map(|line| {
            line.split("|")
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .fold(HashMap::new(), |mut acc, rule| {
            let (before, after) = (rule[0], rule[1]);
            acc.entry(after).or_insert(Vec::new()).push(before);
            acc
        });
    let updates = sections[1]
        .lines()
        .map(|line| {
            line.split(",")
                .filter_map(|s| s.parse::<usize>().ok())
                .collect()
        })
        .collect();
    (rules, updates)
}

fn out_of_order(update: &[usize], rules: &HashMap<usize, Vec<usize>>) -> Option<(usize, usize)> {
    update
        .iter()
        .enumerate()
        .filter_map(|(i, page)| {
            rules
                .get(page)
                .and_then(|befores| {
                    befores
                        .iter()
                        .filter_map(|before| {
                            update
                                .iter()
                                .position(|p| p == before)
                                .and_then(|j| (j > i).then_some(j))
                        })
                        .max()
                })
                .map(|j| (i, j))
        })
        .next()
}

fn reorder(update: &[usize], rules: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut fixed = update.to_vec();
    let mut i = 0;
    loop {
        let Some((j, k)) = out_of_order(&fixed[i..], rules) else {
            break fixed;
        };
        let p = fixed.remove(i + j);
        fixed.insert(i + k, p);
        i += j;
    }
}

fn in_right_order(update: &[usize], rules: &HashMap<usize, Vec<usize>>) -> bool {
    update.iter().enumerate().all(|(i, page)| {
        rules
            .get(page)
            .map(|befores| {
                befores
                    .iter()
                    .filter_map(|before| update.iter().position(|p| p == before))
                    .all(|j| j < i)
            })
            .unwrap_or(true)
    })
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let (rules, updates) = parse(input);
    let sum = updates
        .into_iter()
        .filter(|update| in_right_order(update, &rules))
        .map(|update| update[update.len() / 2])
        .sum();
    Ok(sum)
}

fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    let (rules, updates) = parse(input);
    let sum = updates
        .into_iter()
        .filter(|update| !in_right_order(update, &rules))
        .map(|update| reorder(&update, &rules))
        .map(|update| update[update.len() / 2])
        .sum();
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part1_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part1(EXAMPLE)?, 143);
        Ok(())
    }

    #[test]
    fn part2_example() -> Result<(), Box<dyn Error>> {
        assert_eq!(part2(EXAMPLE)?, 123);
        Ok(())
    }
}
