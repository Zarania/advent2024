use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let multiplications = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    multiplications.captures_iter(input).map(|cap| cap.extract()).map(|(_, [a, b])| {
        a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap()
    }).sum::<u32>().into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut enabled = true;
    let multiplications = Regex::new(r"mul\((\d+),(\d+)\)|do(n't)?\(\)").unwrap();
    multiplications.captures_iter(input).map(|cap| {
        match cap.get(0).unwrap().as_str() {
            "do()" => {enabled = true; 0},
            "don't()" => {enabled = false; 0},
            _ => if enabled {
                cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap()
            } else {0},
        }
    }).sum::<u32>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    #[ignore]
    fn solve_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(175615763));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }

    #[test]
    #[ignore]
    fn solve_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(74361272));
    }
}
