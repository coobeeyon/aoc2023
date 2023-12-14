use std::{collections::HashMap, iter::once};

use itertools::{repeat_n, Itertools};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let card_ranks: HashMap<char, u8> = "23456789TJQKA"
        .chars()
        .enumerate()
        .map(|(i, v)| (v, i as u8))
        .collect();
    let hands = input
        .lines()
        .filter_map(|s| s.split(' ').collect_tuple::<(_, _)>())
        .map(|(cards, bid)| (cards, bid.parse::<u32>().unwrap()))
        .collect_vec();
    let ranked_bids = hands
        .iter()
        .map(|(cards, bid)| {
            (
                cards
                    .chars()
                    .sorted()
                    .map(|c| (c, 1))
                    .coalesce(|(prev_c, prev_n), (cur_c, cur_n)| {
                        if prev_c == cur_c {
                            Ok((cur_c, prev_n + cur_n))
                        } else {
                            Err(((prev_c, prev_n), (cur_c, cur_n)))
                        }
                    })
                    .map(|t| t.1)
                    .sorted()
                    .rev()
                    .collect_vec(),
                cards.chars().map(|c| card_ranks[&c]).collect_vec(),
                bid,
            )
        })
        .sorted_by(|a, b| {
            let sa = &a.0;
            let sb = &b.0;
            (sa[0], sa.get(1).unwrap_or(&-1), &a.1).cmp(&(sb[0], sb.get(1).unwrap_or(&-1), &b.1))
        })
        .map(|(_, _, bid)| bid)
        .collect_vec();
    dbg!(&ranked_bids);
    Some(
        ranked_bids
            .iter()
            .enumerate()
            .inspect(|e| {
                dbg!(e);
            })
            .map(|(i, bid)| (i as u32 + 1) * *bid)
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let card_ranks: HashMap<char, u8> = "J23456789TQKA"
        .chars()
        .enumerate()
        .map(|(i, v)| (v, i as u8))
        .collect();
    let hands = input
        .lines()
        .filter_map(|s| s.split(' ').collect_tuple::<(_, _)>())
        .map(|(cards, bid)| (cards, bid.parse::<u32>().unwrap()))
        .collect_vec();
    let ranked_bids = hands
        .iter()
        .map(|(cards, bid)| {
            (
                cards
                    .chars()
                    .filter(|c| *c != 'J')
                    .sorted()
                    .map(|c| (c, 1))
                    .coalesce(|(prev_c, prev_n), (cur_c, cur_n)| {
                        if prev_c == cur_c {
                            Ok((cur_c, prev_n + cur_n))
                        } else {
                            Err(((prev_c, prev_n), (cur_c, cur_n)))
                        }
                    })
                    .map(|t| t.1)
                    .sorted()
                    .rev()
                    .chain(repeat_n(0, 5))
                    .zip(once(cards.chars().filter(|c| *c == 'J').count()).chain(repeat_n(0, 5)))
                    .map(|(a, b)| a + b)
                    .collect_vec(),
                cards.chars().map(|c| card_ranks[&c]).collect_vec(),
                bid,
            )
        })
        .sorted_by(|a, b| {
            let sa = &a.0;
            let sb = &b.0;
            (sa[0], sa.get(1).unwrap_or(&0), &a.1).cmp(&(sb[0], sb.get(1).unwrap_or(&0), &b.1))
        })
        .map(|(_, _, bid)| bid)
        .collect_vec();
    Some(
        ranked_bids
            .iter()
            .enumerate()
            .map(|(i, bid)| (i as u32 + 1) * *bid)
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
