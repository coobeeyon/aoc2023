use std::collections::HashMap;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect_vec();
    let directions = lines[0].chars().collect_vec();
    let nodes = lines[2..]
        .iter()
        .map(|s| {
            (
                s[0..3].to_string(),
                (s[7..10].to_string(), s[12..15].to_string()),
            )
        })
        .collect::<HashMap<_, _>>();
    Some(
        directions
            .iter()
            .cycle()
            .fold_while(("AAA", 0), |(node, steps), dir| {
                if node == "ZZZ" {
                    Done((node, steps))
                } else {
                    Continue(if dir == &'L' {
                        (&nodes[node].0, steps + 1)
                    } else {
                        (&nodes[node].1, steps + 1)
                    })
                }
            })
            .into_inner()
            .1,
    )
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
