use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult, Parser,
};

pub fn process_part_1(input: &str) -> String {
    let (input, patterns) = parse_input(input).unwrap();
    debug_assert_eq!(input, "");
    patterns
        .iter()
        .map(|pattern| {
            let vertical = (0..pattern.pattern[0].len() as i32)
                .filter(|&i| is_vertical_mirror(pattern, i))
                .sum::<i32>();
            let horizontal = (0..pattern.pattern.len() as i32)
                .filter(|&i| is_horizontal_mirror(pattern, i))
                .map(|i| i * 100)
                .sum::<i32>();
            vertical + horizontal
        })
        .sum::<i32>()
        .to_string()
}

pub fn process_part_2(input: &str) -> String {
    let (input, patterns) = parse_input(input).unwrap();
    debug_assert_eq!(input, "");
    patterns
        .iter()
        .map(|pattern| {
            let vertical = (0..pattern.pattern[0].len() as i32)
                .filter(|&i| is_smudged_vertical_mirror(pattern, i))
                .sum::<i32>();
            let horizontal = (0..pattern.pattern.len() as i32)
                .filter(|&i| is_smudged_horizontal_mirror(pattern, i))
                .map(|i| i * 100)
                .sum::<i32>();
            vertical + horizontal
        })
        .sum::<i32>()
        .to_string()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Pattern>> {
    separated_list1(tuple((line_ending, line_ending)), parse_pattern)(input)
}

fn parse_pattern(input: &str) -> IResult<&str, Pattern> {
    separated_list1(
        line_ending,
        many1(alt((
            tag("#").map(|_| Tile::Rock),
            tag(".").map(|_| Tile::Ash),
        ))),
    )
    .map(|pattern| Pattern { pattern })
    .parse(input)
}

fn is_horizontal_mirror(pattern: &Pattern, mirror_location: i32) -> bool {
    if mirror_location <= 0 || mirror_location >= pattern.pattern.len() as i32 {
        return false;
    }
    let mut is_mirror = true;
    let mut top_side = mirror_location - 1;
    let mut bottom_side = mirror_location;
    while top_side >= 0 && bottom_side < pattern.pattern.len() as i32 {
        let top = &pattern.pattern[top_side as usize];
        let bottom = &pattern.pattern[bottom_side as usize];
        if top != bottom {
            is_mirror = false;
            break;
        }
        top_side -= 1;
        bottom_side += 1;
    }
    return is_mirror;
}

fn is_vertical_mirror(pattern: &Pattern, mirror_location: i32) -> bool {
    if mirror_location <= 0 || mirror_location >= pattern.pattern[0].len() as i32 {
        return false;
    }
    let mut is_mirror = true;
    let mut left_side = mirror_location - 1;
    let mut right_side = mirror_location;
    while left_side >= 0 && right_side < pattern.pattern[0].len() as i32 {
        let left = pattern
            .pattern
            .iter()
            .map(|row| &row[left_side as usize])
            .collect::<Vec<_>>();
        let right = pattern
            .pattern
            .iter()
            .map(|row| &row[right_side as usize])
            .collect::<Vec<_>>();
        if left != right {
            is_mirror = false;
            break;
        }
        left_side -= 1;
        right_side += 1;
    }
    return is_mirror;
}

fn is_smudged_horizontal_mirror(pattern: &Pattern, mirror_location: i32) -> bool {
    if mirror_location <= 0 || mirror_location >= pattern.pattern.len() as i32 {
        return false;
    }
    let mut top_side = mirror_location - 1;
    let mut bottom_side = mirror_location;
    let mut diffs = 0;
    while top_side >= 0 && bottom_side < pattern.pattern.len() as i32 {
        let top = &pattern.pattern[top_side as usize];
        let bottom = &pattern.pattern[bottom_side as usize];
        diffs += top
            .iter()
            .zip(bottom.iter())
            .filter(|(t, b)| t != b)
            .count();
        top_side -= 1;
        bottom_side += 1;
    }
    return diffs == 1;
}

fn is_smudged_vertical_mirror(pattern: &Pattern, mirror_location: i32) -> bool {
    if mirror_location <= 0 || mirror_location >= pattern.pattern[0].len() as i32 {
        return false;
    }
    let mut left_side = mirror_location - 1;
    let mut right_side = mirror_location;
    let mut diffs = 0;
    while left_side >= 0 && right_side < pattern.pattern[0].len() as i32 {
        let left = pattern
            .pattern
            .iter()
            .map(|row| &row[left_side as usize])
            .collect::<Vec<_>>();
        let right = pattern
            .pattern
            .iter()
            .map(|row| &row[right_side as usize])
            .collect::<Vec<_>>();
        diffs += left
            .iter()
            .zip(right.iter())
            .filter(|(l, r)| l != r)
            .count();

        left_side -= 1;
        right_side += 1;
    }
    return diffs == 1;
}

#[derive(Debug)]
struct Pattern {
    pattern: Vec<Vec<Tile>>,
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "405");
    }

    #[test]
    fn part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "400");
    }
}
