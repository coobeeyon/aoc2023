use itertools::Itertools;
use std::iter::once;

advent_of_code::solution!(3);

fn find_numbers(s: &str) -> Vec<(u32, (usize, usize))> {
    once('.')
        .chain(s.chars())
        .chain(once('.'))
        .tuple_windows::<(_, _)>()
        .enumerate()
        .filter_map(|(i, (this, next))| {
            if (this.is_ascii_digit() && !next.is_ascii_digit())
                || (!this.is_ascii_digit() && next.is_ascii_digit())
            {
                Some(i)
            } else {
                None
            }
        })
        .chunks(2)
        .into_iter()
        .filter_map(|chunk| chunk.collect_tuple::<(_, _)>())
        .map(|(i, j)| (s[i..j].parse::<u32>().unwrap(), (i, j)))
        .collect_vec()
}

fn find_symbols(s: &str) -> Vec<usize> {
    s.chars()
        .enumerate()
        .filter_map(|(i, c)| {
            if c != '.' && !c.is_ascii_digit() {
                Some(i)
            } else {
                None
            }
        })
        .collect_vec()
}

fn next_to_symbol(number: &(usize, (u32, (usize, usize))), symbol: &(usize, usize)) -> bool {
    let (ny, (_, (nxs, nxe))) = number;
    let (sym_y, sym_x) = symbol;
    (ny.abs_diff(*sym_y) <= 1) && (*sym_x + 1 >= *nxs) && (*sym_x <= *nxe)
}

pub fn part_one(input: &str) -> Option<u32> {
    let all_numbers = input
        .lines()
        .enumerate()
        .flat_map(|(i, s)| find_numbers(s).into_iter().map(move |l| (i, l)))
        .collect_vec();
    let all_symbols = input
        .lines()
        .enumerate()
        .flat_map(|(i, s)| find_symbols(s).into_iter().map(move |l| (i, l)))
        .collect_vec();
    let good_numbers = all_numbers
        .into_iter()
        .filter(|n| all_symbols.iter().any(|s| next_to_symbol(n, s)))
        .map(|(_, (n, _))| n)
        .collect_vec();
    let sum = good_numbers.iter().sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
