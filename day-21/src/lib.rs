use std::collections::{HashMap, HashSet};

use pathfinding::directed::dijkstra::dijkstra_all;

pub fn process_part_1(input: &str, steps: i32) -> String {
    let (start, map) = parse_input(input);
    visit(&map, start, steps).len().to_string()
}

// https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
pub fn process_part_2(input: &str, steps: i32) -> String {
    let (start, map) = parse_input(input);

    let height = map.len() as i32;
    let width = map[0].len() as i32;
    let shortest_distances = shortest_distances(&map, start);

    // shortest_distances is a HashMap<Coord, usize> which maps tiles in the input-square to their distance from the starting tile
    // So read this as "even_corners is the number of tiles which have a distance that is even and greater than 65"
    let even_corners = shortest_distances
        .values()
        .filter(|v| **v % 2 == 0 && **v > 65)
        .count();
    let odd_corners = shortest_distances
        .values()
        .filter(|v| **v % 2 == 1 && **v > 65)
        .count();

    let even_full = shortest_distances.values().filter(|v| **v % 2 == 0).count();
    let odd_full = shortest_distances.values().filter(|v| **v % 2 == 1).count();

    // This is 202300 but im writing it out here to show the process
    let n = ((steps - (width / 2)) / height) as usize;
    assert_eq!(n, 202300);

    let p2 = ((n + 1) * (n + 1)) * odd_full + (n * n) * even_full - (n + 1) * odd_corners
        + n * even_corners;
    p2.to_string()
}

fn parse_input(input: &str) -> (Position, Vec<Vec<Tile>>) {
    let mut start = Position { x: 0, y: 0 };
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Tile::GardenPlot,
                    '#' => Tile::Rock,
                    'S' => {
                        start = Position {
                            x: x as i32,
                            y: y as i32,
                        };
                        Tile::GardenPlot
                    }
                    _ => panic!("Unexpected character: {}", c),
                })
                .collect()
        })
        .collect();
    (start, map)
}

fn visit(map: &[Vec<Tile>], start: Position, max_depth: i32) -> HashSet<Position> {
    let start = (start, 0);
    let mut seen = HashSet::new();
    seen.insert(start);
    let mut to_see = vec![start];
    while let Some(next) = to_see.pop() {
        let (position, depth) = next;
        if depth >= max_depth {
            continue;
        }
        for neighbor in position.neighbors() {
            if let Some(Tile::GardenPlot) = map
                .get(neighbor.y as usize)
                .and_then(|row| row.get(neighbor.x as usize))
            {
                let neighbor = (neighbor, depth + 1);
                if !seen.contains(&neighbor) {
                    seen.insert(neighbor);
                    to_see.push(neighbor);
                }
            }
        }
    }
    seen.into_iter()
        .filter_map(|(position, depth)| {
            if depth == max_depth {
                Some(position)
            } else {
                None
            }
        })
        .collect()
}

fn shortest_distances(map: &[Vec<Tile>], start: Position) -> HashMap<Position, u64> {
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    dijkstra_all(&start, |position| {
        position
            .neighbors()
            .into_iter()
            .filter(|position: &Position| {
                position.x >= 0 && position.y >= 0 && position.x < width && position.y < height
            })
            .filter_map(|neighbor| {
                if let Some(Tile::GardenPlot) = map
                    .get(neighbor.y as usize)
                    .and_then(|row| row.get(neighbor.x as usize))
                {
                    Some((neighbor, 1))
                } else {
                    None
                }
            })
    })
    .into_iter()
    .map(|(position, (_, distance))| (position, distance as u64))
    .chain([(start, 0)])
    .collect()
}

enum Tile {
    GardenPlot,
    Rock,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn neighbors(&self) -> Vec<Position> {
        vec![
            Position {
                x: self.x - 1,
                y: self.y,
            },
            Position {
                x: self.x + 1,
                y: self.y,
            },
            Position {
                x: self.x,
                y: self.y - 1,
            },
            Position {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT, 6);
        assert_eq!(result, "16");
    }
}
