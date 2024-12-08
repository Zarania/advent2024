use rustc_hash::{FxHashMap, FxHashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut antennas = FxHashMap::default();
    let mut antinodes = FxHashSet::default();
    let line_length = input.lines().next()?.len();
    let line_count = input.lines().count();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                antennas.entry(char).or_insert_with(Vec::new).push((x, y));
            }
        }
    }

    for (_, antennas) in antennas.iter() {
        for i in 0..antennas.len() - 1 {
            for j in i + 1..antennas.len() {
                let (x1, y1) = antennas[i];
                let (x2, y2) = antennas[j];
                let dx = x2 as i32 - x1 as i32;
                let dy = y2 as i32 - y1 as i32;

                let a1 = (x1 as i32 - dx, y1 as i32 - dy);
                let a2 = (x2 as i32 + dx, y2 as i32 + dy);

                if a1.0 >= 0 && a1.1 >= 0 && a1.0 < line_length as i32 && a1.1 < line_count as i32 {
                    antinodes.insert(a1);
                }

                if a2.0 >= 0 && a2.1 >= 0 && a2.0 < line_length as i32 && a2.1 < line_count as i32 {
                    antinodes.insert(a2);
                }
            }
        }
    }

    antinodes.len().try_into().ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut antennas = FxHashMap::default();
    let mut antinodes = FxHashSet::default();
    let line_length = input.lines().next()?.len();
    let line_count = input.lines().count();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                antennas.entry(char).or_insert_with(Vec::new).push((x, y));
            }
        }
    }

    for (_, antennas) in antennas.iter() {
        for i in 0..antennas.len() - 1 {
            for j in i + 1..antennas.len() {
                let (x1, y1) = antennas[i];
                let (x2, y2) = antennas[j];
                let dx = x2 as i32 - x1 as i32;
                let dy = y2 as i32 - y1 as i32;

                for i in 0.. {
                    let a1 = (x1 as i32 - (dx * i), y1 as i32 - (dy * i));
                    if a1.0 >= 0
                        && a1.1 >= 0
                        && a1.0 < line_length as i32
                        && a1.1 < line_count as i32
                    {
                        antinodes.insert(a1);
                    } else {
                        break;
                    }
                }

                for i in 0.. {
                    let a2 = (x2 as i32 + (dx * i), y2 as i32 + (dy * i));
                    if a2.0 >= 0
                        && a2.1 >= 0
                        && a2.0 < line_length as i32
                        && a2.1 < line_count as i32
                    {
                        antinodes.insert(a2);
                    } else {
                        break;
                    }
                }
            }
        }
    }

    antinodes.len().try_into().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }

    #[test]
    #[ignore]
    fn solve_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(295));
    }

    #[test]
    #[ignore]
    fn solve_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1034));
    }
}
