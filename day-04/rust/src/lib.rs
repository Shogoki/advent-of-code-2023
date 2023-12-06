use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, newline, space1},
    multi::{separated_list1 },
    sequence::{terminated, preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
#[derive(Clone)]
struct ScratchCard {
    num: usize,
    winning_numbers: Vec<u32>,
    printed_numbers: Vec<u32>,
    is_redeemed: bool,
}
impl ScratchCard {
    fn num_winnings(&self) -> usize {
        self.printed_numbers.iter().filter(|num| self.winning_numbers.contains(num)).collect::<Vec<_>>().len() 
    }

    fn wins_cardnums(&self) -> std::ops::Range<usize>{
        let start = self.num +1;
        start..(start +self.num_winnings())

    }
}

fn numbers(input: &str) -> IResult<&str, Vec<u32>> {
    //dbg!(input);
    let (input, res) = separated_list1(space1,complete::u32)(input)?; 
    Ok((input, res))

}


fn scratchcards(input: &str) -> IResult<&str, ScratchCard> {
    let (input, num) = preceded(terminated(tag("Card"),space1),terminated(digit1, terminated(tag(":"), space1)))(input)?;
    let (input, vecs) = separated_pair(numbers, preceded(space1,terminated(tag("|"),space1)),numbers)(input)?;
    Ok((input, ScratchCard {num: num.parse().unwrap(), winning_numbers: vecs.0, printed_numbers: vecs.1, is_redeemed: false}))
}
fn lines_p(input: &str) -> IResult<&str, BTreeMap<usize,ScratchCard>> {
     let (input, res) = separated_list1(newline, scratchcards)(input)?;
     let map = res.iter().map(|c| (c.num, c.clone())).collect::<BTreeMap<_,_>>();
     Ok((input,map))
}
pub fn process_part1(input: &str) -> String {
    let (_, cards) = lines_p(input).unwrap();
    let result = cards.values().map(|card| {
        let ret = match card.num_winnings() {
            0 => 0,
            v => 2_u32.pow((v -1) as u32)
        };
        ret

    })
    .sum::<u32>();
    result.to_string()
}

fn redeem_cards(cards: Vec<ScratchCard>, base: BTreeMap<usize,ScratchCard>, prev_wins: Vec<ScratchCard>) -> Vec<ScratchCard> {
    
    if cards.len() == 0  {
        return prev_wins;
    }


    let mut won_cards = vec![];
    cards.iter().for_each(|card| {
         card.wins_cardnums()
             .for_each(|win| {

            won_cards.push(base.get(&win).unwrap().clone()); 
        });
    });
    let mut old_cards = cards.to_vec();
    old_cards.extend(prev_wins);

    return redeem_cards(won_cards, base, old_cards)

}


pub fn process_part2(input: &str) -> String {
    let (_, base_cards) = lines_p(input).unwrap();
    let cards = base_cards.values().map(|v| v.clone()).collect::<Vec<_>>().to_vec();
    let total_cards = redeem_cards(cards, base_cards, vec![]);
    total_cards.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    const INPUT2: &str = INPUT;

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "13");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "30");
    }
}
