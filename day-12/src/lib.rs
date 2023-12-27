use itertools::{repeat_n, Itertools};
use memoize::memoize;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};
use rayon::prelude::*;

pub fn process_part_1(input: &str) -> String {
    let (_, rows) = parse_input(input).unwrap();
    let options = vec![SpringState::Operational, SpringState::Damaged];
    rows.par_iter()
        .map(|row| {
            let unknown_count = row
                .spring_states
                .iter()
                .filter(|s| matches!(s, SpringState::Unknown))
                .count() as u32;
            perms_iter(&options, unknown_count)
                .map(|mut perm| {
                    row.spring_states
                        .iter()
                        .map(|s| match s {
                            SpringState::Unknown => perm.next().unwrap(),
                            _ => *s,
                        })
                        .collect::<Vec<SpringState>>()
                })
                .filter(|spring_states| calc_damaged_springs(spring_states) == row.damaged_springs)
                .count() as u32
        })
        .sum::<u32>()
        .to_string()
}

pub fn process_part_2(input: &str) -> String {
    let (_, rows) = parse_input(input).unwrap();
    let rows = rows.into_iter().map(expand_row).collect::<Vec<_>>();
    rows.into_iter()
        .map(|row| count_options(row.spring_states, row.damaged_springs))
        .sum::<u64>()
        .to_string()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Row>> {
    separated_list1(line_ending, parse_row)(input)
}

fn parse_row(input: &str) -> IResult<&str, Row> {
    separated_pair(
        many1(alt((
            tag(".").map(|_| SpringState::Operational),
            tag("#").map(|_| SpringState::Damaged),
            tag("?").map(|_| SpringState::Unknown),
        ))),
        tag(" "),
        separated_list1(tag(","), complete::u32),
    )
    .map(|(spring_states, damaged_springs)| Row {
        spring_states,
        damaged_springs,
    })
    .parse(input)
}

fn perms_iter<T: Copy>(
    input: &[T],
    len: u32,
) -> impl Iterator<Item = impl Iterator<Item = T> + '_> {
    (0..input.len().pow(len)).map(move |mut n| {
        (0..len).map(move |_| {
            let s = input[n % input.len()];
            n /= input.len();
            s
        })
    })
}

fn calc_damaged_springs(spring_states: &[SpringState]) -> Vec<u32> {
    let mut damaged_springs = vec![];
    let mut is_damaged = false;
    let mut damaged_count = 0;
    for spring_state in spring_states.iter() {
        match spring_state {
            SpringState::Operational => {
                if is_damaged {
                    damaged_springs.push(damaged_count);
                    damaged_count = 0;
                }
                is_damaged = false;
            }
            SpringState::Damaged => {
                is_damaged = true;
                damaged_count += 1;
            }
            SpringState::Unknown => {
                break;
            }
        }
    }
    if is_damaged {
        damaged_springs.push(damaged_count);
    }
    damaged_springs
}

#[memoize]
fn count_options(spring_states: Vec<SpringState>, damaged_springs: Vec<u32>) -> u64 {
    if spring_states.is_empty() {
        return if damaged_springs.is_empty() { 1 } else { 0 };
    }
    if damaged_springs.is_empty() {
        return if spring_states
            .iter()
            .any(|state| matches!(state, SpringState::Damaged))
        {
            0
        } else {
            1
        };
    }
    match spring_states[0] {
        SpringState::Operational => count_options(spring_states[1..].to_vec(), damaged_springs),
        SpringState::Damaged => {
            let damaged_count = damaged_springs[0] as usize;
            if spring_states.len() >= damaged_count
                && spring_states[0..damaged_count]
                    .iter()
                    .all(|state| matches!(state, SpringState::Damaged | SpringState::Unknown))
                && (spring_states.len() == damaged_count
                    || matches!(
                        spring_states[damaged_count],
                        SpringState::Operational | SpringState::Unknown
                    ))
            {
                count_options(
                    if spring_states.len() == damaged_count {
                        spring_states[damaged_count..].to_vec()
                    } else {
                        spring_states[damaged_count + 1..].to_vec()
                    },
                    damaged_springs[1..].to_vec(),
                )
            } else {
                0
            }
        }
        SpringState::Unknown => {
            let mut spring_states = spring_states;
            spring_states[0] = SpringState::Operational;
            let operational = count_options(spring_states.clone(), damaged_springs.clone());
            spring_states[0] = SpringState::Damaged;
            let damaged = count_options(spring_states, damaged_springs);
            operational + damaged
        }
    }
}

fn expand_row(row: Row) -> Row {
    let spring_states = repeat_n(row.spring_states, 5)
        .interleave(repeat_n(vec![SpringState::Unknown], 4))
        .flatten()
        .collect::<Vec<_>>();
    let damaged_springs = repeat_n(row.damaged_springs, 5)
        .flatten()
        .collect::<Vec<_>>();
    Row {
        spring_states,
        damaged_springs,
    }
}

#[derive(Debug)]
struct Row {
    spring_states: Vec<SpringState>,
    damaged_springs: Vec<u32>,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "21");
    }

    #[test]
    fn part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "525152");
    }

    #[rstest]
    #[case("#.#.### 1,1,3")]
    #[case(".#...#....###. 1,1,3")]
    #[case(".#.###.#.###### 1,3,1,6")]
    #[case("####.#...#... 4,1,1")]
    #[case("#....######..#####. 1,6,5")]
    #[case(".###.##....# 3,2,1")]
    fn calc_damaged_springs_works(#[case] row: &str) {
        let (_, row) = parse_row(row).unwrap();
        let result = calc_damaged_springs(&row.spring_states);
        assert_eq!(result, row.damaged_springs);
    }

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 16384)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 16)]
    #[case("????.######..#####. 1,6,5", 2500)]
    #[case("?###???????? 3,2,1", 506250)]
    fn part_2_lines_4(#[case] row: &str, #[case] expected: u64) {
        let (_, row) = parse_row(row).unwrap();
        let row = expand_row(row);
        let result = count_options(row.spring_states, row.damaged_springs);
        assert_eq!(result, expected);
    }
}
