use std::{collections::{BTreeMap, HashSet}, ops::Range};

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, newline, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

enum MapType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug, Clone)]
struct SeedMap<'a> {
    from: &'a str,
    to: &'a str,
    dst: Vec<Range<u64>>,
    src: Vec<Range<u64>>,
}
impl SeedMap<'_> {
    fn get_dst(&self, from: u64) -> u64 {
        let gap: i64 = match self
            .src
            .iter()
            .enumerate()
            .find(|(_, r)| r.start <= from && from < r.end) {
            Some((idx, r)) => (self.dst[idx].start as i64) - (r.start as i64),
            None => 0 as i64,
        };
        return (from as i64 + gap) as u64;
    }
}

fn parse_map(input: &str) -> IResult<&str, SeedMap> {
    let (input, from) = terminated(alpha1, tag("-to-"))(input)?;
    let (input, to) = terminated(terminated(alpha1, tag(" map:")), newline)(input)?;
    println!("parse start Map {from} -> {to}");
    let (input, mapdefs) = separated_list1(
        newline,
        tuple((
            complete::u64,
            preceded(space1, complete::u64),
            preceded(space1, complete::u64),
        )),
    )(input)?;
    println!("Got Definitions");
    let mut dst = vec![];
    let mut src = vec![];
    mapdefs.iter().for_each(|def| {
        let (d_start, s_start, r_len) = def;
        dst.push(*d_start..(d_start + r_len));
        src.push(*s_start..(s_start + r_len));
    });
    Ok((input, SeedMap { dst, src, from, to }))
}

fn parse_almanac(input: &str) -> IResult<&str, (Vec<u32>, Vec<SeedMap>)> {
    let (input, seeds) =
        terminated(
            preceded(tag("seeds: "), separated_list1(space1, complete::u32)),
            tag("\n\n"),
        )(input)?;
    println!("parsed SEEDS");
    let (input, maps) = separated_list1(preceded(newline, newline), parse_map)(input)?;
    Ok((input, (seeds, maps)))
}
pub fn process_part1(input: &str) -> String {
    println!("Start");
    let (_, (seeds, maps)) = parse_almanac(input).unwrap();
    println!("FINISHED PARSING");
    const TARGET: &str = "location";
    let result = seeds
        .iter()
        .map(|s| {
            println!("Processing Seed: {}", s);
            let mut current_from = "seed";
            let mut id = *s as u64;
            while current_from != TARGET {
                println!("Currently at {current_from}");
                let map = maps.iter().find(|m| m.from == current_from).unwrap();
                id = map.get_dst(id);
                current_from = map.to;
            }
            id
        })
        .min()
        .unwrap();
    result.to_string()
}

fn parse_almanac2(input: &str) -> IResult<&str, (Vec<u32>, Vec<SeedMap>)> {
    let (input, seeds_params) =
        terminated(
            preceded(tag("seeds: "), separated_list1(space1, separated_pair(complete::u32, space1, complete::u32))),
            tag("\n\n"),
        )(input)?;
    let seed  = seeds_params.iter().map(|&(start, len)| start..(start + len)).flatten().collect();
    println!("parsed SEEDS");
    let (input, maps) = separated_list1(preceded(newline, newline), parse_map)(input)?;
    Ok((input, (seed, maps)))
}

pub fn process_part2(input: &str) -> String {
    println!("Start");
    let (_, (seeds, maps)) = parse_almanac2(input).unwrap();
    println!("FINISHED PARSING");
    const TARGET: &str = "location";
    let result = seeds
        .iter()
        .map(|s| {
            println!("Processing Seed: {}", s);
            let mut current_from = "seed";
            let mut id = *s as u64;
            while current_from != TARGET {
                let map = maps.iter().find(|m| m.from == current_from).unwrap();
                id = map.get_dst(id);
                current_from = map.to;
            }
            id
        })
        .min()
        .unwrap();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    const INPUT2: &str = INPUT;

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "35");
    }

    #[test]
    //#[ignore = "part1"]
    fn part2_works() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "46");
    }
}
