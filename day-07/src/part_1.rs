use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult, Parser,
};

pub fn process_part_1(input: &str) -> String {
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
        "J" => 11,
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
    let mut counts = [0; 15];
    for card in cards.iter() {
        counts[*card as usize] += 1;
    }
    counts.sort_unstable();
    counts.reverse();
    match counts[0..4] {
        [5, _, _, _] => HandType::FiveOfAKind,
        [4, 1, _, _] => HandType::FourOfAKind,
        [3, 2, _, _] => HandType::FullHouse,
        [3, 1, 1, _] => HandType::ThreeOfAKind,
        [2, 2, 1, _] => HandType::TwoPair,
        [2, 1, 1, 1] => HandType::OnePair,
        _ => HandType::HighCard,
    }
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
