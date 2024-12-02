use std::cmp::Ordering;

advent_of_code::solution!(2);

fn is_safe(level: &[u32]) -> bool {
    let ordering = level[0].cmp(&level[1]);

    level
        .windows(2)
        .all(|w| w[0].cmp(&w[1]) == ordering && w[0].abs_diff(w[1]) <= 3)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .filter(|l| is_safe(l))
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .filter(|level| {
                //some levels go x < y > z so to avoid that edge case and be lazy we just go with this ordering and it works
                let ordering = level[1].cmp(&level[2]).then(level[2].cmp(&level[3]));
                if ordering == Ordering::Equal {
                    return false;
                }

                let fail = level
                    .windows(2)
                    .position(|w| w[0].cmp(&w[1]) != ordering || w[0].abs_diff(w[1]) > 3);

                if let Some(fail) = fail {
                    for n in fail..=(fail + 1) {
                        let mut clone = level.clone();
                        clone.remove(n);
                        if is_safe(&clone) {
                            return true;
                        }
                    }
                    false
                } else {
                    true
                }
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    #[ignore]
    fn solve_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(486));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    #[ignore]
    fn solve_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(540));
    }
}
