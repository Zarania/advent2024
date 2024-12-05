use std::{cmp::Ordering, collections::HashMap, hash::BuildHasherDefault};

use nohash_hasher::NoHashHasher;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (first, second) = input.split_once("\n\n").unwrap();
    let rules_vec = first
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("|").unwrap();
            let a = a.parse::<u32>().unwrap();
            let b = b.parse::<u32>().unwrap();
            (a, b)
        })
        .collect::<Vec<_>>();

    let mut rules: HashMap<u32, Vec<u32>, BuildHasherDefault<NoHashHasher<u32>>> =
        HashMap::with_capacity_and_hasher(rules_vec.len(), BuildHasherDefault::default());
    rules_vec.iter().for_each(|&(a, b)| {
        rules
            .entry(a)
            .and_modify(|e| e.push(b))
            .or_insert_with(|| vec![b]);
    });

    second
        .lines()
        .map(|line| {
            let pages = line
                .split(",")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            if pages.iter().enumerate().any(|(i, &page)| {
                if let Some(rule) = rules.get(&page)
                {
                    pages[..i].iter().any(|&p| rule.contains(&p))
                } else {
                    false
                }
            }) {
                0
            } else {
                pages[pages.len() / 2]
            }
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (first, second) = input.split_once("\n\n").unwrap();
    let rules_vec = first
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("|").unwrap();
            let a = a.parse::<u32>().unwrap();
            let b = b.parse::<u32>().unwrap();
            (a, b)
        })
        .collect::<Vec<_>>();

    let mut rules: HashMap<u32, Vec<u32>, BuildHasherDefault<NoHashHasher<u32>>> =
        HashMap::with_capacity_and_hasher(rules_vec.len(), BuildHasherDefault::default());
    rules_vec.iter().for_each(|&(a, b)| {
        rules
            .entry(a)
            .and_modify(|e| e.push(b))
            .or_insert_with(|| vec![b]);
    });

    second
        .lines()
        .map(|line| {
            let mut pages = line
                .split(",")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            if pages.iter().enumerate().any(|(i, &page)| {
                if let Some(rule) = rules.get(&page)
                {
                    pages[..i].iter().any(|&p| rule.contains(&p))
                } else {
                    false
                }
            }) {
                pages.sort_by(|a, b| {
                    if let Some(rule) = rules.get(&a) {
                        if rule.contains(b) {
                            return Ordering::Less;
                        }
                    }

                    if let Some(rule) = rules.get(&b) {
                        if rule.contains(a) {
                            return Ordering::Greater;
                        }
                    }

                    Ordering::Equal
                });
                pages[pages.len() / 2]
            } else {
                0
            }
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
}
