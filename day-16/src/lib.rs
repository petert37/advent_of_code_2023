use std::{
    collections::HashSet,
    fmt::Display,
    sync::{Arc, RwLock},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    multi::{many1, separated_list1},
    IResult, Parser,
};
use rayon::prelude::*;
use rayon::Scope;

type BeamState = Vec<Vec<HashSet<Direction>>>;

pub fn process_part_1(input: &str) -> String {
    let (input, map) = parse_input(input).unwrap();
    debug_assert_eq!(input, "");
    let map = Arc::new(map);
    let width = map[0].len();
    let height = map.len();
    let stating_position = Position { x: 0, y: 0 };
    let stating_direction = Direction::East;

    let state = start_beam(
        map.clone(),
        width,
        height,
        stating_position,
        stating_direction,
    );

    map.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, tile)| {
            let state = &state[y][x];
            if matches!(tile, Tile::Space) && !state.is_empty() {
                let len = state.len();
                if len == 1 {
                    print!("{}", state.iter().next().unwrap());
                } else {
                    print!("{}", len);
                }
            } else {
                print!("{}", tile);
            }
        });
        println!();
    });

    count_energized_tiles(&state).to_string()
}

pub fn process_part_2(input: &str) -> String {
    let (input, map) = parse_input(input).unwrap();
    debug_assert_eq!(input, "");
    let map = Arc::new(map);
    let width = map[0].len();
    let height = map.len();

    [
        (
            Direction::South,
            (0..width).map(|x| (x, 0)).collect::<Vec<_>>(),
        ),
        (
            Direction::West,
            (0..height).map(|y| (width - 1, y)).collect::<Vec<_>>(),
        ),
        (
            Direction::North,
            (0..width).map(|x| (x, height - 1)).collect::<Vec<_>>(),
        ),
        (
            Direction::East,
            (0..height).map(|y| (0, y)).collect::<Vec<_>>(),
        ),
    ]
    .iter()
    .flat_map(|(direction, start_positions)| {
        start_positions
            .par_iter()
            .map(|(x, y)| {
                let starting_position = Position {
                    x: *x as i32,
                    y: *y as i32,
                };
                let state = start_beam(map.clone(), width, height, starting_position, *direction);
                count_energized_tiles(&state)
            })
            .collect::<Vec<_>>()
    })
    .max()
    .unwrap()
    .to_string()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list1(
        line_ending,
        many1(alt((
            tag(".").map(|_| Tile::Space),
            tag("-").map(|_| Tile::HorizontalSplitter),
            tag("|").map(|_| Tile::VerticalSplitter),
            tag("\\").map(|_| Tile::LeftMirror),
            tag("/").map(|_| Tile::RightMirror),
        ))),
    )(input)
}

fn start_beam(
    map: Arc<Vec<Vec<Tile>>>,
    width: usize,
    height: usize,
    stating_position: Position,
    stating_direction: Direction,
) -> BeamState {
    let state = Arc::new(RwLock::new(vec![vec![HashSet::new(); width]; height]));
    rayon::scope(|scope| {
        beam(
            scope,
            map,
            width as i32,
            height as i32,
            state.clone(),
            stating_position,
            stating_direction,
        );
    });
    Arc::try_unwrap(state).unwrap().into_inner().unwrap()
}

fn next_positions(
    position: &Position,
    direction: &Direction,
    map: &[Vec<Tile>],
) -> ((Position, Direction), Option<(Position, Direction)>) {
    let tile = &map[position.y as usize][position.x as usize];
    match tile {
        Tile::Space => ((position.move_in_direction(direction), *direction), None),
        Tile::HorizontalSplitter => match direction {
            Direction::West | Direction::East => {
                ((position.move_in_direction(direction), *direction), None)
            }
            Direction::North | Direction::South => (
                (
                    position.move_in_direction(&Direction::East),
                    Direction::East,
                ),
                Some((
                    position.move_in_direction(&Direction::West),
                    Direction::West,
                )),
            ),
        },
        Tile::VerticalSplitter => match direction {
            Direction::North | Direction::South => {
                ((position.move_in_direction(direction), *direction), None)
            }
            Direction::West | Direction::East => (
                (
                    position.move_in_direction(&Direction::South),
                    Direction::South,
                ),
                Some((
                    position.move_in_direction(&Direction::North),
                    Direction::North,
                )),
            ),
        },
        Tile::LeftMirror => {
            let next_direction = match direction {
                Direction::North | Direction::South => direction.turn_left(),
                Direction::West | Direction::East => direction.turn_right(),
            };
            (
                (position.move_in_direction(&next_direction), next_direction),
                None,
            )
        }
        Tile::RightMirror => {
            let next_direction = match direction {
                Direction::North | Direction::South => direction.turn_right(),
                Direction::West | Direction::East => direction.turn_left(),
            };
            (
                (position.move_in_direction(&next_direction), next_direction),
                None,
            )
        }
    }
}

fn beam(
    scope: &Scope,
    map: Arc<Vec<Vec<Tile>>>,
    width: i32,
    height: i32,
    state: Arc<RwLock<BeamState>>,
    position: Position,
    direction: Direction,
) {
    let mut position = position;
    let mut direction = direction;
    loop {
        if position.x >= width || position.x < 0 || position.y >= height || position.y < 0 {
            return;
        }
        if state.read().unwrap()[position.y as usize][position.x as usize].contains(&direction) {
            return;
        }
        state.write().unwrap()[position.y as usize][position.x as usize].insert(direction);
        let ((next_position, next_direction), split_next_position) =
            next_positions(&position, &direction, &map);
        position = next_position;
        direction = next_direction;
        if let Some((split_next_position, split_next_direction)) = split_next_position {
            let state = state.clone();
            let map = map.clone();
            scope.spawn(move |scope| {
                beam(
                    scope,
                    map,
                    width,
                    height,
                    state,
                    split_next_position,
                    split_next_direction,
                );
            });
        }
    }
}

fn count_energized_tiles(state: &BeamState) -> usize {
    state
        .iter()
        .map(|line| line.iter().filter(|position| !position.is_empty()).count())
        .sum()
}

enum Tile {
    Space,
    HorizontalSplitter,
    VerticalSplitter,
    LeftMirror,
    RightMirror,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Space => write!(f, "."),
            Tile::HorizontalSplitter => write!(f, "-"),
            Tile::VerticalSplitter => write!(f, "|"),
            Tile::LeftMirror => write!(f, "\\"),
            Tile::RightMirror => write!(f, "/"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "^"),
            Direction::East => write!(f, ">"),
            Direction::South => write!(f, "v"),
            Direction::West => write!(f, "<"),
        }
    }
}

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn move_in_direction(&self, direction: &Direction) -> Position {
        match direction {
            Direction::North => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::East => Position {
                x: self.x + 1,
                y: self.y,
            },
            Direction::South => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Position {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "46");
    }

    #[test]
    fn part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "51");
    }
}
