use std::time::Instant;
use md5;
const INPUT_FILE: &str = "input.txt";

fn main() {
    let mut start = Instant::now();
    let raw_input: String = aoc_utils::input::load_input_line(INPUT_FILE);

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

fn solve_part_one(input: &String) -> usize {
    let mut solution = 1;
    loop {
        let value = format!("{}{}", input, solution);
        let hash = format!("{:016x}", md5::compute(&value));
        if &hash[0..5] == "00000" {
           return solution;
        }
        solution = solution + 1;
    }
}

fn solve_part_two(input: &String) -> usize {
    let mut solution = 1;
    loop {
        let value = format!("{}{}", input, solution);
        let hash = format!("{:016x}", md5::compute(&value));
        if &hash[0..6] == "000000" {
            return solution;
        }
        solution = solution + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_example1() {
        let input = String::from("abcdef");
        assert_eq!(solve_part_one(&input), 609043);
    }

    #[test]
    fn test_p1_example2() {
        let input = String::from("pqrstuv");
        assert_eq!(solve_part_one(&input), 1048970);
    }

}