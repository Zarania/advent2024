use rustc_hash::FxHashMap;

advent_of_code::solution!(11);

fn length(number: usize) -> u32 {
    if number == 0 {
        1
    } else {
        number.ilog10() + 1
    }
}

fn solve_stones(stones: &mut FxHashMap<usize, usize>, iterations: usize) -> usize {
    for _ in 0..iterations {
        let mut results = FxHashMap::default();
        
        for (key, val) in &mut *stones {
            let length = length(*key);
            match key {
                0 => {
                    *results.entry(1).or_default() += *val;
                },
                x if length % 2 == 0 => {
                    let divider = 10usize.pow(length / 2);
                    let left = x / divider;
                    let right = x % divider;
                    *results.entry(left).or_default() += *val;
                    *results.entry(right).or_default() += *val;
                }
                _ => {
                    *results.entry(key * 2024).or_default() += *val;
                }
            }
        }
        *stones = results;
    }
    stones.values().sum::<usize>()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut stones = FxHashMap::default();
    for stone in input.lines().next().unwrap().split_whitespace().map(|n| n.parse::<usize>().unwrap()) {
        *stones.entry(stone).or_default() += 1;
    }

    Some(solve_stones(&mut stones, 25))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut stones = FxHashMap::default();
    for stone in input.lines().next().unwrap().split_whitespace().map(|n| n.parse::<usize>().unwrap()) {
        *stones.entry(stone).or_default() += 1;
    }
    
    Some(solve_stones(&mut stones, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }

    #[test]
    #[ignore]
    fn solve_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(185205));
    }

    #[test]
    #[ignore]
    fn solve_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(221280540398419));
    }
}
