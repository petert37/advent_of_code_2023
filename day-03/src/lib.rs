use std::ops::{Not, Range};

pub fn process_part_1(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let grid_numbers = get_grid_numbers(&grid);
    grid_numbers
        .iter()
        .filter(|grid_number| grid_number.is_engine_part(&grid))
        .map(|grid_number| grid_number.number)
        .sum::<u32>()
        .to_string()
}

pub fn process_part_2(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let grid_numbers = get_grid_numbers(&grid);
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == '*')
                .map(|(x, _)| (x, y))
                .collect::<Vec<_>>()
        })
        .filter_map(|(x, y)| {
            let surrounding_numbers = grid_numbers
                .iter()
                .filter(|grid_number| grid_number.surrounding_indices(&grid).contains(&(x, y)))
                .collect::<Vec<_>>();
            if surrounding_numbers.len() == 2 {
                Some(
                    surrounding_numbers
                        .iter()
                        .map(|grid_number| grid_number.number)
                        .product::<u32>(),
                )
            } else {
                None
            }
        })
        .sum::<u32>()
        .to_string()
}

fn get_grid_numbers(grid: &[Vec<char>]) -> Vec<GridNumber> {
    let mut grid_numbers = vec![];
    grid.iter().enumerate().for_each(|(y, line)| {
        let mut is_number = false;
        let mut x_start = 0;
        let mut number = 0;
        line.iter().enumerate().for_each(|(x, c)| {
            if c.is_ascii_digit() {
                number = number * 10 + c.to_digit(10).unwrap();
                if !is_number {
                    is_number = true;
                    x_start = x;
                }
            } else if is_number {
                grid_numbers.push(GridNumber {
                    number,
                    x_range: x_start..x,
                    y,
                });
                is_number = false;
                number = 0;
            }
        });
        if is_number {
            grid_numbers.push(GridNumber {
                number,
                x_range: x_start..line.len(),
                y,
            });
        }
    });
    grid_numbers
}

#[derive(Debug)]
struct GridNumber {
    number: u32,
    x_range: Range<usize>,
    y: usize,
}

impl GridNumber {
    fn is_engine_part(&self, grid: &Vec<Vec<char>>) -> bool {
        let surrounding_chars = self.surrounding_chars(grid);
        surrounding_chars
            .iter()
            .all(|c| c.is_ascii_digit() || *c == '.')
            .not()
    }

    fn surrounding_chars(&self, grid: &Vec<Vec<char>>) -> Vec<char> {
        self.surrounding_indices(grid)
            .iter()
            .map(|(x, y)| grid[*y][*x])
            .collect::<Vec<_>>()
    }

    fn surrounding_indices(&self, grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
        let mut indices = vec![];
        let sy = self.y as i32;
        let sx_start = self.x_range.start as i32;
        let sx_end = self.x_range.end as i32;
        for y in sy - 1..=sy + 1 {
            for x in sx_start - 1..=sx_end {
                if y < 0 || y >= grid.len() as i32 || x < 0 || x >= grid[y as usize].len() as i32 {
                    continue;
                }
                if y == sy && x >= sx_start && x < sx_end {
                    continue;
                }
                indices.push((x as usize, y as usize));
            }
        }
        indices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "4361");
    }

    #[test]
    fn part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "467835");
    }
}
