use std::{cmp::Ordering, collections::HashSet};

advent_of_code::solution!(5);

fn parse_input(input: &str) -> (HashSet<(u32, u32)>, &str) {
    let (first, second) = input.split_once("\n\n").unwrap();
    let rules = first
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("|").unwrap();
            (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
        })
        .collect::<HashSet<_>>();

    (rules, second)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, second) = parse_input(input);

    second
        .lines()
        .map(|line| {
            line.split(",")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|pages| pages.is_sorted_by(|a, b| rules.contains(&(*a, *b))))
        .map(|pages| pages[pages.len() / 2])
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, second) = parse_input(input);

    second
        .lines()
        .map(|line| {
            line.split(",")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|pages| !pages.is_sorted_by(|a, b| rules.contains(&(*a, *b))))
        .map(|mut pages| {
            pages.sort_unstable_by(|a, b| {
                if rules.contains(&(*a, *b)) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            pages[pages.len() / 2]
        })
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }

    #[test]
    #[ignore]
    fn solve_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(5275));
    }

    #[test]
    #[ignore]
    fn solve_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6191));
    }
}
