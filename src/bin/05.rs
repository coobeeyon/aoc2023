use std::ops::Range;

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let chunks = input
        .lines()
        .group_by(|s| s != &"")
        .into_iter()
        .map(|chunk| chunk.1.collect_vec())
        .filter(|v| v.len() > 1 || !v[0].is_empty())
        .collect_vec();
    let seeds = chunks[0][0]
        .split(' ')
        .filter_map(|s| s.parse::<u64>().ok())
        .collect_vec();
    let translation = chunks
        .into_iter()
        .skip(1)
        .map(|chunk| {
            chunk
                .into_iter()
                .skip(1)
                .filter_map(|s| {
                    s.split(' ')
                        .filter_map(|s| s.parse::<u64>().ok())
                        .collect_tuple::<(_, _, _)>()
                })
                .collect_vec()
        })
        .collect_vec();
    let locations = seeds
        .into_iter()
        .map(|n| {
            translation
                .iter()
                .map(|v| v.as_slice())
                .fold(n, translate_stage)
        })
        .collect_vec();
    locations.into_iter().min()
}

fn translate_stage(n: u64, layer: &[(u64, u64, u64)]) -> u64 {
    if let Some((dst, src, _)) = layer
        .iter()
        .find(|(_, src, len)| n >= *src && n < len + src)
    {
        n - src + dst
    } else {
        n
    }
}

fn translate_stage_range(
    range: Range<u64>,
    layer: &(u64, u64, u64),
) -> Vec<(Range<u64>, Option<i64>)> {
    let (dst, src, len) = *layer;
    if (range.end <= src) || (range.start >= src + len) {
        return vec![(range, None)];
    }
    let cutrange = src..src + len;
    let left_cut = range.start < cutrange.start;
    let right_cut = range.end > cutrange.end;
    let offset = Some(dst as i64 - src as i64);
    if left_cut && right_cut {
        vec![
            (range.start..cutrange.start, None),
            (cutrange.clone(), offset),
            (cutrange.end..range.end, None),
        ]
    } else if left_cut {
        vec![
            (range.start..cutrange.start, None),
            (cutrange.start..range.end, offset),
        ]
    } else if right_cut {
        vec![
            (range.start..cutrange.end, offset),
            (cutrange.end..range.end, None),
        ]
    } else {
        vec![(range, offset)]
    }
}

fn translate_stage_ranges(r: Range<u64>, layer: &[(u64, u64, u64)]) -> Vec<Range<u64>> {
    let src_ranges = layer.iter().fold(vec![(r, None)], |a, l| {
        a.into_iter()
            .flat_map(|(r, offset)| {
                if offset.is_none() {
                    translate_stage_range(r, l)
                } else {
                    vec![(r, offset)]
                }
            })
            .collect_vec()
    });
    src_ranges
        .iter()
        .map(|(r, offset)| {
            let offset = offset.unwrap_or(0);
            (r.start as i64 + offset) as u64..(r.end as i64 + offset) as u64
        })
        .collect_vec()
}

pub fn part_two(input: &str) -> Option<u64> {
    let chunks = input
        .lines()
        .group_by(|s| s != &"")
        .into_iter()
        .map(|chunk| chunk.1.collect_vec())
        .filter(|v| v.len() > 1 || !v[0].is_empty())
        .collect_vec();
    let seeds = chunks[0][0]
        .split(' ')
        .filter_map(|s| s.parse::<u64>().ok())
        .batching(|it| match it.next() {
            None => None,
            Some(i) => it.next().map(|j| i..i + j),
        })
        .collect_vec();

    let translation = chunks
        .into_iter()
        .skip(1)
        .map(|chunk| {
            chunk
                .into_iter()
                .skip(1)
                .filter_map(|s| {
                    s.split(' ')
                        .filter_map(|s| s.parse::<u64>().ok())
                        .collect_tuple::<(_, _, _)>()
                })
                .collect_vec()
        })
        .collect_vec();

    let output_ranges = translation.into_iter().fold(seeds, |input_ranges, layer| {
        input_ranges
            .into_iter()
            .flat_map(|r| translate_stage_ranges(r, &layer))
            .collect_vec()
    });
    output_ranges.into_iter().map(|r| r.start).min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
