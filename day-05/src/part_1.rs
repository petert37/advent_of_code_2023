use std::ops::Range;

use nom::{
    bytes::complete::{tag, take_until1},
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::{preceded, terminated},
    IResult, Parser,
};

pub fn process_part_1(input: &str) -> String {
    let (_, almanac) = parse_input(input).unwrap();
    almanac
        .seeds
        .iter()
        .map(|seed| get_location_number(&almanac, *seed))
        .min()
        .unwrap()
        .to_string()
}

fn parse_input(input: &str) -> IResult<&str, Almanac> {
    let (input, seeds) = parse_seeds(input)?;
    let (input, _) = many1(line_ending)(input)?;
    let (input, maps) = separated_list1(many1(line_ending), parse_maps)(input)?;
    let mut almanac = Almanac {
        seeds,
        ..Default::default()
    };
    for (map_name, maps) in maps {
        match map_name {
            "seed-to-soil" => almanac.seed_to_soil_map = maps,
            "soil-to-fertilizer" => almanac.soil_to_fertilizer_map = maps,
            "fertilizer-to-water" => almanac.fertilizer_to_water_map = maps,
            "water-to-light" => almanac.water_to_light_map = maps,
            "light-to-temperature" => almanac.light_to_temperature_map = maps,
            "temperature-to-humidity" => almanac.temperature_to_humidity_map = maps,
            "humidity-to-location" => almanac.humidity_to_location_map = maps,
            _ => panic!("Unknown map name: {}", map_name),
        }
    }
    Ok((input, almanac))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(
        terminated(tag("seeds:"), space1),
        separated_list1(space1, complete::u64),
    )(input)
}

fn parse_maps(input: &str) -> IResult<&str, (&str, Vec<Map>)> {
    let (input, map_name) =
        terminated(take_until1(" map:"), terminated(tag(" map:"), line_ending))(input)?;
    let (input, maps) = separated_list1(line_ending, parse_map)(input)?;
    Ok((input, (map_name, maps)))
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    separated_list1(space1, complete::u64)
        .map(|values| {
            let destination_range_start = values[0];
            let source_range_start = values[1];
            let range_length = values[2];
            let destination_range = destination_range_start..destination_range_start + range_length;
            let source_range = source_range_start..source_range_start + range_length;
            Map {
                destination_range,
                source_range,
            }
        })
        .parse(input)
}

#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil_map: Vec<Map>,
    soil_to_fertilizer_map: Vec<Map>,
    fertilizer_to_water_map: Vec<Map>,
    water_to_light_map: Vec<Map>,
    light_to_temperature_map: Vec<Map>,
    temperature_to_humidity_map: Vec<Map>,
    humidity_to_location_map: Vec<Map>,
}

#[derive(Debug)]
struct Map {
    destination_range: Range<u64>,
    source_range: Range<u64>,
}

impl Map {
    fn map(&self, value: u64) -> Option<u64> {
        if self.source_range.contains(&value) {
            let offset = value - self.source_range.start;
            let destination_value = self.destination_range.start + offset;
            Some(destination_value)
        } else {
            None
        }
    }
}

fn get_destination(value: u64, maps: &[Map]) -> u64 {
    for map in maps {
        if let Some(destination) = map.map(value) {
            return destination;
        }
    }
    value
}

fn get_location_number(almanac: &Almanac, seed: u64) -> u64 {
    let soil = get_destination(seed, &almanac.seed_to_soil_map);
    let fertilizer = get_destination(soil, &almanac.soil_to_fertilizer_map);
    let water = get_destination(fertilizer, &almanac.fertilizer_to_water_map);
    let light = get_destination(water, &almanac.water_to_light_map);
    let temperature = get_destination(light, &almanac.light_to_temperature_map);
    let humidity = get_destination(temperature, &almanac.temperature_to_humidity_map);
    let location = get_destination(humidity, &almanac.humidity_to_location_map);
    #[allow(clippy::let_and_return)]
    location
}
