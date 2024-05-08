use itertools::Itertools;

pub fn process_part1(input: &str) -> String {
    let res = input.lines().map(|line| {
        let (data, grp_str) = line.split_once(" ").unwrap();
        let grps = grp_str.split(",").map(|s| s.parse::<u32>()).collect();
        

        todo!()
        
    }).sum();
    res.to_string()
}

pub fn process_part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    const INPUT2: &str = INPUT;

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "21");
    }

    #[test]
    #[ignore = "part1"]
    fn part2_works() {
        let result = process_part2(INPUT2);
        // This would be the test for factor 100, but the actual factor is 1000000
        // assert_eq!(result, "8410");
        assert_eq!(result, "82000210");
    }
}
