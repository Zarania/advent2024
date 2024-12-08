use std::{collections::HashMap, hash::BuildHasherDefault};

use nohash_hasher::NoHashHasher;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut first, mut second): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| line.split_once(char::is_whitespace).unwrap())
        .map(|(first, second)| {
            (
                first.parse::<u32>().unwrap(),
                second.trim().parse::<u32>().unwrap(),
            )
        })
        .unzip();

    first.sort_unstable();
    second.sort_unstable();

    first
        .iter()
        .zip(second.iter())
        .map(|(first, second)| first.abs_diff(*second))
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (first, second_temp): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| line.split_once(char::is_whitespace).unwrap())
        .map(|(first, second)| {
            (
                first.parse::<u32>().unwrap(),
                second.trim().parse::<u32>().unwrap(),
            )
        })
        .unzip();

    let mut second: HashMap<u32, u32, BuildHasherDefault<NoHashHasher<u32>>> =
        HashMap::with_capacity_and_hasher(second_temp.len(), BuildHasherDefault::default());
    second_temp.iter().for_each(|&x| {
        second.entry(x).and_modify(|e| *e += 1).or_insert(1);
    });

    first
        .iter()
        .map(|n| n * second.get(n).unwrap_or(&0))
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }

    #[test]
    #[ignore]
    fn solve_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1530215));
    }

    #[test]
    #[ignore]
    fn solve_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(26800609));
    }
}
