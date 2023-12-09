use std::{
    collections::{BTreeMap, HashSet},
    ops::Range,
};

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, newline, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

fn parse_races(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, times) = preceded(
        preceded(tag("Time:"), space1),
        separated_list1(space1, complete::u32),
    )(input)?;
    let (input, _) = newline(input)?;
    let (input, distances) = preceded(
        preceded(tag("Distance:"), space1),
        separated_list1(space1, complete::u32),
    )(input)?;

    Ok((input, (times, distances)))
}

fn get_possible_charges(time: u64, dist: u64) -> Vec<u64> {
    (1..time)
        .filter(|speed| {
            // we move with speed, for the full time minus charged time = speed
            let moved = speed * (time - speed);
            moved > dist
        })
        .collect()
}
pub fn process_part1(input: &str) -> String {
    let (_, (times, distances)) = parse_races(input).unwrap();
    let result: u64 = times
        .iter()
        .enumerate()
        .map(|(idx, time )| get_possible_charges(*time as u64, distances[idx] as u64).len() as u64)
        .product();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut iter = input
        .lines().take(2).map(|line| line.split(":")
        .last()
        .unwrap()
        .replace(" ", "")
        .parse::<u64>().unwrap());
    let time = iter.next().unwrap();
    let dist = iter.next().unwrap();
    let wins = get_possible_charges(time, dist);
    wins.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";
    const INPUT2: &str = INPUT;

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "288");
    }

    #[test]
    //#[ignore = "part1"]
    fn part2_works() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "71503");
    }
}
