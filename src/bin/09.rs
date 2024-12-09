use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let mut drive = input.chars().chunks(2).into_iter().enumerate().flat_map(|(i, mut chunk)| {
        let file = chunk.next().unwrap() as u32 - '0' as u32;
        let free = chunk.next().unwrap_or('0') as u32 - '0' as u32;
        
        (0..file).map(move |_| Some(i)).chain((0..free).map(|_| None))
    }).collect::<Vec<_>>();

    let mut empty = 0;
    let mut last = drive.len() - 1;

    loop {
        empty = drive[empty..].iter().position(|&x| x.is_none()).unwrap() + empty;
        last = last.checked_sub(drive[..=last].iter().rev().position(|&x| x.is_some()).unwrap()).unwrap_or(1);

        if empty >= last {
            break;
        }
        
        drive[empty] = drive[last];
        drive[last] = None;
        last -= 1;
    }

    Some(drive.iter().enumerate().map(|(i, x)| {
        if x.is_some() {
            x.unwrap() * i
        } else {
            0
        }
    }).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut drive = input.chars().chunks(2).into_iter().enumerate().flat_map(|(i, mut chunk)| {
        let file = chunk.next().unwrap() as u32 - '0' as u32;
        let free = chunk.next().unwrap_or('0') as u32 - '0' as u32;
        
        (0..file).map(move |_| Some(i)).chain((0..free).map(|_| None))
    }).collect::<Vec<_>>();

    let mut last = drive.len() - 1;

    loop {
        last = last.checked_sub(drive[..=last].iter().rev().position(|&x| x.is_some()).unwrap()).unwrap_or(0);
        
        if last == 0 {
            break;
        }

        let file = drive[last];
        let file_size = drive[..last].iter().rev().take_while(|&&x| x.is_some() && x.unwrap() == file.unwrap()).count();

        
        for i in 0..(last - file_size) {
            if drive[i..=i + file_size].iter().all(|&x| x.is_none()) {
                drive[i..=i + file_size].iter_mut().for_each(|x| *x = file);
                drive[last - file_size..=last].iter_mut().for_each(|x| *x = None);
                break;
            }
        }
        
        last = last.checked_sub(file_size + 1).unwrap_or(0);
    }


    Some(drive.iter().enumerate().map(|(i, x)| {
        if x.is_some() {
            x.unwrap() * i
        } else {
            0
        }
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<usize> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }

    #[test]
    #[ignore]
    fn solve_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6398608069280));
    }

    #[test]
    #[ignore]
    fn solve_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6427437134372));
    }
}
