advent_of_code::solution!(3);

#[allow(unused_assignments)]
pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0u32;
    for mut i in 0..input.len() - 7 {
        if &input[i..i + 4] == "mul(" {
            let j = (i + 4..input.len())
                .find(|c| &input[*c..*c + 1] == ")")
                .unwrap();
            if let Some(index) = input[i + 4..j].find(',') {
                let (a, b) = input[i + 4..j].split_at(index);
                sum += a.parse::<u32>().unwrap_or(0) * b[1..].parse::<u32>().unwrap_or(0);
            }
            i += j;
        }
    }

    sum.into()
}

#[allow(unused_assignments)]
pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0u32;
    let mut enabled = true;
    for mut i in 0..input.len() - 7 {
        if enabled && &input[i..i + 4] == "mul(" {
            let j = (i + 4..input.len())
                .find(|c| &input[*c..*c + 1] == ")")
                .unwrap();
            if let Some(index) = input[i + 4..j].find(',') {
                let (a, b) = input[i + 4..j].split_at(index);
                sum += a.parse::<u32>().unwrap_or(0) * b[1..].parse::<u32>().unwrap_or(0);
            }
            i += j;
        } else if &input[i..i + 4] == "do()" {
            enabled = true;
            i += 4;
        } else if &input[i..i + 7] == "don't()" {
            enabled = false;
            i += 7;
        }
    }

    sum.into()
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
