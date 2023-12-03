use take_until::TakeUntilExt;

pub fn process_part1(input: &str) -> String {
    let result = input
        .lines()
        .map(|line| {
           let digit1: u32 = line.chars().take_until(|x | x.is_numeric()).last().unwrap().to_digit(10).unwrap() * 10 ;

           let digit2 = line.chars().rev().take_until(|x | x.is_numeric()).last().unwrap().to_digit(10).unwrap();
           return digit1 + digit2; 
        })
        .sum::<u32>();
    result.to_string()
}

fn parse_digit(digit: & str) -> u32 {
    return match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => digit.parse::<u32>().unwrap()
    };
}

pub fn process_part2(input: &str) -> String {
    let result = input
        .lines()
        .map(|line| {
    let digits = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut first: u32 = 1;
    let mut first_idx: usize = 9999; 
    let mut last: u32 = 1;
    let mut last_idx: usize = 0;
    digits.iter().for_each(|x| {
        let found = line.find(x);
        if found.is_some() {
            let f_num = found.unwrap();
            if f_num <= first_idx {
                first_idx = f_num;
                first = parse_digit(x);
            }
            let l_num = line.rfind(x).unwrap();
            if l_num >= last_idx {
                last_idx = l_num;
                last = parse_digit(x);
            }
        }
        
    });
       return first * 10 + last; 

        })
        .sum::<u32>();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    const INPUT2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "142");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "281");
    }
}
