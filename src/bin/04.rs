advent_of_code::solution!(4);

fn get_char(input: &str, position: usize, offset: isize) -> char {
    let position = position as isize + offset;
    if position >= 0 && position < input.len() as isize {
        input[position as usize..(position + 1) as usize].chars().next().unwrap()
    } else {
        '.'
    }
}

fn is_x_mas(input: &str, position: usize, offset: isize) -> bool {
    let up_left = get_char(input, position, offset);
    let down_right = get_char(input, position, offset * -1);
    if !((up_left == 'M' && down_right == 'S') || (up_left == 'S' && down_right == 'M')) {
        return false;
    }

    let up_right = get_char(input, position, offset + 2);
    let down_left = get_char(input, position, (offset + 2) * -1);

    (up_right == 'M' && down_left == 'S') || (up_right == 'S' && down_left == 'M')
}

pub fn part_one(input: &str) -> Option<u32> {
    let line_length = (input.lines().next()?.len() + 1) as isize;
    let mut sum = 0;
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

    for (i, char) in input.chars().enumerate() {
        if char == 'X' {
            for offset in offsets.iter() {
                if get_char(input, i, *offset) == 'M'
                    && get_char(input, i, offset * 2) == 'A'
                    && get_char(input, i, offset * 3) == 'S'
                {
                    sum += 1;
                }
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let line_length = (input.lines().next()?.len() + 1) as isize;
    let mut sum = 0;

    for (i, char) in input.chars().enumerate() {
        if char == 'A' {
            if is_x_mas(input, i, line_length - 1) {
                sum += 1;
            }
        }
    }

    Some(sum)
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
    #[ignore]
    fn solve_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2336));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }

    #[test]
    #[ignore]
    fn solve_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1831));
    }
}
