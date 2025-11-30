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

enum Operations {
    ON,
    OFF,
    TOGGLE,
}

#[derive(PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(input: &str) -> Self {
        let dimensions = input.split(',').collect::<Vec<&str>>();
        if dimensions.len() > 2 {
            panic!("Invalid coordinate string");
        }
        return Coordinate { x: dimensions[0].parse::<usize>().unwrap(), y: dimensions[1].parse::<usize>().unwrap() }
    }
}
struct Instruction {
    operation: Operations,
    top_left: Coordinate,
    bottom_right: Coordinate,
}

impl Instruction {
    fn new(input: &str) -> Self {
        let parts = input.split(" ").collect::<Vec<&str>>();
        match parts.len() {
            4 => {
                let operation = Operations::TOGGLE;
                let top_left = Coordinate::new(parts[1]);
                let bottom_right = Coordinate::new(parts[3]);
                return Self { operation, top_left, bottom_right }
            },
            5 => {
                let operation = match parts[1] {
                    "on" => Operations::ON,
                    "off" => Operations::OFF,
                    _ => panic!("Invalid instruction string")
                };
                let top_left = Coordinate::new(parts[2]);
                let bottom_right = Coordinate::new(parts[4]);
                return Self { operation, top_left, bottom_right }
            },
            _ => panic!("Invalid instruction string")
        }
    }
}
fn solve_part_one(input: &Vec<String>) -> usize {
    let width = 1000;
    let mut grid: HashSet<usize> = HashSet::new();
    for item in input {
        let instruction = Instruction::new(item);
        for i in instruction.top_left.y..=instruction.bottom_right.y {
            for j in instruction.top_left.x..=instruction.bottom_right.x {
                let index = (width * i) + j;
                match instruction.operation {
                    Operations::ON => {
                        grid.insert(index);
                    },
                    Operations::OFF => {
                        grid.remove(&index);
                    },
                    Operations::TOGGLE => {
                        if grid.contains(&index) {
                            grid.remove(&index);
                        } else {
                            grid.insert(index);
                        }
                    },
                }
            }
        }
    }
    return grid.len();
}
fn solve_part_two(input: &Vec<String>) -> usize {
    let width = 1000;
    let mut grid: HashMap<Coordinate, u32> = HashMap::new();
    for item in input {
        let instruction = Instruction::new(item);
        for i in instruction.top_left.y..=instruction.bottom_right.y {
            for j in instruction.top_left.x..=instruction.bottom_right.x {
                let coordinate = Coordinate { x: j, y: i };
                match instruction.operation {
                    Operations::ON => {
                        grid.entry(coordinate).and_modify(|count| *count = *count + 1).or_insert(1);
                    },
                    Operations::OFF => {
                        grid.entry(coordinate).and_modify(|count| *count = count.saturating_sub(1));
                    },
                    Operations::TOGGLE => {
                        grid.entry(coordinate).and_modify(|count| *count = *count + 2).or_insert(2);
                    },
                }
            }
        }
    }
    return grid.values().map(|x| *x as usize).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_example1() {
        let input = vec![String::from("turn on 0,0 through 999,999")];
        assert_eq!(solve_part_one(&input), 1_000_000);
    }

    #[test]
    fn test_p1_example2() {
        let input = vec![String::from("toggle 500,0 through 999,0")];
        assert_eq!(solve_part_one(&input), 500);
    }

    #[test]
    fn test_p1_example3() {
        let input = vec![String::from("turn on 0,0 through 999,999"), String::from("turn off 499,499 through 500,500")];
        assert_eq!(solve_part_one(&input), 999_996);
    }

    #[test]
    fn test_p2_example1() {
        let input = vec![String::from("turn on 0,0 through 0,0")];
        assert_eq!(solve_part_two(&input), 1);
    }

    #[test]
    fn test_p2_example2() {
        let input = vec![String::from("toggle 0,0 through 999,999")];
        assert_eq!(solve_part_two(&input), 2_000_000);
    }
}