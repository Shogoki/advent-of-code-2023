use std::{
    collections::{BTreeMap, HashSet},
    ops::Range,
};

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, newline, space1, alphanumeric1},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

fn parse_node(input: &str) -> IResult<&str, (&str, &str, &str)> {
    let (input, node) = terminated(alphanumeric1, terminated(preceded(space1, tag("=")), space1))(input)?;
    dbg!(node);
    let (input, (left, right)) = preceded(
        tag("("),
        terminated(
            separated_pair(alphanumeric1, preceded(tag(","), space1), alphanumeric1),
            tag(")"),
        ),
    )(input)?;
    Ok((input, (node, left, right)))
}
fn parse_map(input: &str) -> IResult<&str, (&str, BTreeMap<&str, (&str, &str)>)> {
    let (input, instructions) = alpha1(input)?;
    dbg!(instructions);
    let (input, _) = preceded(newline, newline)(input)?;
    let (input, nodes) = separated_list1(newline, parse_node)(input)?;

    let mut node_map: BTreeMap<&str, (&str, &str)> = Default::default();
    nodes.iter().for_each(|(node, left, right)| {
        node_map.insert(node, (left, right));
    });

    Ok((input, (instructions, node_map)))
}

pub fn process_part1(input: &str) -> String {
    let (_, (inst_base, node_map)) = parse_map(input).unwrap();
    let target = "ZZZ";
    let start = "AAA";
    let mut instructions = inst_base.chars().cycle();

    let mut steps = 0;
    let mut current = start;
    while current != target {
        steps += 1;
        current = match instructions.next() {
            Some('L') => node_map.get(current).unwrap().0,
            Some('R') => node_map.get(current).unwrap().1,
            _ => panic!("should not happen"),
        };
    }

    steps.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, (inst_base, node_map)) = parse_map(input).unwrap();
    let mut instructions = inst_base.chars().cycle();

    let mut steps = 0;
    //DEFINE StARt
    let mut current: Vec<&str> = node_map
        .keys()
        .cloned()
        .filter(|node| node.ends_with("A"))
        .collect();
    while !current.iter().all(|node| node.ends_with("Z")) {
        steps += 1;
        let inst = instructions.next().unwrap();
        current = current
            .iter()
            .map(|node| match inst {
                'L' => node_map.get(node).unwrap().0,
                'R' => node_map.get(node).unwrap().1,
                _ => panic!("should not happen"),
            })
            .collect();
    }
    steps.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    const INPUT2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "6");
    }

    #[test]
    //#[ignore = "part1"]
    fn part2_works() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "6");
    }
}
