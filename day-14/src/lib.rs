use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    multi::{many1, separated_list1},
    IResult, Parser,
};

pub fn process_part_1(input: &str) -> String {
    let (input, platform) = parse_input(input).unwrap();
    debug_assert_eq!(input, "");
    let mut platform = platform;
    for _ in 0..platform.len() {
        for y in (1..platform.len()).rev() {
            for x in 0..platform[y].len() {
                if matches!(platform[y][x], Tile::RoundRock)
                    && matches!(platform[y - 1][x], Tile::Empty)
                {
                    platform[y - 1][x] = Tile::RoundRock;
                    platform[y][x] = Tile::Empty;
                }
            }
        }
    }

    print_platform(&platform);
    calc_total_load(&platform).to_string()
}

pub fn process_part_2(input: &str) -> String {
    let (input, platform) = parse_input(input).unwrap();
    debug_assert_eq!(input, "");

    let mut platform = platform;
    let mut cycles = HashMap::new();
    cycles.insert(platform.clone(), 0_usize);

    for i in 1..=1_000_000_000 {
        spin_cycle(&mut platform);
        let key = platform.clone();
        if cycles.contains_key(&key) {
            break;
        }
        cycles.insert(platform.clone(), i);
    }

    let loop_start = cycles[&platform];
    let mut cycles = cycles.iter().map(|(k, v)| (k, v)).collect::<Vec<_>>();
    cycles.sort_by(|a, b| a.1.cmp(b.1));
    let cycles = cycles.iter().map(|(k, _)| k).collect::<Vec<_>>();
    let loop_len = cycles.len() - loop_start;
    let platform = cycles[loop_start + (1_000_000_000 - loop_start) % loop_len];

    print_platform(platform);
    calc_total_load(platform).to_string()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list1(
        line_ending,
        many1(alt((
            tag("O").map(|_| Tile::RoundRock),
            tag("#").map(|_| Tile::CubeRock),
            tag(".").map(|_| Tile::Empty),
        ))),
    )(input)
}

#[allow(clippy::needless_range_loop)]
fn spin_cycle(platform: &mut [Vec<Tile>]) {
    for _ in 0..platform.len() {
        for y in (1..platform.len()).rev() {
            for x in 0..platform[y].len() {
                if matches!(platform[y][x], Tile::RoundRock)
                    && matches!(platform[y - 1][x], Tile::Empty)
                {
                    platform[y - 1][x] = Tile::RoundRock;
                    platform[y][x] = Tile::Empty;
                }
            }
        }
    }
    for _ in 0..platform[0].len() {
        for x in (1..platform[0].len()).rev() {
            for y in 0..platform.len() {
                if matches!(platform[y][x], Tile::RoundRock)
                    && matches!(platform[y][x - 1], Tile::Empty)
                {
                    platform[y][x - 1] = Tile::RoundRock;
                    platform[y][x] = Tile::Empty;
                }
            }
        }
    }
    for _ in 0..platform.len() {
        for y in 0..platform.len() - 1 {
            for x in 0..platform[y].len() {
                if matches!(platform[y][x], Tile::RoundRock)
                    && matches!(platform[y + 1][x], Tile::Empty)
                {
                    platform[y + 1][x] = Tile::RoundRock;
                    platform[y][x] = Tile::Empty;
                }
            }
        }
    }
    for _ in 0..platform[0].len() {
        for x in 0..platform[0].len() - 1 {
            for y in 0..platform.len() {
                if matches!(platform[y][x], Tile::RoundRock)
                    && matches!(platform[y][x + 1], Tile::Empty)
                {
                    platform[y][x + 1] = Tile::RoundRock;
                    platform[y][x] = Tile::Empty;
                }
            }
        }
    }
}

fn print_platform(platform: &[Vec<Tile>]) {
    for row in platform {
        for tile in row {
            print!("{}", tile)
        }
        println!()
    }
}

fn calc_total_load(platform: &[Vec<Tile>]) -> usize {
    let len = platform.len();
    platform.iter().enumerate().fold(0, |acc, (y, row)| {
        acc + row
            .iter()
            .filter(|tile| matches!(tile, Tile::RoundRock))
            .map(|_| len - y)
            .sum::<usize>()
    })
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    RoundRock,
    CubeRock,
    Empty,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Tile::RoundRock => write!(f, "O"),
            Tile::CubeRock => write!(f, "#"),
            Tile::Empty => write!(f, "."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "136");
    }

    #[test]
    fn part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "64");
    }
}
