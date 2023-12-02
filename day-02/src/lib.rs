use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

pub fn process_part_1(input: &str) -> String {
    let (_, games) = parse_input(input).expect("Failed to parse input");
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let sum_ids: u32 = games
        .iter()
        .filter(|game| {
            game.rounds.iter().all(|round| {
                round.red <= max_red && round.green <= max_green && round.blue <= max_blue
            })
        })
        .map(|game| game.id)
        .sum();
    sum_ids.to_string()
}

pub fn process_part_2(input: &str) -> String {
    let (_, games) = parse_input(input).expect("Failed to parse input");
    let sum_power: u32 = games
        .iter()
        .map(|game| {
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;
            game.rounds.iter().for_each(|round| {
                max_red = max_red.max(round.red);
                max_green = max_green.max(round.green);
                max_blue = max_blue.max(round.blue);
            });
            max_red * max_green * max_blue
        })
        .sum();
    sum_power.to_string()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(line_ending, parse_game)(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, ((_, game_id), rounds)) =
        separated_pair(tuple((tag("Game "), digit1)), tag(": "), parse_rounds)(input)?;
    Ok((
        input,
        Game {
            id: game_id
                .parse()
                .unwrap_or_else(|_| panic!("Invalid game id: {}", game_id)),
            rounds,
        },
    ))
}

fn parse_rounds(input: &str) -> IResult<&str, Vec<Round>> {
    separated_list1(tag("; "), parse_round)(input)
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    let (input, round_bits) = separated_list1(
        tag(", "),
        tuple((digit1, space1, alt((tag("red"), tag("green"), tag("blue"))))),
    )(input)?;
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for (amount, _, color) in round_bits {
        let amount: u32 = amount
            .parse()
            .unwrap_or_else(|_| panic!("Invalid amount: {}", amount));
        match color {
            "red" => red += amount,
            "green" => green += amount,
            "blue" => blue += amount,
            _ => panic!("Invalid color: {}", color),
        }
    }
    Ok((input, Round { red, green, blue }))
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "8");
    }

    #[test]
    fn part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "2286");
    }
}
