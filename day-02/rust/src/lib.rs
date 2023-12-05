use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, multispace1, newline, space1},
    multi::{many1, separated_list1},
    combinator::map_res,
    sequence::tuple,
    IResult,
};
use std::cmp;


#[derive(Debug)]
struct GameRound {
    red: u32,
    green: u32,
    blue: u32,
}

impl GameRound {
    fn get_power(&self) -> u32{
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Game {
    num: u32,
    rounds: Vec<GameRound>,
}

impl Game {

    fn get_max(&self) -> GameRound {
        let mut max_dices = GameRound {red: 0, green:0 , blue: 0};
        self.rounds.iter().for_each(|r| {
            max_dices.red = cmp::max(max_dices.red, r.red);
            max_dices.blue = cmp::max(max_dices.blue, r.blue);
            max_dices.green = cmp::max(max_dices.green, r.green);
        });
        return max_dices;

    }

    fn is_possible(&self, round: &GameRound) -> bool {
        let max = self.get_max();
        if round.red < max.red  {
            return false;
        }
        if round.blue < max.blue  {
            return false;
        }
        if round.green < max.green  {
            return false;
        }
        return true;
    }
}
//Parsers
fn dices (input: &str) -> IResult<&str, (u32, &str)> {
    let (input, num) = complete::u32(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = alt((tag("red"),tag("green"),tag("blue")))(input)?;
    Ok((input, (num, color)))

}
fn round (input: &str) -> IResult<&str, GameRound> {
   let (input, dice) = separated_list1(tag(", "), dices)(input)?;
   let mut res = GameRound {red: 0,green: 0, blue: 0};
   dice.iter().for_each(|d| {
       let (num, color) = d;
       match *color {
           "red" => res.red = *num,
           "blue" => res.blue = *num,
           "green" => res.green= *num,
           _ => panic!("invalid Color")
       }
   });
   Ok((input, res))
}
fn game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, num) = complete::u32(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, rounds) = separated_list1(tag("; "),round)(input)?;
    let res= Game {num,  rounds};
    Ok((input, res))


}
fn games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, res) = separated_list1(newline, game)(input)?;
    Ok((input, res))
}
pub fn process_part1(input: &str) -> String {
    let round = GameRound{red: 12, green: 13, blue: 14};
    let (_, gs) = games(input).unwrap();
    let result = gs.iter().filter(|g| g.is_possible(&round)).map(|game| game.num).sum::<u32>();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, gs) = games(input).unwrap();
    let result = gs.iter().map(|game| game.get_max().get_power()).sum::<u32>();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    const INPUT2: &str = INPUT;

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "8");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "2286");
    }
}
