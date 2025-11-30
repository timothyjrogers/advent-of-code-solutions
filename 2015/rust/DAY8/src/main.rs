use std::time::Instant;
use std::collections::{HashMap, VecDeque};

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
fn mem_length(s: &String) -> u32 {
    let chars = s.chars().collect::<Vec<char>>();
    let mut length = 0;
    let mut index = 0;
    while index < s.len() {
        if *chars.get(index).unwrap() == '\\' {
            if *chars.get(index + 1).unwrap() == 'x' {
                index += 4;
            } else {
                index += 2;
            }
        } else {
            index += 1;
        }
        length += 1;
    }
    return length - 2; // subtract open and closing quotes
}

fn escape(s: &String) -> String {
    let mut input_chars: Vec<char> = s.chars().collect();
    let mut output_chars: Vec<char> = vec!['"'];
    let mut index = 0;
    while index < input_chars.len() {
        let cur = input_chars.get(index).unwrap();
        if *cur == '\\' {
            output_chars.push('\\');
            output_chars.push('\\');
        } else if *cur == '"' {
            output_chars.push('\\');
            output_chars.push('"');
        } else {
            output_chars.push(*cur);
        }
        index += 1;
    }
    output_chars.push('"');
    return output_chars.into_iter().collect();
}
fn solve_part_one(input: &Vec<String>) -> u32 {
    let mut total: u32 = 0;
    for s in input {
        total = total + s.len() as u32 - mem_length(s);
    }
    return total;;
}
fn solve_part_two(input: &Vec<String>) -> u32 {
    let mut total: u32 = 0;
    for s in input {
        total = total + escape(s).len() as u32 - s.len() as u32;
    }
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem_length1() {
        let s = String::from("\"\"");
        assert_eq!(mem_length(&s), 0);
    }

    #[test]
    fn test_mem_length2() {
        let s = String::from("\"abc\"");
        assert_eq!(mem_length(&s), 3);
    }

    #[test]
    fn test_mem_length3() {
        let s = String::from("\"aaa\\\"aaa\"");
        assert_eq!(mem_length(&s), 7);
    }

    #[test]
    fn test_mem_length4() {
        let s = String::from("\"\\x27\"");
        assert_eq!(mem_length(&s), 1);
    }
    #[test]
    fn test_p1_example1() {
        let input = vec![String::from("\"\"")];
        assert_eq!(solve_part_one(&input), 2);
    }

    #[test]
    fn test_p1_example2() {
        let input = vec![String::from("\"abc\"")];
        assert_eq!(solve_part_one(&input), 2);
    }

    #[test]
    fn test_p1_example3() {
        let input = vec![String::from("\"aaa\\\"aaa\"")];
        assert_eq!(solve_part_one(&input), 3);
    }

    #[test]
    fn test_p1_example4() {
        let input = vec![String::from("\"\\x27\"")];
        assert_eq!(solve_part_one(&input), 5);
    }

    #[test]
    fn test_escape1() {
        let s = String::from("\"\"");
        assert_eq!(escape(&s), String::from("\"\\\"\\\"\""))
    }

    #[test]
    fn test_escape2() {
        let s = String::from("\"abc\"");
        assert_eq!(escape(&s), String::from("\"\\\"abc\\\"\""))
    }

    #[test]
    fn test_escape3() {
        let s = String::from("\"aaa\"aaa\"");
        assert_eq!(escape(&s), String::from("\"\\\"aaa\\\"aaa\\\"\""))
    }

    #[test]
    fn test_escape4() {
        let s = String::from("\"\\x27\"");
        assert_eq!(escape(&s), String::from("\"\\\"\\\\x27\\\"\""))
    }

    #[test]
    fn test_p2_example1() {
        let input = vec![String::from("\"\"")];
        assert_eq!(solve_part_two(&input), 4);
    }

    #[test]
    fn test_p2_example2() {
        let input = vec![String::from("\"abc\"")];
        assert_eq!(solve_part_two(&input), 4);
    }

    #[test]
    fn test_p2_example3() {
        let input = vec![String::from("\"aaa\\\"aaa\"")];
        assert_eq!(solve_part_two(&input), 6);
    }

    #[test]
    fn test_p2_example4() {
        let input = vec![String::from("\"\\x27\"")];
        assert_eq!(solve_part_two(&input), 5);
    }
}