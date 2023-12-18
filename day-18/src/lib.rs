use std::collections::hash_map::Entry;
use std::fmt::Write;
use std::{collections::HashMap, fmt::Display};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{self, line_ending, space1},
        streaming::hex_digit1,
    },
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult, Parser,
};

pub fn process_part_1(input: &str) -> String {
    let (input, steps) = parse_input(input).unwrap();
    debug_assert_eq!(input, "");
    let mut trench: HashMap<Position, Tile> = HashMap::new();
    let mut position = Position::new(0, 0);
    trench.insert(
        position,
        Tile::Edge {
            start: Direction::Unknown,
            end: Direction::Unknown,
        },
    );
    steps.iter().for_each(|step| {
        (0..step.amount).for_each(|_| {
            let next_position = position.move_in_direction(&step.direction, 1);
            if let Some(Tile::Edge { start: _, end }) = trench.get_mut(&position) {
                *end = step.direction;
            }
            if let Entry::Vacant(e) = trench.entry(next_position) {
                e.insert(Tile::Edge {
                    start: step.direction.opposite(),
                    end: Direction::Unknown,
                });
            } else if let Some(Tile::Edge { start, end: _ }) = trench.get_mut(&position) {
                *start = step.direction.opposite();
            }

            position = next_position;
        });
    });
    trench.entry(Position::new(0, 0)).and_modify(|tile| {
        if let Tile::Edge { start, end: _ } = tile {
            if matches!(start, Direction::Unknown) {
                *start = steps.last().unwrap().direction.opposite();
            }
        }
    });
    let filled_trench = fill_trench(&trench);
    // print_trench(&filled_trench);
    filled_trench.len().to_string()
}

pub fn process_part_2(input: &str) -> String {
    let (input, steps) = parse_input_part_2(input).unwrap();
    debug_assert_eq!(input, "");
    let mut position = Position::new(0, 0);
    let mut vertical_edges = vec![];
    let mut horizontal_edges = vec![];
    steps.iter().for_each(|step| {
        let next_position = position.move_in_direction(&step.direction, step.amount as i64);
        if matches!(step.direction, Direction::Up | Direction::Down) {
            vertical_edges.push(Edge {
                start: position,
                end: next_position,
            });
        } else {
            horizontal_edges.push(Edge {
                start: position,
                end: next_position,
            });
        }
        position = next_position;
    });
    vertical_edges.iter_mut().for_each(|edge| {
        if edge.start.y > edge.end.y {
            std::mem::swap(&mut edge.start, &mut edge.end);
        }
    });
    horizontal_edges.iter_mut().for_each(|edge| {
        if edge.start.x > edge.end.x {
            std::mem::swap(&mut edge.start, &mut edge.end);
        }
    });
    vertical_edges.sort_by(|a, b| a.start.x.cmp(&b.start.x));
    let min_y = vertical_edges.iter().map(|e| e.start.y).min().unwrap();
    let max_y = vertical_edges.iter().map(|e| e.end.y).max().unwrap();
    let mut area = 0;
    (min_y..max_y).for_each(|y| {
        let inline_horizontal_edges: Vec<&Edge> = horizontal_edges
            .iter()
            .filter(|edge| y == edge.start.y && y == edge.end.y)
            .collect();
        let mut corssing_start: Option<&Edge> = None;
        vertical_edges.iter().for_each(|edge| {
            if y >= edge.start.y && y < edge.end.y {
                match corssing_start {
                    Some(Edge { start, end: _ }) => {
                        let horizontal_area = inline_horizontal_edges
                            .iter()
                            .filter(|horizontal_edge| {
                                horizontal_edge.start.x >= start.x
                                    && horizontal_edge.end.x <= edge.end.x
                            })
                            .map(|horizontal_edge| {
                                let mut len = horizontal_edge.end.x - horizontal_edge.start.x + 1;
                                if horizontal_edge.start.x == start.x {
                                    len -= 1;
                                }
                                if horizontal_edge.end.x == edge.end.x {
                                    len -= 1;
                                }
                                len
                            })
                            .sum::<i64>();
                        area += edge.start.x - start.x - 1 - horizontal_area;
                        corssing_start = None;
                    }
                    None => {
                        corssing_start = Some(edge);
                    }
                }
            }
        })
    });
    let vertical_area = vertical_edges
        .iter()
        .map(|edge| edge.end.y - edge.start.y + 1)
        .sum::<i64>();
    let horizontal_area = horizontal_edges
        .iter()
        .map(|edge| edge.end.x - edge.start.x - 1)
        .sum::<i64>();
    area += vertical_area;
    area += horizontal_area;
    area.to_string()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Step>> {
    separated_list1(line_ending, parse_step)(input)
}

fn parse_step(input: &str) -> IResult<&str, Step> {
    let (input, (direction, amount, _color)) = tuple((
        alt((
            tag("U").map(|_| Direction::Up),
            tag("D").map(|_| Direction::Down),
            tag("L").map(|_| Direction::Left),
            tag("R").map(|_| Direction::Right),
        )),
        delimited(space1, complete::u32, space1),
        delimited(tag("("), preceded(tag("#"), hex_digit1), tag(")")),
    ))(input)?;
    let step = Step { direction, amount };
    Ok((input, step))
}

fn parse_input_part_2(input: &str) -> IResult<&str, Vec<Step>> {
    separated_list1(line_ending, parse_step_part_2)(input)
}

fn parse_step_part_2(input: &str) -> IResult<&str, Step> {
    let (input, (_direction, _amount, color)) = tuple((
        alt((tag("U"), tag("D"), tag("L"), tag("R"))),
        delimited(space1, complete::u32, space1),
        delimited(tag("("), preceded(tag("#"), hex_digit1), tag(")")),
    ))(input)?;
    let amount = u32::from_str_radix(&color[0..5], 16).unwrap();
    let direction = u32::from_str_radix(&color[5..6], 16).unwrap();
    let direction = match direction {
        0 => Direction::Right,
        1 => Direction::Down,
        2 => Direction::Left,
        3 => Direction::Up,
        _ => panic!("Invalid direction"),
    };
    let step = Step { direction, amount };
    Ok((input, step))
}

fn fill_trench(trench: &HashMap<Position, Tile>) -> HashMap<Position, Tile> {
    let mut filled_trench = trench.clone();

    let min_x = trench.keys().map(|p| p.x).min().unwrap();
    let max_x = trench.keys().map(|p| p.x).max().unwrap();
    let min_y = trench.keys().map(|p| p.y).min().unwrap();
    let max_y = trench.keys().map(|p| p.y).max().unwrap();

    for y in min_y..max_y {
        let mut crossed = 0;
        for x in min_x..max_x {
            let position = Position::new(x, y);
            match trench.get(&position) {
                Some(Tile::Edge { start, end }) => {
                    if matches!(start, Direction::Up) || matches!(end, Direction::Up) {
                        crossed += 1;
                    }
                }
                None => {
                    if crossed % 2 == 1 {
                        filled_trench.insert(position, Tile::Fill);
                    }
                }
                _ => {}
            }
        }
    }

    filled_trench
}

#[allow(dead_code)]
fn print_trench(tiles: &HashMap<Position, Tile>) {
    let min_x = tiles.keys().map(|p| p.x).min().unwrap();
    let max_x = tiles.keys().map(|p| p.x).max().unwrap();
    let min_y = tiles.keys().map(|p| p.y).min().unwrap();
    let max_y = tiles.keys().map(|p| p.y).max().unwrap();
    let mut map =
        vec![vec![Tile::Empty; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    tiles.iter().for_each(|(position, tile)| {
        map[(position.y - min_y) as usize][(position.x - min_x) as usize] = *tile;
    });
    let result_map = map.iter().fold(String::new(), |mut output, row| {
        row.iter().for_each(|c| {
            let _ = write!(output, "{}", c);
        });
        let _ = writeln!(output);
        output
    });
    println!("{}", result_map);
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Unknown,
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Unknown => Direction::Unknown,
        }
    }
}

#[derive(Debug)]
struct Step {
    direction: Direction,
    amount: u32,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Edge { start: Direction, end: Direction },
    Fill,
    Empty,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Edge { start, end } => match (start, end) {
                (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => {
                    write!(f, "┃")
                }
                (Direction::Up, Direction::Left) | (Direction::Left, Direction::Up) => {
                    write!(f, "┛")
                }
                (Direction::Up, Direction::Right) | (Direction::Right, Direction::Up) => {
                    write!(f, "┗")
                }
                (Direction::Down, Direction::Left) | (Direction::Left, Direction::Down) => {
                    write!(f, "┓")
                }
                (Direction::Down, Direction::Right) | (Direction::Right, Direction::Down) => {
                    write!(f, "┏")
                }
                (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => {
                    write!(f, "━")
                }
                _ => panic!("Invalid edge: {:?} {:?}", start, end),
            },
            Tile::Fill => write!(f, "#"),
            Tile::Empty => write!(f, "."),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn move_in_direction(&self, direction: &Direction, amount: i64) -> Position {
        match direction {
            Direction::Up => Position {
                x: self.x,
                y: self.y - amount,
            },
            Direction::Right => Position {
                x: self.x + amount,
                y: self.y,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + amount,
            },
            Direction::Left => Position {
                x: self.x - amount,
                y: self.y,
            },
            Direction::Unknown => panic!("Unknown direction"),
        }
    }
}

#[derive(Debug)]
struct Edge {
    start: Position,
    end: Position,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "62");
    }

    #[test]
    fn part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "952408144115");
    }
}
