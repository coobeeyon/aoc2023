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

fn next_node<'a>(
    node_map: &'a HashMap<String, (String, String)>,
    node: &'a str,
    dir: &'a char,
) -> &'a str {
    if dir == &'L' {
        &node_map[node].0
    } else {
        &node_map[node].1
    }
}

fn get_path<'a>(
    node_map: &'a HashMap<String, (String, String)>,
    directions: &'a [char],
    start_node: &'a str,
) -> impl Iterator<Item = (&'a str, usize)> {
    directions
        .iter()
        .enumerate()
        .cycle()
        .scan(start_node, |node, (dir_step, dir)| {
            let next_node = next_node(node_map, node, dir);
            let ret: &str = node;
            *node = next_node;
            Some((ret, dir_step))
        })
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines().collect_vec();
    let directions = lines[0].chars().collect_vec();
    let node_map = lines[2..]
        .iter()
        .map(|s| {
            (
                s[0..3].to_string(),
                (s[7..10].to_string(), s[12..15].to_string()),
            )
        })
        .collect::<HashMap<_, _>>();
    let start_nodes = node_map.keys().filter(|s| s.ends_with('A')).collect_vec();
    let ret = start_nodes
        .iter()
        .map(|s| {
            let slow = get_path(&node_map, &directions, s);
            let fast = get_path(&node_map, &directions, s).skip(1).step_by(2);
            let collision = slow.zip(fast).position(|(a, b)| a == b).unwrap();
            let collision_element = get_path(&node_map, &directions, s).nth(collision).unwrap();
            let cycle_length = get_path(&node_map, &directions, s)
                .skip(collision + 1)
                .position(|n| n == collision_element)
                .unwrap()
                + 1;
            cycle_length
        })
        .fold(1, |a, b| {
            let gcd = num::integer::gcd(a, b);
            a * b / gcd
        });
    Some(ret as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
