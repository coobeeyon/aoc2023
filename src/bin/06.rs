use itertools::Itertools;

advent_of_code::solution!(6);

fn race_distances(time: u64) -> impl Iterator<Item = u64> {
    (0..time).map(move |charge_time| {
        let v = charge_time;
        v * (time - charge_time)
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let vals = input
        .lines()
        .map(|s| {
            s.split(' ')
                .filter_map(|s| s.parse::<u64>().ok())
                .collect_vec()
        })
        .collect_vec();
    let races = vals[0]
        .iter()
        .cloned()
        .zip(vals[1].iter().cloned())
        .collect_vec();
    let winning_races = races
        .into_iter()
        .map(|(time, distance_record)| {
            race_distances(time)
                .filter(|d| *d > distance_record)
                .count() as u32
        })
        .collect_vec();
    Some(winning_races.into_iter().product())
}

pub fn part_two(input: &str) -> Option<u32> {
    let vals = input
        .lines()
        .filter_map(|s| {
            s.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u64>()
                .ok()
        })
        .collect_vec();
    let (time, distance) = (vals[0], vals[1]);
    Some(race_distances(time).filter(|d| *d > distance).count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
