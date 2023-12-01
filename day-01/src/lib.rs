pub fn process_part_1(input: &str) -> String {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let numbers: Vec<u32> = line.chars().filter_map(|char| char.to_digit(10)).collect();
            numbers.first().unwrap_or(&0) * 10 + numbers.last().unwrap_or(&0)
        })
        .sum();
    sum.to_string()
}

pub fn process_part_2(input: &str) -> String {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let numbers = parse_line(line);
            numbers.first().unwrap_or(&0) * 10 + numbers.last().unwrap_or(&0)
        })
        .sum();
    sum.to_string()
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_line(line: &str) -> Vec<u32> {
    let mut numbers = vec![];
    for i in 0..line.len() {
        let slice = line.get(i..);
        if let Some(slice) = slice {
            let digit = slice
                .get(0..1)
                .map(|digit| digit.parse::<u32>().ok())
                .flatten();
            if let Some(digit) = digit {
                numbers.push(digit);
            } else {
                DIGITS.iter().enumerate().for_each(|(index, digit)| {
                    if slice.starts_with(digit) {
                        numbers.push((index + 1) as u32);
                        return;
                    }
                });
            }
        }
    }
    return numbers;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT_1);
        assert_eq!(result, "142");
    }

    const INPUT_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn part_2_works() {
        let result = process_part_2(INPUT_2);
        assert_eq!(result, "281");
    }
}
