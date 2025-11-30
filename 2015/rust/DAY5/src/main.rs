use std::time::Instant;
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

fn solve_part_one(input: &Vec<String>) -> usize {
    let mut solution = 0;
    for item in input {
        let mut vowels = 0;
        let mut double = false;
        let mut forbidden = false;
        for i in 0..item.len()-1 {
            let cur = item.chars().nth(i).unwrap();
            let next = item.chars().nth(i + 1).unwrap();
            if cur == 'a' || cur == 'e' || cur == 'i' || cur == 'o' || cur == 'u' {
                vowels = vowels + 1;
            }
            if cur == next {
                double = true;
            }
            if (cur == 'a' && next == 'b') || (cur == 'c' && next == 'd') ||  (cur == 'p' && next == 'q') || (cur == 'x' && next == 'y') {
                forbidden = true;
            }
        }
        let last = item.chars().last().unwrap();
        if last == 'a' || last == 'e' || last == 'i' || last == 'o' || last == 'u' {
            vowels = vowels + 1;
        }
        if vowels >= 3 && double && !forbidden {
            solution = solution + 1;
        }
    }
    return solution
}

fn solve_part_two(input: &Vec<String>) -> usize {
    let mut solution = 0;
    for item in input {
        let mut valid_pair = false;
        let mut gap = false;
        for i in 0..item.len() - 1 {
            let first = item.chars().nth(i).unwrap();
            let middle = item.chars().nth(i + 1).unwrap();
            if i < item.len() - 2 && !gap {
                let last = item.chars().nth(i + 2).unwrap();
                if first == last {
                    gap = true;
                }
            }
            if item[i+2..].contains(item.get(i..i + 2).unwrap()) && !valid_pair {
                valid_pair = true;
            }
            if valid_pair && gap {
                solution = solution + 1;
                break;
            }
        }
    }
    return solution;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_example1() {
        let input = vec![String::from("ugknbfddgicrmopn")];
        assert_eq!(solve_part_one(&input), 1);
    }

    #[test]
    fn test_p1_example2() {
        let input = vec![String::from("aaa")];
        assert_eq!(solve_part_one(&input), 1);
    }

    #[test]
    fn test_p1_example3() {
        let input = vec![String::from("jchzalrnumimnmhp")];
        assert_eq!(solve_part_one(&input), 0);
    }

    #[test]
    fn test_p1_example4() {
        let input = vec![String::from("haegwjzuvuyypxyu")];
        assert_eq!(solve_part_one(&input), 0);
    }

    #[test]
    fn test_p1_example5() {
        let input = vec![String::from("dvszwmarrgswjxmb")];
        assert_eq!(solve_part_one(&input), 0);
    }

    #[test]
    fn test_p2_example1() {
        let input = vec![String::from("qjhvhtzxzqqjkmpb")];
        assert_eq!(solve_part_two(&input), 1);
    }

    #[test]
    fn test_p2_example2() {
        let input = vec![String::from("xxyxx")];
        assert_eq!(solve_part_two(&input), 1);
    }

    #[test]
    fn test_p2_example3() {
        let input = vec![String::from("uurcxstgmygtbstg")];
        assert_eq!(solve_part_two(&input), 0);
    }

    #[test]
    fn test_p2_example4() {
        let input = vec![String::from("ieodomkazucvgmuy")];
        assert_eq!(solve_part_two(&input), 0);
    }
}