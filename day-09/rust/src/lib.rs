#![feature(iter_map_windows)]

fn get_next(input: Vec<i64>) -> i64 {
    dbg!(&input);

    if input.iter().all(|x| *x == 0) {
        return 0;
    }
    let child: Vec<i64> = input.iter().map_windows(|[x, y]| *y - *x).collect();

    let res = get_next(child.to_vec());
    dbg!(res);
    return res + input.last().unwrap();
}

pub fn process_part1(input: &str) -> String {
    let result: i64 =
        input
            .lines()
            .map(|line| {
                let history: Vec<i64> =
                    line.split(" ").map(|n| n.parse::<i64>().unwrap()).collect();
                dbg!(&history);
                get_next(history)
            })
            .sum();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result: i64 =
        input
            .lines()
            .map(|line| {
                let history: Vec<i64> =
                    line.split(" ").map(|n| n.parse::<i64>().unwrap()).collect();
                let reversed = history.iter().rev().map(|n| *n).collect();
                dbg!(&history);
                get_next(reversed)
            })
            .sum();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    const INPUT2: &str = INPUT;

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "114");
    }

    #[test]
    //#[ignore = "part1"]
    fn part2_works() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "2");
    }
}
