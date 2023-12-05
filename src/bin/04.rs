use std::collections::HashSet;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(|l| l.split('|'))
        .map(|v| {
            v.map(|s| {
                s.split(' ')
                    .filter_map(|ns| ns.parse::<u32>().ok())
                    .collect::<HashSet<u32>>()
            })
            .reduce(|a, s| a.intersection(&s).cloned().collect::<HashSet<_>>())
            .unwrap_or(HashSet::new())
        })
        .map(|h| {
            let num_matching = h.len() as u32;
            if num_matching > 0 {
                2u32.pow(num_matching - 1)
            } else {
                0
            }
        })
        .sum();
    Some(sum)
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
