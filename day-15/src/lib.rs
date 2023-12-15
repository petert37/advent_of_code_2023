use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult, Parser,
};

pub fn process_part_1(input: &str) -> String {
    input.split(',').map(my_hash).sum::<u64>().to_string()
}

pub fn process_part_2(input: &str) -> String {
    let (input, steps) = parse_input(input).unwrap();
    debug_assert_eq!(input, "");
    let mut my_hashmap = (0..256).map(|_| vec![]).collect::<Vec<Vec<Lens>>>();
    steps.into_iter().for_each(|step| match step {
        Step::Remove(label) => {
            let hash = my_hash(label);
            let my_box = &mut my_hashmap[hash as usize];
            if let Some(index) = my_box.iter().position(|lens| lens.label == label) {
                my_box.remove(index);
            }
        }
        Step::Add(label, focal_length) => {
            let hash = my_hash(label);
            let my_box = &mut my_hashmap[hash as usize];
            let lens = Lens {
                label,
                focal_length,
            };
            if let Some(index) = my_box.iter().position(|lens| lens.label == label) {
                my_box[index] = lens;
            } else {
                my_box.push(lens);
            }
        }
    });
    my_hashmap
        .iter()
        .enumerate()
        .map(|(box_index, my_box)| {
            my_box
                .iter()
                .enumerate()
                .map(|(lens_index, lens)| {
                    (1 + box_index as u64) * (lens_index as u64 + 1) * lens.focal_length
                })
                .sum::<u64>()
        })
        .sum::<u64>()
        .to_string()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Step>> {
    separated_list1(
        tag(","),
        alt((
            separated_pair(alpha1, tag("="), complete::u64)
                .map(|(label, focal_length)| Step::Add(label, focal_length)),
            terminated(alpha1, tag("-")).map(Step::Remove),
        )),
    )(input)
}

fn my_hash(input: &str) -> u64 {
    input.chars().fold(0, |acc, c| (acc + c as u64) * 17 % 256)
}

enum Step<'a> {
    Remove(&'a str),
    Add(&'a str, u64),
}

struct Lens<'a> {
    label: &'a str,
    focal_length: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "1320");
    }

    #[test]
    fn part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "145");
    }

    #[rstest]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    fn my_hash_works(#[case] input: &str, #[case] expected: u64) {
        let result = my_hash(input);
        assert_eq!(result, expected);
    }
}
