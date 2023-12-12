use std::collections::BTreeMap;

#[derive(Debug,Eq,PartialEq)]
enum Direction {
    North,
    West,
    South,
    East,
    Nowehere, //This is if we reached the start
}
use take_until::TakeUntilExt;
use Direction::*;

pub fn process_part1(input: &str) -> String {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, character)| ((y as i32, x as i32), character))
        })
        //.filter(|(_, c)| *c != '.')
        .collect::<BTreeMap<(i32, i32), char>>();
    let start_pos = map
        .iter()
        .find_map(|(pos, char)| match char {
            'S' => Some(*pos as (i32, i32)),
            _ => None,
        })
        .unwrap();
    dbg!(&map);
    dbg!(start_pos);

    let ways = vec![
        ((start_pos.0 + 1, start_pos.1), North),
        ((start_pos.0 - 1, start_pos.1), South),
        ((start_pos.0, start_pos.1 + 1), West),
        ((start_pos.0, start_pos.1 - 1), East),
    ];

    let mut res =
        ways.into_iter().map(|(pos, dir)| {
            std::iter::successors(Some((pos, dir)), |(start, from)| {
                let current = *(map.get(&start)?);

                let way_to_go = match (current, from) {
                    ('|', North) => South,
                    ('|', South) => North,
                    ('-', West) => East,
                    ('-', East) => West,
                    ('L', North) => East,
                    ('L', East) => North,
                    ('J', North) => West,
                    ('J', West) => North,
                    ('7', West) => South,
                    ('7', South) => West,
                    ('F', South) => East,
                    ('F', East) => South,
                    ('S', _) => Nowehere, //TODO: This is just a dirty hack to handle
                    _ => unreachable!("should not happen"),
                };
                match way_to_go {
                    North => Some(((start.0 - 1, start.1), South)),
                    East => Some(((start.0, start.1 + 1), West)),
                    South => Some(((start.0 + 1, start.1), North)),
                    West => Some(((start.0, start.1 - 1), East)),
                    Nowehere => Some(((0, 1), Nowehere)),
                }
            })
        });
    let path = res.next().unwrap();
    let steps = path
        .take_until(|(pos, _)| *pos == start_pos)
        .collect::<Vec<_>>()
        .len();
    ((steps + 1) / 2).to_string()
}

pub fn process_part2(input: &str) -> String {
    let height  = input.lines().collect::<Vec<_>>().len();
    let width = input.lines().nth(0).unwrap().len();
    let mut map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, character)| ((y as i32, x as i32), character))
        })
        .collect::<BTreeMap<(i32, i32), char>>();
    let start_pos = map
        .iter()
        .find_map(|(pos, char)| match char {
            'S' => Some(*pos as (i32, i32)),
            _ => None,
        })
        .unwrap();
    dbg!(&map);
    dbg!(start_pos);

    let ways = vec![
        ((start_pos.0 + 1, start_pos.1), North),
        ((start_pos.0 - 1, start_pos.1), South),
        ((start_pos.0, start_pos.1 + 1), West),
        ((start_pos.0, start_pos.1 - 1), East),
    ];

    let mut res =
        ways.into_iter().map(|(pos, dir)| {
            std::iter::successors(Some((pos, dir)), |(start, from)| {
                let current = *(map.get(&start)?);

                let way_to_go = match (current, from) {
                    ('|', North) => South,
                    ('|', South) => North,
                    ('-', West) => East,
                    ('-', East) => West,
                    ('L', North) => East,
                    ('L', East) => North,
                    ('J', North) => West,
                    ('J', West) => North,
                    ('7', West) => South,
                    ('7', South) => West,
                    ('F', South) => East,
                    ('F', East) => South,
                    ('S', _) => Nowehere, //TODO: This is just a dirty hack to handle
                    _ => unreachable!("should not happen"),
                };
                match way_to_go {
                    North => Some(((start.0 - 1, start.1), South)),
                    East => Some(((start.0, start.1 + 1), West)),
                    South => Some(((start.0 + 1, start.1), North)),
                    West => Some(((start.0, start.1 - 1), East)),
                    Nowehere => Some(((0, 1), Nowehere)),
                }
            })
        });
    let pipes = res
        .next()
        .unwrap()
        .take_until(|(pos, _)| *pos == start_pos)
        .collect::<Vec<_>>();
    //now we got our pipes again. we can go thru the loop again, and then mark some neighbour tiles
    let loop_tiles = pipes.iter().map(|(pos, _)| pos).collect::<Vec<_>>();
    // replace non loop pipes with ground for now
    map = map
        .iter()
        .map(|(pos, c)| match loop_tiles.contains(&pos) {
            true => (*pos, *c),
            false => (*pos, '.'),
        })
        .collect();

    //Now we find left right for one position of the loop
    let (pos, dir) = pipes
        .iter()
        .take_until(|(pos, _)| {
            map.get(&pos).unwrap() == &'|'
                && map.get(&(pos.0, pos.1 - 1)) == Some(&'.')
                && map.get(&(pos.0, pos.1 + 1)) == Some(&'.')
        })
        .last()
        .unwrap();

    let left = (pos.0, pos.1 -1);
    let right = (pos.0, pos.1 +1);

    // now we can do a naive traversal thru the whole map, to see if left or right of the loop is
    // inside. We start at a definite outside pos and look thru all the pipes until we find the r|l
    // we know that the 0,0 is outside in both our test and real input
    let mut traversed = Vec::new();
    let mut current = (0 as i32,0 as i32);
    loop {
        traversed.push(current);


    }
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    const INPUT2: &str = INPUT;

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "8");
    }

    #[test]
    #[ignore = "part1"]
    fn part2_works() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "2");
    }
}
