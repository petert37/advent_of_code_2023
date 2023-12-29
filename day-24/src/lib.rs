use glam::{DVec3, I64Vec3};
use indicatif::ProgressIterator;
use ndarray::prelude::*;
use ndarray_linalg::Solve;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space0, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

const MAX_OFFSET: f64 = 0.04;

pub fn process_part_1(input: &str, area_min: i64, area_max: i64) -> String {
    let (input, hailstones) = parse_input(input).unwrap();
    debug_assert_eq!(input, "");
    let intersections = hailstones
        .iter()
        .enumerate()
        .flat_map(|(i, hailstone)| {
            hailstones
                .iter()
                .skip(i + 1)
                .filter_map(|other| {
                    let line1: Line2D = (*hailstone).into();
                    let line2: Line2D = (*other).into();
                    line1
                        .intersection(&line2)
                        .map(|(x, y)| (hailstone, other, x, y))
                })
                .collect::<Vec<_>>()
        })
        .filter(|(hailstone, other, x, y)| {
            hailstone.intersection_in_future((*x, *y)) && other.intersection_in_future((*x, *y))
        })
        .filter(|(_, _, x, y)| {
            let x_is_in_area = *x >= area_min as f64 && *x <= area_max as f64;
            let y_is_in_area = *y >= area_min as f64 && *y <= area_max as f64;
            x_is_in_area && y_is_in_area
        })
        .collect::<Vec<_>>();
    intersections.len().to_string()
}

// https://aoc.csokavar.hu/?day=24
pub fn process_part_2_brute_force(input: &str, stone_velocity_range: i64) -> String {
    let (input, hailstones) = parse_input(input).unwrap();
    debug_assert_eq!(input, "");
    let stone = stone_position_brute_force(hailstones, stone_velocity_range);
    if let Some(stone) = stone {
        (stone.x.round() as i64 + stone.y.round() as i64 + stone.z.round() as i64).to_string()
    } else {
        "No solution found".to_string()
    }
}

// https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/kepu26z/
pub fn process_part_2_linalg(input: &str) -> String {
    let (input, hailstones) = parse_input(input).unwrap();
    debug_assert_eq!(input, "");
    let stone = stone_position_linalg(&hailstones);
    if let Some(stone) = stone {
        (stone.x.round() as i64 + stone.y.round() as i64 + stone.z.round() as i64).to_string()
    } else {
        "No solution found".to_string()
    }
}

fn stone_position_brute_force(
    hailstones: Vec<HailStone>,
    stone_velocity_range: i64,
) -> Option<DVec3> {
    for vx in (-stone_velocity_range..=stone_velocity_range)
        .progress_count((stone_velocity_range * 2 + 1) as u64)
    {
        for vy in -stone_velocity_range..=stone_velocity_range {
            'vz: for vz in -stone_velocity_range..=stone_velocity_range {
                let stone_velocity = I64Vec3::new(vx, vy, vz);
                let mut projected_hailstones = hailstones
                    .iter()
                    .map(|hailstone| hailstone.project_velocity(&stone_velocity));
                let first = projected_hailstones.next().unwrap();
                let second = projected_hailstones.next().unwrap();
                if let Some(intersection) = first.intersect_3d(&second) {
                    for second in projected_hailstones {
                        match first.intersect_3d(&second) {
                            Some(new_intersection) => {
                                let diff = intersection - new_intersection;
                                if diff.x.abs() > MAX_OFFSET
                                    || diff.y.abs() > MAX_OFFSET
                                    || diff.z.abs() > MAX_OFFSET
                                {
                                    continue 'vz;
                                }
                            }
                            None => continue 'vz,
                        }
                    }
                    dbg!(vx, vy, vz, &intersection);
                    return Some(intersection);
                }
            }
        }
    }
    None
}

fn stone_position_linalg(hailstones: &[HailStone]) -> Option<DVec3> {
    let v0 = hailstones[0].velocity;
    let v1 = hailstones[1].velocity;
    let p0 = hailstones[0].position;
    let p1 = hailstones[1].position;
    let a = v1 - v0;
    let b = p0 - p1;
    let c = v0.cross(p0) + p1.cross(v1);
    let l1 = [0, a.z, -a.y, 0, b.z, -b.y].map(|x| x as f64);
    let l2 = [-a.z, 0, a.x, -b.z, 0, b.x].map(|x| x as f64);
    let l3 = [a.y, -a.x, 0, b.y, -b.x, 0].map(|x| x as f64);
    let c1 = c.x as f64;
    let c2 = c.y as f64;
    let c3 = c.z as f64;
    let v1 = hailstones[2].velocity;
    let p1 = hailstones[2].position;
    let a = v1 - v0;
    let b = p0 - p1;
    let c = v0.cross(p0) + p1.cross(v1);
    let l4 = [0, a.z, -a.y, 0, b.z, -b.y].map(|x| x as f64);
    let l5 = [-a.z, 0, a.x, -b.z, 0, b.x].map(|x| x as f64);
    let l6 = [a.y, -a.x, 0, b.y, -b.x, 0].map(|x| x as f64);
    let c4 = c.x as f64;
    let c5 = c.y as f64;
    let c6 = c.z as f64;

    let a = array![l1, l2, l3, l4, l5, l6];
    let b = array![c1, c2, c3, c4, c5, c6];
    let result = a.solve_into(b);
    dbg!(&result);
    result
        .map(|result| DVec3::new(result[0], result[1], result[2]))
        .ok()
}

fn parse_input(input: &str) -> IResult<&str, Vec<HailStone>> {
    separated_list1(line_ending, parse_hailstone)(input)
}

fn parse_hailstone(input: &str) -> IResult<&str, HailStone> {
    let (input, (position, velocity)) =
        separated_pair(parse_vec, delimited(space1, tag("@"), space1), parse_vec)(input)?;
    Ok((input, HailStone { position, velocity }))
}

fn parse_vec(input: &str) -> IResult<&str, I64Vec3> {
    let (input, x) = complete::i64(input)?;
    let (input, _) = delimited(space0, tag(","), space0)(input)?;
    let (input, y) = complete::i64(input)?;
    let (input, _) = delimited(space0, tag(","), space0)(input)?;
    let (input, z) = complete::i64(input)?;
    Ok((input, I64Vec3::new(x, y, z)))
}

// #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
// struct Position {
//     x: i64,
//     y: i64,
//     z: i64,
// }

// impl Add for Position {
//     type Output = Position;

//     fn add(self, rhs: Self) -> Self::Output {
//         Position {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//             z: self.z + rhs.z,
//         }
//     }
// }

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct HailStone {
    position: I64Vec3,
    velocity: I64Vec3,
}

impl HailStone {
    fn intersection_in_future(&self, intersection: (f64, f64)) -> bool {
        let x_is_future = if self.velocity.x > 0 {
            intersection.0 > self.position.x as f64
        } else {
            intersection.0 <= self.position.x as f64
        };
        let y_is_future = if self.velocity.y > 0 {
            intersection.1 > self.position.y as f64
        } else {
            intersection.1 <= self.position.y as f64
        };
        x_is_future && y_is_future
    }

    fn project_velocity(&self, reference_velocity: &I64Vec3) -> HailStone {
        HailStone {
            position: self.position,
            velocity: self.velocity - *reference_velocity,
        }
    }

    fn intersect_3d(&self, other: &HailStone) -> Option<DVec3> {
        let mut a: Array2<f64> = Array2::zeros((2, 2));
        let mut b: Array1<f64> = Array1::zeros(2);

        a[[0, 0]] = self.velocity.x as f64;
        a[[0, 1]] = -other.velocity.x as f64;
        a[[1, 0]] = self.velocity.y as f64;
        a[[1, 1]] = -other.velocity.y as f64;
        b[[0]] = (other.position.x - self.position.x) as f64;
        b[[1]] = (other.position.y - self.position.y) as f64;

        if let Ok(x) = a.solve_into(b) {
            let self_t = x[[0]];
            let other_t = x[[1]];
            if self_t < 0.0 || other_t < 0.0 {
                return None;
            }
            let self_z = self.position.z as f64 + self.velocity.z as f64 * self_t;
            let other_z = other.position.z as f64 + other.velocity.z as f64 * other_t;
            if (self_z - other_z).abs() < MAX_OFFSET {
                return Some(DVec3::new(
                    self.position.x as f64 + self.velocity.x as f64 * self_t,
                    self.position.y as f64 + self.velocity.y as f64 * self_t,
                    self_z,
                ));
            }
        }

        None
    }
}

struct Line2D {
    a: f64,
    b: f64,
    c: f64,
}

impl Line2D {
    fn intersection(&self, other: &Line2D) -> Option<(f64, f64)> {
        let det = self.a * other.b - other.a * self.b;
        if det == 0.0 {
            return None;
        }
        let x = (self.b * other.c - other.b * self.c) / det;
        let y = (self.c * other.a - other.c * self.a) / det;
        Some((x, y))
    }
}

impl From<HailStone> for Line2D {
    fn from(val: HailStone) -> Self {
        let p1 = val.position;
        let p2 = val.position + val.velocity;
        let m = (p2.y - p1.y) as f64 / (p2.x - p1.x) as f64;
        let c = p1.y as f64 - m * p1.x as f64;
        Line2D { a: m, b: -1.0, c }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT, 7, 27);
        assert_eq!(result, "2");
    }

    #[test]
    fn part_2_works_brute_force_works() {
        let result = process_part_2_brute_force(INPUT, 10);
        assert_eq!(result, "47");
    }

    #[test]
    fn part_2_works_linalg() {
        let result = process_part_2_linalg(INPUT);
        assert_eq!(result, "47");
    }

    #[test]
    fn intersect_3d_works() {
        let hailstone1 = HailStone {
            position: I64Vec3::new(19, 13, 30),
            velocity: I64Vec3::new(-2, 1, -2),
        };
        let hailstone2 = HailStone {
            position: I64Vec3::new(24, 13, 10),
            velocity: I64Vec3::new(-3, 1, 2),
        };
        let intersection = hailstone1.intersect_3d(&hailstone2);
        assert_eq!(intersection, Some(DVec3::new(9.0, 18.0, 20.0)));
    }
}
