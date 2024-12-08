use rustc_hash::FxHashSet;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut direction = (0, -1);
    let mut visited = FxHashSet::default();
    let mut obstacles = FxHashSet::default();
    let mut position = (0, 0);
    let line_length = input.lines().next().unwrap().len();
    let line_count = input.lines().count();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '^' => {
                    position = (x, y);
                }
                '#' => {
                    obstacles.insert((x, y));
                }
                _ => {}
            }
        }
    }
    visited.insert(position);

    loop {
        let next = (
            position.0 as isize + direction.0,
            position.1 as isize + direction.1,
        );
        if next.0 < 0
            || next.1 < 0
            || next.0 >= line_length as isize
            || next.1 >= line_count as isize
        {
            break;
        }

        if obstacles.contains(&(next.0 as usize, next.1 as usize)) {
            direction = (-direction.1, direction.0);
        } else {
            position = (next.0 as usize, next.1 as usize);
            visited.insert(position);
        }
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let start_direction = (0, -1);
    let mut visited = vec![];
    let mut obstacles = FxHashSet::default();
    let mut start_position = (0, 0);
    let line_length = input.lines().next().unwrap().len();
    let line_count = input.lines().count();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '^' => {
                    start_position = (x, y);
                }
                '#' => {
                    obstacles.insert((x, y));
                }
                _ => {}
            }
        }
    }

    let mut direction = start_direction;
    let mut position = start_position;
    loop {
        let next = (
            position.0 as isize + direction.0,
            position.1 as isize + direction.1,
        );
        if next.0 < 0
            || next.1 < 0
            || next.0 >= line_length as isize
            || next.1 >= line_count as isize
        {
            break;
        }

        if obstacles.contains(&(next.0 as usize, next.1 as usize)) {
            direction = (-direction.1, direction.0);
        } else {
            position = (next.0 as usize, next.1 as usize);
            visited.push((position, direction));
        }
    }

    let mut successes = 0;
    for x in 0..line_length {
        for y in 0..line_count {
            if obstacles.contains(&(x, y)) || (x, y) == start_position {
                continue;
            }
            let position = visited.iter().find(|(p, _)| p == &(x, y));
            if position.is_none() {
                continue;
            }

            let (mut position, mut direction) = position.unwrap();
            position = (
                (position.0 as isize - direction.0) as usize,
                (position.1 as isize - direction.1) as usize,
            );
            let mut visited = FxHashSet::default();

            visited.insert((position, direction));
            obstacles.insert((x, y));

            loop {
                let next = (
                    position.0 as isize + direction.0,
                    position.1 as isize + direction.1,
                );
                if next.0 < 0
                    || next.1 < 0
                    || next.0 >= line_length as isize
                    || next.1 >= line_count as isize
                {
                    obstacles.remove(&(x, y));
                    break;
                }
                if obstacles.contains(&(next.0 as usize, next.1 as usize)) {
                    direction = (-direction.1, direction.0);
                } else {
                    position = (next.0 as usize, next.1 as usize);
                    if !visited.insert((position, direction)) {
                        obstacles.remove(&(x, y));
                        successes += 1;
                        break;
                    }
                }
            }
        }
    }
    Some(successes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    #[ignore]
    fn solve_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(4606));
    }

    #[test]
    #[ignore]
    fn solve_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1703));
    }
}
