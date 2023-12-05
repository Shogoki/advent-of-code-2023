#![feature(map_try_insert)]
use std::collections::BTreeMap;

fn get_neighbors(y: usize, x: usize, len: usize, y_size: usize, x_size: usize) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize,usize)> = vec![];
    let x_start = if x > 0 {
        res.push((y,x-1));
        x -1 
    } else  {
        0
    };
    let x_end = if x +len < x_size -1 { 
        res.push((y,x+len ));
        x + len  
    } else {
        x_size -1 
    };
    let y_start = if y > 0 { y -1 } else  {0};
    let y_end = if y < y_size -1  { y + 1 } else { y_size -1 };

    for i in x_start..=x_end {
        for j in y_start..=y_end {
            if j == y { continue;} 
            res.push((j,i));
        }
    }
    return res;
}

pub fn process_part1(input: &str) -> String {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<Vec<_>>>(); 
    let mut sum: u32 = 0;
    //result.to_string()
    let y_size = map.len();
    let x_size = map[0].len();
    for y in 0..y_size {
        let mut part_number: String = "".to_string();
        for x in 0..x_size {
            if map[y][x].is_numeric() {
                part_number = part_number + &map[y][x].to_string();
                // check if we are at the end of the line, or next char is non numeric
                if x +1 == x_size  || !map[y][x+1].is_numeric() {
                    let neighbors = get_neighbors(y, x +1 -part_number.len(), part_number.len(), y_size, x_size);
                    //dbg!(&neighbors);
                    for n in neighbors.iter() {
                        let current = map[n.0][n.1];
                        if current != '.' && !current.is_numeric() {
                            dbg!(&part_number);
                            sum = sum + part_number.parse::<u32>().unwrap();
                            dbg!(sum);
                            break;
                        }
                    }
                    part_number = "".to_string();
                }
            } 
        }
    }
    sum.to_string()
}


pub fn process_part2(input: &str) -> String {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<Vec<_>>>(); 
    let mut star_parts = BTreeMap::new();
    let y_size = map.len();
    let x_size = map[0].len();
    for y in 0..y_size {
        let mut part_number: String = "".to_string();
        for x in 0..x_size {
            if map[y][x].is_numeric() {
                part_number = part_number + &map[y][x].to_string();
                // check if we are at the end of the line, or next char is non numeric
                if x +1 == x_size  || !map[y][x+1].is_numeric() {
                    let neighbors = get_neighbors(y, x +1 -part_number.len(), part_number.len(), y_size, x_size);
                    for &n in neighbors.iter() {
                        let current = map[n.0][n.1];
                        if current == '*' {
                            let mut vec = match star_parts.try_insert(n,vec![]) {
                                Ok(v) => v.to_vec(),
                                Err(e) => e.entry.get().to_vec(),
                            };
                            vec.push(part_number.parse::<u32>().unwrap());
                            star_parts.insert(n,vec); 
                        }
                    }
                    part_number = "".to_string();
                }
            } 
        }
    }
    star_parts.keys().filter_map(|key| {
        let v = star_parts.get(key).unwrap();
        if v.len() == 2 { 
            Some(v.iter().product::<u32>())
        } else {
            None
        }
    }).sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    const INPUT2: &str = INPUT;

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "4361");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "467835");
    }
}
