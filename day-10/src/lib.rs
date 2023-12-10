use std::{collections::HashSet, fmt};

type TileMap = Vec<Vec<Tile>>;

pub fn process_part_1(input: &str) -> String {
    let (map, start) = parse_input(input);
    let length = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ]
    .iter()
    .filter_map(|starting_direction| {
        let mut position = start;
        let mut direction = *starting_direction;
        let mut length = 0;
        let mut done = false;
        while length < 100_000_000 {
            match next_pipe_and_direction(&position, &direction, &map) {
                Some((next_position, next_direction)) => {
                    position = next_position;
                    direction = next_direction;
                    length += 1;
                    if position == start {
                        done = true;
                        break;
                    }
                }
                None => break,
            }
        }
        if done {
            Some(length)
        } else {
            None
        }
    })
    .last()
    .unwrap();
    (length / 2).to_string()
}

pub fn process_part_2(input: &str) -> String {
    let (map, start) = parse_input(input);
    let (pipe_loop, start_tile) = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ]
    .iter()
    .filter_map(|starting_direction| {
        let mut position = start;
        let mut direction = *starting_direction;
        let mut length = 0;
        let mut done = false;
        let mut positions = HashSet::new();
        let mut start_tile = None;
        while length < 100_000_000 {
            match next_pipe_and_direction(&position, &direction, &map) {
                Some((next_position, next_direction)) => {
                    let prev_direction = direction;
                    position = next_position;
                    direction = next_direction;
                    length += 1;
                    positions.insert(position);
                    if position == start {
                        done = true;
                        start_tile =
                            Some(Tile::Pipe(*starting_direction, prev_direction.opposite()));
                        break;
                    }
                }
                None => break,
            }
        }
        if done {
            Some((positions, start_tile.unwrap()))
        } else {
            None
        }
    })
    .last()
    .unwrap();
    let mut inside_positions = vec![];
    for y in 0..map.len() as i32 {
        let mut entry_tile = None;
        let mut crossing_count = 0;
        for x in 0..map[y as usize].len() as i32 {
            let position = Position::new(x, y);
            let tile = &map[y as usize][x as usize];
            let tile = match tile {
                Tile::Start => &start_tile,
                _ => tile,
            };
            if !pipe_loop.contains(&position) {
                if is_odd(crossing_count) {
                    inside_positions.push(position);
                }
            } else if let Tile::Pipe(direction_1, direction_2) = tile {
                if direction_1 == &Direction::West && direction_2 == &Direction::East
                    || direction_1 == &Direction::East && direction_2 == &Direction::West
                {
                    continue;
                }
                if direction_1 == &Direction::North && direction_2 == &Direction::South
                    || direction_1 == &Direction::South && direction_2 == &Direction::North
                {
                    crossing_count += 1;
                    continue;
                }
                match entry_tile {
                    None => {
                        entry_tile = Some(*tile);
                        crossing_count += 1;
                    }
                    Some(Tile::Pipe(entry_tile_direction_1, entry_tile_direction_2)) => {
                        let directions = [
                            direction_1,
                            direction_2,
                            &entry_tile_direction_1,
                            &entry_tile_direction_2,
                        ];
                        if !directions.contains(&&Direction::North)
                            || !directions.contains(&&Direction::South)
                        {
                            crossing_count += 1;
                        }
                        entry_tile = None;
                    }
                    _ => panic!("Invalid entry tile: {:?}", entry_tile),
                }
            }
        }
    }

    for (y, line) in map.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            let position = Position::new(x as i32, y as i32);
            if pipe_loop.contains(&position) {
                print!("{}", tile);
            } else if inside_positions.contains(&position) {
                print!("I");
            } else {
                print!("O");
            }
        }
        println!();
    }

    inside_positions.len().to_string()
}

fn next_pipe_and_direction(
    position: &Position,
    direction: &Direction,
    map: &TileMap,
) -> Option<(Position, Direction)> {
    let next_position = direction.next_position(position);
    let next_tile = map
        .get(next_position.y as usize)?
        .get(next_position.x as usize)?;
    match next_tile {
        Tile::Pipe(direction_1, direction_2) => {
            let opposite = direction.opposite();
            if direction_1 == &opposite {
                Some((next_position, *direction_2))
            } else if direction_2 == &opposite {
                Some((next_position, *direction_1))
            } else {
                None
            }
        }
        Tile::Start => Some((next_position, *direction)),
        _ => None,
    }
}

fn is_odd(num: u32) -> bool {
    num % 2 != 0
}

fn parse_input(input: &str) -> (Vec<Vec<Tile>>, Position) {
    let mut start: Option<Position> = None;
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '|' => Tile::Pipe(Direction::North, Direction::South),
                    '-' => Tile::Pipe(Direction::East, Direction::West),
                    'L' => Tile::Pipe(Direction::North, Direction::East),
                    'J' => Tile::Pipe(Direction::North, Direction::West),
                    '7' => Tile::Pipe(Direction::South, Direction::West),
                    'F' => Tile::Pipe(Direction::South, Direction::East),
                    '.' => Tile::Ground,
                    'S' => {
                        start = Some(Position {
                            x: x as i32,
                            y: y as i32,
                        });
                        Tile::Start
                    }
                    _ => panic!("Invalid character at ({}, {}): {}", x, y, c),
                })
                .collect()
        })
        .collect();
    (map, start.expect("No start found"))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    fn next_position(&self, position: &Position) -> Position {
        match self {
            Direction::North => Position {
                x: position.x,
                y: position.y - 1,
            },
            Direction::South => Position {
                x: position.x,
                y: position.y + 1,
            },
            Direction::East => Position {
                x: position.x + 1,
                y: position.y,
            },
            Direction::West => Position {
                x: position.x - 1,
                y: position.y,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Pipe(Direction, Direction),
    Ground,
    Start,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Tile::Pipe(direction_1, direction_2) => match (direction_1, direction_2) {
                (Direction::North, Direction::South) | (Direction::South, Direction::North) => "|",
                (Direction::North, Direction::East) | (Direction::East, Direction::North) => "L",
                (Direction::North, Direction::West) | (Direction::West, Direction::North) => "J",
                (Direction::East, Direction::South) | (Direction::South, Direction::East) => "F",
                (Direction::West, Direction::South) | (Direction::South, Direction::West) => "7",
                (Direction::West, Direction::East) | (Direction::East, Direction::West) => "-",
                _ => "?",
            },
            Tile::Ground => ".",
            Tile::Start => "S",
        };
        write!(f, "{}", c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const INPUT_1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const INPUT_2: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    const INPUT_3: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const INPUT_4: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    const INPUT_5: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const INPUT_6: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

    const INPUT_7: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const INPUT_8: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[rstest]
    #[case(INPUT_1, "4")]
    #[case(INPUT_2, "4")]
    #[case(INPUT_3, "8")]
    #[case(INPUT_4, "8")]
    fn part_1_works(#[case] input: &str, #[case] expected: &str) {
        let result = process_part_1(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(INPUT_5, "4")]
    #[case(INPUT_6, "4")]
    #[case(INPUT_7, "8")]
    #[case(INPUT_8, "10")]
    fn part_2_works(#[case] input: &str, #[case] expected: &str) {
        let result = process_part_2(input);
        assert_eq!(result, expected);
    }
}
