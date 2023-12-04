use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult, Parser,
};

pub fn process_part_1(input: &str) -> String {
    let (_, cards) = parse_input(input).unwrap();
    cards.iter().map(Card::score).sum::<u32>().to_string()
}

pub fn process_part_2(input: &str) -> String {
    let (_, cards) = parse_input(input).unwrap();
    let mut card_numbers = cards
        .iter()
        .map(|card| (card.id, 1))
        .collect::<BTreeMap<u32, u32>>();
    for card in cards {
        let card_count = card_numbers.get(&card.id).cloned().unwrap();
        let winning_number_count = card.winning_number_count();
        for i in 0..winning_number_count {
            card_numbers
                .entry(card.id + i + 1)
                .and_modify(|card_number| *card_number += card_count)
                .or_insert(card_count);
        }
    }
    card_numbers.values().sum::<u32>().to_string()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(
        line_ending,
        separated_pair(
            preceded(terminated(tag("Card"), space1), complete::u32),
            terminated(tag(":"), space1),
            separated_pair(
                separated_list1(space1, complete::u32),
                delimited(space1, tag("|"), space1),
                separated_list1(space1, complete::u32),
            ),
        )
        .map(|(id, (winning_numbers, my_numbers))| Card {
            id,
            winning_numbers,
            my_numbers,
        }),
    )(input)
}

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        let winning_number_count = self.winning_number_count();
        if winning_number_count == 0 {
            0
        } else {
            2_u32.pow(winning_number_count - 1)
        }
    }

    fn winning_number_count(&self) -> u32 {
        self.my_numbers
            .iter()
            .filter(|my_number| self.winning_numbers.contains(my_number))
            .count() as u32
    }
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

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "13");
    }

    #[test]
    fn part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "30");
    }
}
