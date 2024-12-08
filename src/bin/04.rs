advent_of_code::solution!(4);

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

fn get_char(input: &str, position: usize, offset: isize) -> char {
    let position = position as isize + offset;
    if position >= 0 && position < input.len() as isize {
        input[position as usize..(position + 1) as usize]
            .chars()
            .next()
            .unwrap()
    } else {
        '.'
    }
}

fn is_x_mas(input: &str, position: usize, offset: isize) -> bool {
    let up_left = get_char(input, position, offset);
    let down_right = get_char(input, position, -offset);
    if !((up_left == 'M' && down_right == 'S') || (up_left == 'S' && down_right == 'M')) {
        return false;
    }

    let up_right = get_char(input, position, offset + 2);
    let down_left = get_char(input, position, -(offset + 2));

    (up_right == 'M' && down_left == 'S') || (up_right == 'S' && down_left == 'M')
}

pub fn part_one(input: &str) -> Option<u32> {
    //+1 to include the newline character
    let line_length = (input.lines().next()?.len() + 1) as isize;
    let offsets: Vec<isize> = vec![
        -line_length - 1,
        -line_length,
        -line_length + 1,
        -1,
        1,
        line_length - 1,
        line_length,
        line_length + 1,
    ];

    input
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == 'X')
        .map(|(i, _)| {
            offsets
                .iter()
                .filter(|&&offset| {
                    //check 'S' first to skip early if it's out of bounds
                    XMAS.iter()
                        .enumerate()
                        .rev()
                        .all(|(j, c)| get_char(input, i, offset * j as isize) == *c)
                })
                .count() as u32
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let line_length = (input.lines().next()?.len() + 1) as isize;

    Some(
        input
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == 'A')
            .filter(|(i, _)| is_x_mas(input, *i, line_length - 1))
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }

    #[test]
    #[ignore]
    fn solve_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2336));
    }

    #[test]
    #[ignore]
    fn solve_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1831));
    }
}
