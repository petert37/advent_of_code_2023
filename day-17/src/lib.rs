use std::fmt::Display;

use dijkstra::{path_n, FxIndexMap};

use crate::dijkstra::dijkstra;

mod dijkstra;

type PathItem = (Position, Direction, Vec<Direction>);

pub fn process_part_1(input: &str) -> String {
    let map = parse_input(input);
    let start = (Position { x: -1, y: -1 }, Direction::North, vec![]);
    let target_position = Position {
        x: map[0].len() as i32 - 1,
        y: map.len() as i32 - 1,
    };

    let (path, cost) = dijkstra(
        &start,
        |(position, direction, _), path, index| position.successors(&map, path, index, direction),
        |(position, _, _)| *position == target_position,
    )
    .unwrap();

    let mut map = map
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| format!("{}", c))
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();
    path.iter().skip(1).for_each(|(position, direction, _)| {
        map[position.y as usize][position.x as usize] = format!("{}", direction);
    });
    let result_map = map
        .iter()
        .map(|row| row.join(""))
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", result_map);

    cost.to_string()
}

pub fn process_part_2(input: &str) -> String {
    let map = parse_input(input);
    let start = (Position { x: -1, y: -1 }, Direction::North, vec![]);
    let target_position = Position {
        x: map[0].len() as i32 - 1,
        y: map.len() as i32 - 1,
    };

    let (path, cost) = dijkstra(
        &start,
        |(position, direction, _), path, index| {
            position.successors_part_2(&map, path, index, direction)
        },
        |(position, direction, path)| {
            *position == target_position && {
                let path_end = path.iter().rev().take(4).collect::<Vec<_>>();
                if path_end.len() == 4 {
                    path_end.iter().all(|d| *d == direction)
                } else {
                    false
                }
            }
        },
    )
    .unwrap();

    let mut map = map
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| format!("{}", c))
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();
    path.iter().skip(1).for_each(|(position, direction, _)| {
        map[position.y as usize][position.x as usize] = format!("{}", direction);
    });
    let result_map = map
        .iter()
        .map(|row| row.join(""))
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", result_map);

    cost.to_string()
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

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

    fn successors(
        &self,
        map: &[Vec<u32>],
        path: &FxIndexMap<PathItem, (usize, u32)>,
        index: usize,
        direction: &Direction,
    ) -> Vec<(PathItem, u32)> {
        if self.x == -1 && self.y == -1 {
            return vec![
                ((Position::new(0, 0), Direction::East, vec![]), 0),
                ((Position::new(0, 0), Direction::South, vec![]), 0),
            ];
        }
        let path: Vec<PathItem> = path_n(path, |&(p, _)| p, index, 0, 3);
        let directions = if path.is_empty() {
            vec![
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]
        } else {
            let continue_straight = if path.len() == 3 {
                !path.iter().all(|(_, d, _)| d == direction)
            } else {
                true
            };
            if continue_straight {
                vec![*direction, direction.turn_right(), direction.turn_left()]
            } else {
                vec![direction.turn_right(), direction.turn_left()]
            }
        };
        directions
            .iter()
            .map(|d| (self.move_in_direction(d), d))
            .filter(|(p, _)| p.x >= 0 && p.y >= 0)
            .filter_map(|(p, d)| {
                let cost = map.get(p.y as usize).and_then(|row| row.get(p.x as usize));
                let prev_directions = path
                    .iter()
                    .map(|(_, d, _)| d)
                    .rev()
                    .cloned()
                    .collect::<Vec<_>>();
                cost.map(|cost| ((p, *d, prev_directions), *cost))
            })
            .collect()
    }

    fn successors_part_2(
        &self,
        map: &[Vec<u32>],
        path: &FxIndexMap<PathItem, (usize, u32)>,
        index: usize,
        direction: &Direction,
    ) -> Vec<(PathItem, u32)> {
        if self.x == -1 && self.y == -1 {
            return vec![
                ((Position::new(0, 0), Direction::East, vec![]), 0),
                ((Position::new(0, 0), Direction::South, vec![]), 0),
            ];
        }
        let path: Vec<PathItem> = path_n(path, |&(p, _)| p, index, 0, 10);
        let directions = if path.is_empty() {
            vec![
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]
        } else {
            let mut directions = vec![];
            let continue_straight = if path.len() == 10 {
                !path.iter().all(|(_, d, _)| d == direction)
            } else {
                true
            };
            let turn = if path.len() >= 4 {
                path.iter().take(4).all(|(_, d, _)| d == direction)
            } else {
                false
            };
            if continue_straight {
                directions.push(*direction);
            }
            if turn {
                directions.push(direction.turn_right());
                directions.push(direction.turn_left());
            }
            directions
        };
        directions
            .iter()
            .map(|d| (self.move_in_direction(d), d))
            .filter(|(p, _)| p.x >= 0 && p.y >= 0)
            .filter_map(|(p, d)| {
                let cost = map.get(p.y as usize).and_then(|row| row.get(p.x as usize));
                let prev_directions = path
                    .iter()
                    .map(|(_, d, _)| d)
                    .rev()
                    .cloned()
                    .collect::<Vec<_>>();
                cost.map(|cost| ((p, *d, prev_directions), *cost))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "102");
    }

    #[test]
    fn part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "94");
    }
}
