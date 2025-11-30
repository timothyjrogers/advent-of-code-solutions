use std::time::Instant;

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

fn solve_part_one(input: &str) -> i32 {
    let mut solution = 0;
    for c in input.chars() {
        match c {
            '(' => solution = solution + 1,
            ')' => solution = solution - 1,
            _ => panic!("Invalid character in input")
        }
    }
    return solution;
}

fn solve_part_two(input: &str) -> i32 {
    let mut solution = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => solution = solution + 1,
            ')' => solution = solution - 1,
            _ => panic!("Invalid character in input")
        }
        if solution < 0 {
            return (i + 1) as i32;
        }
    }
    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_example1() {
        let input = "(())";
        assert_eq!(solve_part_one(input), 0);
    }

    #[test]
    fn test_p1_example2() {
        let input = "()()";
        assert_eq!(solve_part_one(input), 0);
    }

    #[test]
    fn test_p1_example3() {
        let input = "(((";
        assert_eq!(solve_part_one(input), 3);
    }

    #[test]
    fn test_p1_example4() {
        let input = "(()(()(";
        assert_eq!(solve_part_one(input), 3);
    }

    #[test]
    fn test_p1_example5() {
        let input = "))(((((";
        assert_eq!(solve_part_one(input), 3);
    }

    #[test]
    fn test_p1_example6() {
        let input = "())";
        assert_eq!(solve_part_one(input), -1);
    }

    #[test]
    fn test_p1_example7() {
        let input = "))(";
        assert_eq!(solve_part_one(input), -1);
    }

    #[test]
    fn test_p1_example8() {
        let input = ")))";
        assert_eq!(solve_part_one(input), -3);
    }

    #[test]
    fn test_p1_example9() {
        let input = ")())())";
        assert_eq!(solve_part_one(input), -3);
    }

    #[test]
    fn test_p2_example1() {
        let input = ")";
        assert_eq!(solve_part_two(input), 1);
    }

    #[test]
    fn test_p2_example2() {
        let input = "()())";
        assert_eq!(solve_part_two(input), 5);
    }
}