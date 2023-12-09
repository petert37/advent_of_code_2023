use itertools::Itertools;

pub fn process_part_1(input: &str) -> String {
    let histories = parse_input(input);
    histories
        .iter()
        .map(|history| {
            let mut differences = vec![];
            differences.push(history.clone());
            loop {
                let diff = differences
                    .last()
                    .unwrap()
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect::<Vec<i32>>();
                if diff.iter().all(|d| *d == 0) {
                    differences.push(diff);
                    break;
                } else {
                    differences.push(diff);
                }
            }
            let mut last_diff = 0;
            differences.iter().rev().skip(1).for_each(|diff| {
                last_diff += diff.last().unwrap();
            });
            last_diff
        })
        .sum::<i32>()
        .to_string()
}

pub fn process_part_2(input: &str) -> String {
    let histories = parse_input(input);
    histories
        .iter()
        .map(|history| {
            let mut differences = vec![];
            differences.push(history.clone());
            loop {
                let diff = differences
                    .last()
                    .unwrap()
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .collect::<Vec<i32>>();
                if diff.iter().all(|d| *d == 0) {
                    differences.push(diff);
                    break;
                } else {
                    differences.push(diff);
                }
            }
            let mut last_diff = 0;
            differences.iter().rev().skip(1).for_each(|diff| {
                last_diff = diff.first().unwrap() - last_diff;
            });
            last_diff
        })
        .sum::<i32>()
        .to_string()
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.split(' ').map(|part| part.parse().unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "114");
    }

    #[test]
    fn part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "2");
    }
}
