use std::collections::HashMap;

use itertools::Itertools;
advent_of_code::solution!(1);

fn sum_lines(lines: Vec<String>) -> Option<u32> {
    let digitlines = lines
        .iter()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c.to_digit(10).unwrap())
                .collect_vec()
        })
        .collect_vec();
    let sums = digitlines
        .into_iter()
        .map(|digits| digits.first().unwrap() * 10 + digits.last().unwrap());
    Some(sums.sum())
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().map(|s| s.to_string()).collect_vec();
    sum_lines(lines)
}

fn line_to_number(line: String) -> usize {
    let patterns = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    let lefts = patterns
        .iter()
        .filter_map(|(k, v)| line.find(k).map(|i| (i, *v)))
        .collect_vec();
    let rights = patterns
        .iter()
        .filter_map(|(k, v)| line.rfind(k).map(|i| (i, *v)))
        .collect_vec();
    let left = lefts.iter().min_by_key(|(i, _)| i).map(|(_, v)| v).unwrap();
    let right = rights
        .iter()
        .max_by_key(|(i, _)| i)
        .map(|(_, v)| v)
        .unwrap();
    10 * left + right
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines = input.lines().map(|s| s.to_string()).collect_vec();
    let line_sums = lines.iter().map(|line| line_to_number(line.to_string()));
    Some(line_sums.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(198));
    }
}
