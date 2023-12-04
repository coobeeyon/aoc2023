use std::{cmp::max, collections::HashMap};

use itertools::Itertools;
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input
        .lines()
        .map(|s| {
            // remove head of the string until the first space after the first colon
            let colon_i = s.find(':').unwrap();
            s[colon_i + 2..].to_string()
        })
        .collect_vec();
    let bags = lines
        .into_iter()
        .map(|line| {
            line.split(';')
                .map(|s| {
                    s.split(',')
                        .map(|s| {
                            let field = s.trim().split(' ').collect_vec();
                            (field[1].to_string(), field[0].parse::<u32>().unwrap())
                        })
                        .collect::<HashMap<String, u32>>()
                })
                .collect_vec()
        })
        .collect_vec();
    let valid_draws = bags
        .into_iter()
        .map(|bag| {
            bag.into_iter().all(|draw| {
                (draw.get("red") <= Some(&12))
                    && (draw.get("green") <= Some(&13))
                    && (draw.get("blue") <= Some(&14))
            })
        })
        .collect_vec();
    Some(
        valid_draws
            .into_iter()
            .enumerate()
            .filter(|(_, b)| *b)
            .fold(0, |acc, (i, _)| acc + (i + 1) as u32),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input
        .lines()
        .map(|s| {
            // remove head of the string until the first space after the first colon
            let colon_i = s.find(':').unwrap();
            s[colon_i + 2..].to_string()
        })
        .collect_vec();
    let bags = lines
        .into_iter()
        .map(|line| {
            line.split(';')
                .map(|s| {
                    s.split(',')
                        .map(|s| {
                            let field = s.trim().split(' ').collect_vec();
                            (field[1].to_string(), field[0].parse::<u32>().unwrap())
                        })
                        .collect::<HashMap<String, u32>>()
                })
                .collect_vec()
        })
        .collect_vec();
    let fewest = bags
        .into_iter()
        .map(|bag| {
            bag.into_iter()
                .fold(
                    HashMap::from([("red", 0u32), ("green", 0), ("blue", 0)]),
                    |acc, draw| {
                        HashMap::from(
                            ["red", "green", "blue"]
                                .map(|c| (c, max(acc[c], *draw.get(c).unwrap_or(&0)))),
                        )
                    },
                )
                .values()
                .product::<u32>()
        })
        .sum();
    Some(fewest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
