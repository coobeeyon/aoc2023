advent_of_code::solution!(9);
use std::iter::successors;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<i64> {
    let sum_diffs = input
        .lines()
        .map(|line| {
            let nums = line
                .split(' ')
                .filter_map(|s| s.parse::<i64>().ok())
                .collect_vec();
            successors(Some(nums), |last_nums| {
                dbg!(&last_nums);
                if last_nums.iter().all(|n| n == &0) {
                    None
                } else {
                    Some(
                        last_nums
                            .iter()
                            .tuple_windows()
                            .map(|(a, b)| b - a)
                            .collect_vec(),
                    )
                }
            })
            .filter_map(|i| i.last().copied())
            .sum::<i64>()
        })
        .sum::<i64>();
    Some(sum_diffs)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
