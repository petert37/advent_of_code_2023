use std::collections::{HashMap, HashSet};

use indicatif::ParallelProgressIterator;
use nom::character::complete::{self, line_ending};
use nom::sequence::separated_pair;
use nom::{bytes::complete::tag, multi::separated_list1, IResult};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use uuid::Uuid;

pub fn process_part_1(input: &str) -> String {
    let (input, bricks) = parse_input(input).unwrap();
    debug_assert_eq!(input, "");
    let settled = settle(&bricks);
    settled
        .par_iter()
        .progress_count(settled.len() as u64)
        .filter(|brick_to_remove| {
            let removed = settled
                .iter()
                .filter(|other_brick| other_brick != brick_to_remove)
                .copied()
                .collect::<Vec<_>>();
            removed
                .iter()
                .all(|other_brick| other_brick.is_supported(&removed))
        })
        .count()
        .to_string()
}

pub fn process_part_2(input: &str) -> String {
    let (input, bricks) = parse_input(input).unwrap();
    debug_assert_eq!(input, "");
    let settled = settle(&bricks);
    settled
        .par_iter()
        .map(|brick_to_remove| {
            let removed = settled
                .iter()
                .filter(|other_brick| *other_brick != brick_to_remove)
                .copied()
                .collect::<Vec<_>>();
            let removed_settled = settle(&removed)
                .into_iter()
                .map(|brick| (brick.id, brick))
                .collect::<HashMap<_, _>>();
            removed
                .iter()
                .filter(|brick| {
                    let removed_settled_brick = removed_settled.get(&brick.id).unwrap();
                    brick.start != removed_settled_brick.start
                        || brick.end != removed_settled_brick.end
                })
                .count()
        })
        .progress_count(settled.len() as u64)
        .sum::<usize>()
        .to_string()
}

fn settle(bricks: &[Brick]) -> Vec<Brick> {
    let mut bricks = bricks.to_owned();
    let mut moved = true;
    while moved {
        moved = false;
        let not_supported_brick_ids = bricks
            .iter()
            .filter_map(|brick| {
                if brick.is_supported(&bricks) {
                    None
                } else {
                    Some(brick.id)
                }
            })
            .collect::<HashSet<_>>();
        if !not_supported_brick_ids.is_empty() {
            moved = true;
            bricks.iter_mut().for_each(|brick| {
                if not_supported_brick_ids.contains(&brick.id) {
                    brick.move_down();
                }
            });
        }
    }
    bricks
}

fn parse_input(input: &str) -> IResult<&str, Vec<Brick>> {
    separated_list1(line_ending, parse_brick)(input)
}

fn parse_brick(input: &str) -> IResult<&str, Brick> {
    let (input, (start, end)) = separated_pair(parse_position, tag("~"), parse_position)(input)?;
    Ok((input, Brick::new(start, end)))
}

fn parse_position(input: &str) -> IResult<&str, Position> {
    let (input, x) = complete::i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = complete::i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, z) = complete::i32(input)?;
    Ok((input, Position { x, y, z }))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Brick {
    id: Uuid,
    start: Position,
    end: Position,
}

impl Brick {
    fn new(start: Position, end: Position) -> Self {
        Self {
            id: Uuid::new_v4(),
            start,
            end,
        }
    }

    fn is_supported(&self, bricks: &[Brick]) -> bool {
        self.start.z == 1
            || self.end.z == 1
            || bricks
                .iter()
                .filter(|b| *b != self)
                .any(|b| self.is_supported_by(b))
    }

    fn is_supported_by(&self, brick: &Brick) -> bool {
        let cubes = self.cubes();
        let other_cubes = brick.cubes();
        cubes.iter().any(|cube| {
            let moved_cube = cube.moved_down();
            other_cubes
                .iter()
                .any(|other_cube| &moved_cube == other_cube)
        })
    }

    fn move_down(&mut self) {
        self.start.move_down();
        self.end.move_down();
    }

    fn cubes(&self) -> Vec<Position> {
        let mut cubes = Vec::new();
        for x in self.start.x..=self.end.x {
            for y in self.start.y..=self.end.y {
                for z in self.start.z..=self.end.z {
                    cubes.push(Position { x, y, z });
                }
            }
        }
        cubes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    fn move_down(&mut self) {
        self.z -= 1;
    }

    fn moved_down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z - 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "5");
    }

    #[test]
    fn part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "7");
    }
}
