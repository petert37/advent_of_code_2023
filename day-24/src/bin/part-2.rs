use std::fs;

use day_24::process_part_2_brute_force;
use day_24::process_part_2_linalg;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("Linear algebra solution: {}", process_part_2_linalg(&file));
    println!(
        "Brute force solution: {}",
        process_part_2_brute_force(&file, 300)
    );
}
