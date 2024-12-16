use rustc_hash::FxHashSet;

advent_of_code::solution!(10);

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn path_length(
    topological: &Vec<Vec<u32>>,
    cache: &mut FxHashSet<[(usize, usize); 2]>,
    current: (usize, usize),
    start: (usize, usize),
) -> () {
    if topological[current.1][current.0] == 9 {
        cache.insert([current, start]);
        return;
    }

    for direction in DIRECTIONS.iter() {
        let next = (
            current.0 as i32 + direction.0,
            current.1 as i32 + direction.1,
        );
        if next.0 >= 0
            && next.0 < topological[0].len() as i32
            && next.1 >= 0
            && next.1 < topological.len() as i32
            && topological[next.1 as usize][next.0 as usize]
                == topological[current.1][current.0] + 1
        {
            path_length(topological, cache, (next.0 as usize, next.1 as usize), start);
        }
    }
}

fn path_length2(
    topological: &Vec<Vec<u32>>,
    cache: &mut Vec<Vec<i32>>,
    current: (usize, usize),
) -> i32 {
    if cache[current.1][current.0] > 0 {
        return cache[current.1][current.0];
    }

    if topological[current.1][current.0] == 9 {
        return 1;
    }

    let mut paths = 0;
    for direction in DIRECTIONS.iter() {
        let next = (
            current.0 as i32 + direction.0,
            current.1 as i32 + direction.1,
        );
        if next.0 >= 0
            && next.0 < topological[0].len() as i32
            && next.1 >= 0
            && next.1 < topological.len() as i32
            && topological[next.1 as usize][next.0 as usize]
                == topological[current.1][current.0] + 1
        {
            paths += path_length2(topological, cache, (next.0 as usize, next.1 as usize));
        }
    }
    cache[current.1][current.0] = paths;
    paths
}

pub fn part_one(input: &str) -> Option<u32> {
    let topological = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c as u32 - '0' as u32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut cache = FxHashSet::default();
    topological
        .iter()
        .enumerate()
        .for_each(|(y, row)| {
            row.iter()
            .enumerate()
            .filter(|(_, cell)| **cell == 0)
                .for_each(|(x, _)| {
                    path_length(&topological, &mut cache, (x, y), (x, y));
                });
        });
        
    Some(cache.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let topological = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c as u32 - '0' as u32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut cache = vec![vec![0; topological[0].len()]; topological.len()];

    let total = topological
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, cell)| {
                    if *cell == 0 {
                        path_length2(&topological, &mut cache, (x, y))
                    } else {
                        0
                    }
                })
                .sum::<i32>() as u32
        })
        .sum::<u32>()
        .into();

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }

    #[test]
    #[ignore]
    fn solve_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(794));
    }

    #[test]
    #[ignore]
    fn solve_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1706));
    }
}
