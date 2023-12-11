use std::ops::Not;

pub fn process_part_1(input: &str) -> String {
    process(input, 2)
}

pub fn process_part_2(input: &str) -> String {
    process(input, 1_000_000)
}

fn process(input: &str, expansion: i64) -> String {
    let start_map = parse_input(input);
    let expanded_star_map = expand_star_map(&start_map, expansion);
    expanded_star_map
        .iter()
        .enumerate()
        .map(|(i, star)| {
            expanded_star_map
                .iter()
                .skip(i + 1)
                .map(|other_star| (star.x - other_star.x).abs() + (star.y - other_star.y).abs())
                .sum::<i64>()
        })
        .sum::<i64>()
        .to_string()
}

fn parse_input(input: &str) -> Vec<Position> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Position::new(x as i64, y as i64))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn expand_star_map(star_map: &[Position], expansion: i64) -> Vec<Position> {
    let min_x = star_map.iter().min_by(|p1, p2| p1.x.cmp(&p2.x)).unwrap().x;
    let max_x = star_map.iter().max_by(|p1, p2| p1.x.cmp(&p2.x)).unwrap().x;
    let min_y = star_map.iter().min_by(|p1, p2| p1.y.cmp(&p2.y)).unwrap().y;
    let max_y = star_map.iter().max_by(|p1, p2| p1.y.cmp(&p2.y)).unwrap().y;
    let empty_x = (min_x..=max_x)
        .filter(|x| star_map.iter().any(|p| p.x == *x).not())
        .collect::<Vec<i64>>();
    let empty_y = (min_y..=max_y)
        .filter(|y| star_map.iter().any(|p| p.y == *y).not())
        .collect::<Vec<i64>>();
    star_map
        .iter()
        .map(|p| {
            let dx = empty_x.iter().filter(|x| x < &&p.x).count() as i64;
            let dy = empty_y.iter().filter(|y| y < &&p.y).count() as i64;
            let dx = dx * expansion - dx;
            let dy = dy * expansion - dy;
            Position::new(p.x + dx, p.y + dy)
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "374");
    }

    #[rstest]
    #[case(INPUT, 10, "1030")]
    #[case(INPUT, 100, "8410")]
    fn part_2_works(#[case] input: &str, #[case] expansion: i64, #[case] expected: &str) {
        let result = process(input, expansion);
        assert_eq!(result, expected);
    }
}
