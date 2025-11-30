use std::time::Instant;
use std::collections::{HashMap, HashSet};

const INPUT_FILE: &str = "input.txt";

fn main() {
    let mut start = Instant::now();
    let raw_input: Vec<String> = aoc_utils::input::load_input_lines(INPUT_FILE);

    let setup_time = start.elapsed();
    println!("Setup: {:.2?}", setup_time);
    start = Instant::now();
    println!("Day 1 Part 1: {}", solve_part_one(&raw_input));
    let p1_time = start.elapsed();
    println!("Part 1: {:.2?}", setup_time + p1_time);
    start = Instant::now();
    println!("Day 1 Part 2: {}", solve_part_two(&raw_input));
    let p2_time = start.elapsed();
    println!("Part 2: {:.2?}", setup_time + p2_time);
}
fn solve_part_one(input: &Vec<String>) -> u32 {
    
    
    return 0;
}
fn solve_part_two(input: &Vec<String>) -> u32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_example1() {
        let input: Vec<String> = vec![String::from("London to Dublin = 464"), String::from("London to Belfast = 518"), String::from("Dublin to Belfast = 141")];
        assert_eq!(solve_part_one(&input), 605);
    }
}