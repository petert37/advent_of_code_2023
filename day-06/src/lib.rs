use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space0, space1},
    multi::separated_list1,
    sequence::{preceded, terminated},
    IResult,
};

pub fn process_part_1(input: &str) -> String {
    let (_, races) = parse_input(input).unwrap();
    races
        .iter()
        .map(Race::count_winning_times)
        .product::<usize>()
        .to_string()
}

pub fn process_part_2(input: &str) -> String {
    let (_, races) = parse_input(input.replace(' ', "").as_str()).unwrap();
    races.first().unwrap().count_winning_times().to_string()
}

fn parse_input(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, times) = preceded(
        terminated(tag("Time:"), space0),
        separated_list1(space1, complete::u64),
    )(input)?;
    let (input, _) = line_ending(input)?;
    let (input, record_distances) = preceded(
        terminated(tag("Distance:"), space0),
        separated_list1(space1, complete::u64),
    )(input)?;
    let races = times
        .into_iter()
        .zip(record_distances)
        .map(|(time, record_distance)| Race {
            time,
            record_distance,
        })
        .collect();
    Ok((input, races))
}

#[derive(Debug)]
struct Race {
    time: u64,
    record_distance: u64,
}

impl Race {
    fn count_winning_times(&self) -> usize {
        (0_u64..=self.time)
            .map(|press_time| press_time * (self.time - press_time))
            .filter(|distance| distance > &self.record_distance)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "288");
    }

    #[test]
    fn part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "71503");
    }
}
