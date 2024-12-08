advent_of_code::solution!(7);

fn can_add_multiply(target: u64, sum: u64, parts: &[u64]) -> bool {
    if parts.is_empty() || sum > target {
        return sum == target;
    }

    if can_add_multiply(target, sum + parts[0], &parts[1..]) {
        return true;
    }

    if can_add_multiply(target, sum * parts[0], &parts[1..]) {
        return true;
    }

    false
}

fn can_add_multiply_concat(target: u64, sum: u64, parts: &[u64]) -> bool {
    if parts.is_empty() || sum > target {
        return sum == target;
    }

    if can_add_multiply_concat(target, sum + parts[0], &parts[1..]) {
        return true;
    }

    if can_add_multiply_concat(target, sum * parts[0], &parts[1..]) {
        return true;
    }

    let concat = sum * 10u64.pow(parts[0].ilog10() + 1) + parts[0];
    if can_add_multiply_concat(target, concat, &parts[1..]) {
        return true;
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(":").unwrap();
            let target = first.parse::<u64>().unwrap();
            let parts = second
                .split_whitespace()
                .map(|n| n.trim().parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            (target, parts)
        })
        .filter(|(target, parts)| can_add_multiply(*target, parts[0], &parts[1..]))
        .map(|(target, _)| target)
        .sum::<u64>()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(":").unwrap();
            let target = first.parse::<u64>().unwrap();
            let parts = second
                .split_whitespace()
                .map(|n| n.trim().parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            (target, parts)
        })
        .filter(|(target, parts)| can_add_multiply_concat(*target, parts[0], &parts[1..]))
        .map(|(target, _)| target)
        .sum::<u64>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
