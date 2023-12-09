use std::{cmp::Ordering, collections::BTreeMap, fmt::Display};

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, newline, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    // We define Types as enum in reversed order to have the highest value for the best type!
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq)]
struct Hand {
    bid: u64,
    hand: String,
    uses_jokers: bool,
}

impl Hand {
    fn get_type_basic(&self, card_count: Vec<u32>) -> HandType {
        if card_count.contains(&5) {
            return HandType::FiveOfAKind;
        } else if card_count.contains(&4) {
            // we need to check for jokers
            return HandType::FourOfAKind;
        } else if card_count.contains(&3) {
            if card_count.contains(&2) {
                return HandType::FullHouse;
            } else {
                return HandType::ThreeOfAKind;
            }
        } else if card_count.contains(&2) {
            let pair_count = card_count
                .iter()
                .filter(|x| *x == &2)
                .collect::<Vec<_>>()
                .len();
            if pair_count == 2 {
                return HandType::TwoPair;
            } else {
                return HandType::OnePair;
            }
        }
        return HandType::HighCard;
    }
    fn get_type(&self) -> HandType {
        let mut card_count: Vec<u32> = (0..14).map(|_| 0).collect();
        self.hand.chars().for_each(|card| {
            let val = self.get_card_value(card);
            card_count[(val as usize) - 1] += 1;
        });
        //if we use jokers they are at idx 0
        let jokers = card_count[0];
        // we have to reset this, because otherwise it will mess up our matching
        card_count[0] = 0;
        let h_type = self.get_type_basic(card_count);

        if self.uses_jokers && jokers > 0{
            return match (jokers, &h_type){
                (_, HandType::FiveOfAKind) => h_type,
                (_, HandType::FourOfAKind) => HandType::FiveOfAKind,
                (1, HandType::ThreeOfAKind) => HandType::FourOfAKind,
                (1, HandType::FullHouse) => HandType::FourOfAKind,
                (_, HandType::ThreeOfAKind) => HandType::FiveOfAKind,
                (_, HandType::FullHouse) => HandType::FiveOfAKind,
                (1, HandType::TwoPair) => HandType::FullHouse,
                (1, HandType::OnePair) => HandType::ThreeOfAKind,
                (1, HandType::HighCard) => HandType::OnePair,
                (2, HandType::TwoPair) => HandType::FourOfAKind,
                (_, HandType::TwoPair) => HandType::FiveOfAKind,
                (2, HandType::OnePair) => HandType::FourOfAKind,
                (_, HandType::OnePair) => HandType::FiveOfAKind,
                (2, HandType::HighCard) => HandType::ThreeOfAKind,
                (3, HandType::HighCard) => HandType::FourOfAKind,
                (_, HandType::HighCard) => HandType::FiveOfAKind,
                _ => panic!("should not happen, {:?} - {:?} {:?}", jokers, &h_type, self.hand)
            


            }
        } else {
            return h_type;
        }
    }

    fn get_card_value(&self, card: char) -> u32 {
        if self.uses_jokers && card == 'J' {
            return 1;
        }
        match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => card.to_digit(10).unwrap(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let first_type = self.get_type();
        let other_type = other.get_type();
        if first_type != other_type {
            return Some(first_type.cmp(&other_type));
        }
        for i in 0..(self.hand.len()) {
            let left = self.hand.chars().nth(i).unwrap();
            let right = other.hand.chars().nth(i).unwrap();
            if left != right {
                let left_val = self.get_card_value(left);
                return Some(left_val.cmp(&(other.get_card_value(right))));
            }
        }
        Some(1.cmp(&1)) //equal
    }
}

pub fn process_part1(input: &str) -> String {
    let mut hands: Vec<Hand> =
        input
            .lines()
            .map(|line| {
                let (hand, bid) = line.split_once(" ").unwrap();
                Hand {
                    hand: hand.to_string(),
                    bid: bid.parse::<u64>().unwrap(),
                    uses_jokers: false,
                }
            })
            .collect();
    hands.sort_by(|a, b| a.partial_cmp(b).unwrap());
    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid * (1 + idx as u64))
        .sum::<u64>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut hands: Vec<Hand> =
        input
            .lines()
            .map(|line| {
                let (hand, bid) = line.split_once(" ").unwrap();
                Hand {
                    hand: hand.to_string(),
                    bid: bid.parse::<u64>().unwrap(),
                    uses_jokers: true,
                }
            })
            .collect();
    hands.sort_by(|a, b| a.partial_cmp(b).unwrap());
    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid * (1 + idx as u64))
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    const INPUT2: &str = INPUT;

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "6440");
    }

    #[test]
    //#[ignore = "part1"]
    fn part2_works() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "5905");
    }
}
