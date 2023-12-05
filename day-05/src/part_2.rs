use std::{
    cmp::{max, min},
    ops::Range,
};

use nom::{
    bytes::complete::{tag, take_until1},
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};

pub fn process_part_2(input: &str) -> String {
    let (_, almanac) = parse_input(input).unwrap();
    almanac
        .seeds
        .iter()
        .flat_map(|seed| get_location_ranges(&almanac, seed))
        .map(|location_range| location_range.start)
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

fn parse_seeds(input: &str) -> IResult<&str, Vec<Range<u64>>> {
    preceded(
        terminated(tag("seeds:"), space1),
        separated_list1(space1, separated_pair(complete::u64, space1, complete::u64)).map(
            |ranges| {
                ranges
                    .iter()
                    .map(|(start, lenght)| Range {
                        start: *start,
                        end: *start + *lenght,
                    })
                    .collect()
            },
        ),
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
    seeds: Vec<Range<u64>>,
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
    fn map(&self, range: &Range<u64>) -> Option<(Range<u64>, Range<u64>)> {
        let intersecting_range = intersect(&self.source_range, range);
        if let Some(intersecting_range) = intersecting_range {
            let offset = intersecting_range.start - self.source_range.start;
            let start = self.destination_range.start + offset;
            let end = start + (intersecting_range.end - intersecting_range.start);
            Some((intersecting_range, Range { start, end }))
        } else {
            None
        }
    }
}

fn intersect(a: &Range<u64>, b: &Range<u64>) -> Option<Range<u64>> {
    if a.end <= b.start || b.end <= a.start {
        None
    } else {
        Some(max(a.start, b.start)..min(a.end, b.end))
    }
}

fn get_destination_ranges(source_range: &Range<u64>, maps: &[Map]) -> Vec<Range<u64>> {
    let mut range_pairs = maps
        .iter()
        .filter_map(|map| map.map(source_range))
        .collect::<Vec<_>>();
    range_pairs.sort_by_key(|(s_range, _)| s_range.start);
    let range_pairs = range_pairs;
    let mut result = vec![];
    let mut start = source_range.start;
    let mut iter = range_pairs.iter();
    let mut next = iter.next();
    while start < source_range.end {
        if let Some((s_range, d_range)) = &next {
            if s_range.start == start {
                result.push(d_range.clone());
                start = s_range.end;
                next = iter.next();
            } else {
                result.push(start..s_range.start);
                start = s_range.start;
            }
        } else {
            result.push(start..source_range.end);
            start = source_range.end;
        }
    }
    result
}

fn get_location_ranges(almanac: &Almanac, seed_range: &Range<u64>) -> Vec<Range<u64>> {
    let soil = get_destination_ranges(seed_range, &almanac.seed_to_soil_map);
    let fertilizer = soil
        .iter()
        .flat_map(|soil| get_destination_ranges(soil, &almanac.soil_to_fertilizer_map));
    let water = fertilizer.flat_map(|fertilizer| {
        get_destination_ranges(&fertilizer, &almanac.fertilizer_to_water_map)
    });
    let light = water.flat_map(|water| get_destination_ranges(&water, &almanac.water_to_light_map));
    let temperature =
        light.flat_map(|light| get_destination_ranges(&light, &almanac.light_to_temperature_map));
    let humidity = temperature.flat_map(|temperature| {
        get_destination_ranges(&temperature, &almanac.temperature_to_humidity_map)
    });
    let location = humidity
        .flat_map(|humidity| get_destination_ranges(&humidity, &almanac.humidity_to_location_map));
    location.collect()
}
