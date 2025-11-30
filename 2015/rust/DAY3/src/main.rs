use std::time::Instant;
use std::collections::HashSet;
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
    let mut solution = 0;
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut current: (i32, i32) = (0, 0);
    seen.insert(current);
    for c in input.chars() {
        match c {
            '^' => current = (current.0, current.1 + 1),
            '>' => current = (current.0 + 1, current.1),
            'v' => current = (current.0, current.1 - 1),
            '<' => current = (current.0 - 1, current.1),
            _ => panic!("Invalid character in input")
        }
        seen.insert(current);
    }
    return seen.len();
}

fn solve_part_two(input: &String) -> usize {
    let mut solution = 0;
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut current_santa: (i32, i32) = (0, 0);
    let mut current_robot: (i32, i32) = (0, 0);
    let mut santas_turn = true;
    seen.insert(current_santa);
    for c in input.chars() {
        if santas_turn {
            match c {
                '^' => current_santa = (current_santa.0, current_santa.1 + 1),
                '>' => current_santa = (current_santa.0 + 1, current_santa.1),
                'v' => current_santa = (current_santa.0, current_santa.1 - 1),
                '<' => current_santa = (current_santa.0 - 1, current_santa.1),
                _ => panic!("Invalid character in input")
            }
            seen.insert(current_santa);
        } else {
            match c {
                '^' => current_robot = (current_robot.0, current_robot.1 + 1),
                '>' => current_robot = (current_robot.0 + 1, current_robot.1),
                'v' => current_robot = (current_robot.0, current_robot.1 - 1),
                '<' => current_robot = (current_robot.0 - 1, current_robot.1),
                _ => panic!("Invalid character in input")
            }
            seen.insert(current_robot);
        }
        santas_turn = !santas_turn;
    }
    return seen.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_example1() {
        let input = String::from(">");
        assert_eq!(solve_part_one(&input), 2);
    }

    #[test]
    fn test_p1_example2() {
        let input = String::from("^>v<");
        assert_eq!(solve_part_one(&input), 4);
    }

    #[test]
    fn test_p1_example3() {
        let input = String::from("^v^v^v^v^v");
        assert_eq!(solve_part_one(&input), 2);
    }

    #[test]
    fn test_p2_example1() {
        let input = String::from("^v");
        assert_eq!(solve_part_two(&input), 3);
    }

    #[test]
    fn test_p2_example2() {
        let input = String::from("^>v<");
        assert_eq!(solve_part_two(&input), 3);
    }

    #[test]
    fn test_p2_example3() {
        let input = String::from("^v^v^v^v^v");
        assert_eq!(solve_part_two(&input), 11);
    }
}