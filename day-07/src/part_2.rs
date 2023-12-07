use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult, Parser,
};

pub fn process_part_2(input: &str) -> String {
    let (_, hands) = parse_input(input).unwrap();
    let mut hands = hands;
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u64 + 1) * hand.bid)
        .sum::<u64>()
        .to_string()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(line_ending, parse_hand)(input)
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, (cards, bid)) = separated_pair(
        tuple((parse_card, parse_card, parse_card, parse_card, parse_card))
            .map(|cards| cards.into()),
        space1,
        complete::u64,
    )(input)?;
    Ok((
        input,
        Hand {
            cards,
            bid,
            hand_type: hand_type(cards),
        },
    ))
}

fn parse_card(input: &str) -> IResult<&str, u8> {
    alt((
        tag("2"),
        tag("3"),
        tag("4"),
        tag("5"),
        tag("6"),
        tag("7"),
        tag("8"),
        tag("9"),
        tag("T"),
        tag("J"),
        tag("Q"),
        tag("K"),
        tag("A"),
    ))
    .map(|s| match s {
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "T" => 10,
        "J" => 1,
        "Q" => 12,
        "K" => 13,
        "A" => 14,
        _ => unreachable!(),
    })
    .parse(input)
}

#[derive(Debug)]
struct Hand {
    cards: [u8; 5],
    bid: u64,
    hand_type: HandType,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_type
            .value()
            .cmp(&other.hand_type.value())
            .then_with(|| self.cards[0].cmp(&other.cards[0]))
            .then_with(|| self.cards[1].cmp(&other.cards[1]))
            .then_with(|| self.cards[2].cmp(&other.cards[2]))
            .then_with(|| self.cards[3].cmp(&other.cards[3]))
            .then_with(|| self.cards[4].cmp(&other.cards[4]))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), std::cmp::Ordering::Equal)
    }
}

fn hand_type(cards: [u8; 5]) -> HandType {
    let mut counts = BTreeMap::new();
    for card in cards.iter() {
        counts
            .entry(*card)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let five = cards_with_count(&counts, 5);
    let four = cards_with_count(&counts, 4);
    let three = cards_with_count(&counts, 3);
    let two = cards_with_count(&counts, 2);
    let one = cards_with_count(&counts, 1);
    let joker = counts
        .iter()
        .find(|(card, _)| **card == 1)
        .map(|(_, card_count)| *card_count)
        .unwrap_or(0);
    if joker == 5
        || !five.is_empty()
        || joker >= 1 && !four.is_empty()
        || joker >= 2 && !three.is_empty()
        || joker >= 3 && !two.is_empty()
        || joker >= 4 && !one.is_empty()
    {
        return HandType::FiveOfAKind;
    }
    if !four.is_empty()
        || joker >= 1 && !three.is_empty()
        || joker >= 2 && !two.is_empty()
        || joker >= 3 && !one.is_empty()
    {
        return HandType::FourOfAKind;
    }
    if three.len() == 1 && two.len() == 1 || joker >= 1 && two.len() == 2 {
        return HandType::FullHouse;
    }
    if !three.is_empty() || joker >= 1 && !two.is_empty() || joker >= 2 && !one.is_empty() {
        return HandType::ThreeOfAKind;
    }
    if two.len() == 2 {
        return HandType::TwoPair;
    }
    if two.len() == 1 || joker >= 1 && !one.is_empty() {
        return HandType::OnePair;
    }
    HandType::HighCard
}

fn cards_with_count(card_counts: &BTreeMap<u8, i32>, count: i32) -> Vec<u8> {
    card_counts
        .iter()
        .filter(|(card, _)| **card != 1)
        .filter_map(|(card, card_count)| {
            if *card_count == count {
                Some(*card)
            } else {
                None
            }
        })
        .collect::<Vec<u8>>()
}

#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn value(&self) -> u8 {
        match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
}
