use std::{
    collections::HashSet,
    fmt::Display,
    fs::{self},
};

use itertools::Itertools;
use petgraph::{algo::all_simple_paths, dot::Dot, Graph};

pub fn process_part_1(input: &str) -> String {
    let map = parse_input(input);
    let start = start_position(&map);
    let end = end_position(&map);
    let longest_path = visit(&map, start, end);
    (longest_path.len() - 1).to_string()
}

pub fn process_part_2(input: &str) -> String {
    let map = parse_input(input);
    let map = map
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|tile| match tile {
                    Tile::Slope { direction: _ } => Tile::Path,
                    _ => tile,
                })
                .collect()
        })
        .collect::<Vec<Vec<Tile>>>();

    let start = start_position(&map);
    let end = end_position(&map);
    let reduced = reduce(&map, start, end);

    let graph = make_graph(&reduced);
    let dot = Dot::new(&graph);
    fs::write("./graph.dot", format!("{}", dot)).unwrap();

    let start_node = graph.node_indices().find(|i| graph[*i] == start).unwrap();
    let end_node = graph.node_indices().find(|i| graph[*i] == end).unwrap();
    all_simple_paths::<Vec<_>, _>(&graph, start_node, end_node, 0, None)
        .map(|path| {
            path.iter().tuple_windows().fold(0, |acc, (from, to)| {
                let cost = graph
                    .find_edge(*from, *to)
                    .and_then(|edge| graph.edge_weight(edge))
                    .copied()
                    .unwrap_or_default();
                acc + cost
            })
        })
        .max()
        .unwrap()
        .to_string()
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Tile::Forest,
                    '.' => Tile::Path,
                    '>' => Tile::Slope {
                        direction: Direction::East,
                    },
                    '<' => Tile::Slope {
                        direction: Direction::West,
                    },
                    '^' => Tile::Slope {
                        direction: Direction::North,
                    },
                    'v' => Tile::Slope {
                        direction: Direction::South,
                    },
                    _ => panic!("Invalid character in input: {}", c),
                })
                .collect()
        })
        .collect()
}

fn start_position(map: &[Vec<Tile>]) -> Position {
    map[0]
        .iter()
        .enumerate()
        .find_map(|(x, tile)| {
            if matches!(tile, Tile::Path) {
                Some(Position { x: x as i32, y: 0 })
            } else {
                None
            }
        })
        .unwrap()
}

fn end_position(map: &[Vec<Tile>]) -> Position {
    map[map.len() - 1]
        .iter()
        .enumerate()
        .find_map(|(x, tile)| {
            if matches!(tile, Tile::Path) {
                Some(Position {
                    x: x as i32,
                    y: (map.len() - 1) as i32,
                })
            } else {
                None
            }
        })
        .unwrap()
}

fn visit(map: &[Vec<Tile>], start_position: Position, end_position: Position) -> HashSet<Position> {
    let mut max_path = HashSet::new();
    let mut start_path = HashSet::new();
    start_path.insert(start_position);
    let start = (start_position, start_path);
    let mut to_see = vec![start];
    while let Some(next) = to_see.pop() {
        let (position, path) = next;
        let next_steps = position.next_steps(map);
        if position == end_position {
            if path.len() > max_path.len() {
                // println!("New longest path: {}", path.len() - 1);
                max_path = path;
            }
        } else {
            for neighbor_position in next_steps {
                if !path.contains(&neighbor_position) {
                    let mut new_path = path.clone();
                    new_path.insert(neighbor_position);
                    let neighbor = (neighbor_position, new_path);
                    to_see.push(neighbor);
                }
            }
        }
    }
    max_path
}

fn reduce(
    map: &[Vec<Tile>],
    start_position: Position,
    end_position: Position,
) -> Vec<(Position, Position, i32)> {
    let mut intersections = HashSet::new();
    intersections.insert(start_position);
    intersections.insert(end_position);
    let mut graph = vec![];
    let mut start_path = HashSet::new();
    start_path.insert(start_position);
    let mut to_see = vec![(start_position, start_path, start_position, 0)];
    while let Some((next, path, prev_intersection, length)) = to_see.pop() {
        let next_steps = next
            .next_steps(map)
            .into_iter()
            .filter(|step| !path.contains(step))
            .collect::<Vec<_>>();
        let intersection = next_steps.len() > 1 || next == end_position;
        let existing_intersection = intersections.contains(&next);
        if intersection {
            if !existing_intersection {
                intersections.insert(next);
            }
            graph.push((prev_intersection, next, length));
        }
        if existing_intersection && next != start_position && next != end_position {
            continue;
        }
        for neighbor_position in next_steps {
            let mut new_path = path.clone();
            new_path.insert(neighbor_position);
            let prev_intersection = if intersection {
                next
            } else {
                prev_intersection
            };
            let length = if intersection { 1 } else { length + 1 };
            let neighbor = (neighbor_position, new_path, prev_intersection, length);
            to_see.push(neighbor);
        }
    }
    graph
}

fn make_graph(
    reduced: &[(Position, Position, i32)],
) -> Graph<Position, i32, petgraph::prelude::Undirected> {
    let mut graph = Graph::new_undirected();
    let nodes = reduced
        .iter()
        .flat_map(|(from, to, _)| vec![from, to])
        .collect::<HashSet<_>>();
    nodes.iter().for_each(|node| {
        graph.add_node(**node);
    });
    reduced.iter().for_each(|(from, to, weight)| {
        let a = graph.node_indices().find(|i| graph[*i] == *from).unwrap();
        let b = graph.node_indices().find(|i| graph[*i] == *to).unwrap();
        graph.add_edge(a, b, *weight);
    });
    graph
}

#[derive(Debug)]
enum Tile {
    Path,
    Forest,
    Slope { direction: Direction },
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn next_steps(&self, map: &[Vec<Tile>]) -> Vec<Position> {
        [
            (
                Position {
                    x: self.x + 1,
                    y: self.y,
                },
                Direction::East,
            ),
            (
                Position {
                    x: self.x - 1,
                    y: self.y,
                },
                Direction::West,
            ),
            (
                Position {
                    x: self.x,
                    y: self.y + 1,
                },
                Direction::South,
            ),
            (
                Position {
                    x: self.x,
                    y: self.y - 1,
                },
                Direction::North,
            ),
        ]
        .iter()
        .filter_map(|(position, direction)| {
            if position.x < 0 || position.y < 0 {
                None
            } else {
                match map.get(position.y as usize).and_then(|row| {
                    if position.x as usize >= row.len() {
                        None
                    } else {
                        row.get(position.x as usize)
                    }
                }) {
                    Some(Tile::Path) => Some(*position),
                    Some(Tile::Slope {
                        direction: slope_direction,
                    }) => {
                        if slope_direction == direction {
                            Some(*position)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
        })
        .collect()
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "94");
    }

    #[test]
    fn part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "154");
    }
}
