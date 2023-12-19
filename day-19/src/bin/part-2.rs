use std::fs;

use day_19::process_part_2;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part_2(&file));
}
