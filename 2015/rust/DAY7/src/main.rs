use std::time::Instant;
use std::collections::{HashMap, VecDeque};

const INPUT_FILE: &str = "input.txt";

fn main() {
    let mut start = Instant::now();
    let raw_input: Vec<String> = aoc_utils::input::load_input_lines(INPUT_FILE);

    let setup_time = start.elapsed();
    println!("Setup: {:.2?}", setup_time);
    start = Instant::now();
    let p1_solution = solve_part_one(&raw_input);
    println!("Day 1 Part 1: {}", p1_solution);
    let p1_time = start.elapsed();
    println!("Part 1: {:.2?}", setup_time + p1_time);
    start = Instant::now();
    println!("Day 1 Part 2: {}", solve_part_two(&raw_input, p1_solution));
    let p2_time = start.elapsed();
    println!("Part 2: {:.2?}", setup_time + p2_time);
}

#[derive(Debug)]
enum Operations {
    AND,
    OR,
    NOT,
    LSHIFT,
    RSHIFT,
    COPY,
}
struct ParsedExpression  {
    operation: Operations,
    left_wire: Option<String>,
    left_value: Option<u32>,
    right_wire: Option<String>,
    right_value: Option<u32>,
    output: String,
}

impl ParsedExpression {
    fn new(operation: Operations, left_wire: Option<String>, left_value: Option<u32>, right_wire: Option<String>, right_value: Option<u32>, output: String) -> Self {
        Self { operation, left_wire, left_value, right_wire, right_value, output }
    }

    fn parse_expression(expr: &String) -> Self {
        let parts: Vec<&str> = expr.split(" ").collect::<Vec<&str>>();
        if parts.len() == 3 {
            if is_numeric(parts[0]) {
                return ParsedExpression::new(Operations::COPY, None, Some(parts[0].parse::<u32>().unwrap()), None, None, String::from(*parts.last().unwrap()));
            }
            return ParsedExpression::new(Operations::COPY, Some(String::from(parts[0])), None, None, None, String::from(*parts.last().unwrap()));
        } else if parts.len() == 4 {
            return ParsedExpression::new(Operations::NOT, Some(String::from(parts[1])), None, None, None, String::from(*parts.last().unwrap()));
        } else if parts.len() == 5 {
            let operation = match parts[1] {
                "AND" => Operations::AND,
                "OR" => Operations::OR,
                "LSHIFT" => Operations::LSHIFT,
                "RSHIFT" => Operations::RSHIFT,
                _ => panic!("Invalid input expresssion")
            };
            let mut expression = ParsedExpression::new(operation, None, None, None, None, String::from(*parts.last().unwrap()));
            if is_numeric(parts[0]) {
                expression.left_value = Some(parts[0].parse::<u32>().unwrap());
            } else {
                expression.left_wire = Some(String::from(parts[0]))
            }
            if is_numeric(parts[2]) {
                expression.right_value = Some(parts[2].parse::<u32>().unwrap());
            } else {
                expression.right_wire = Some(String::from(parts[2]))
            }
            return expression;
        }
        panic!("Invalid input expression");
    }

    fn update_values(&mut self, wires: &HashMap<String, u32>) {
        if self.left_value.is_none() {
            match &self.left_wire {
                Some(w) => {
                    match wires.get(w) {
                        Some(v) => self.left_value = Some(*v),
                        None => return,
                    }
                },
                None => return
            }
        }
        if self.right_value.is_none() {
            match &self.right_wire {
                Some(w) => {
                    match wires.get(w) {
                        Some(v) => self.right_value = Some(*v),
                        None => return,
                    }
                },
                None => return
            }
        }
    }

    fn evaluate(&mut self) -> Option<u32> {
        if self.left_value.is_none() {
            return None;
        }
        match &self.operation {
            Operations::COPY => return Some(self.left_value?),
            Operations::NOT => return Some(!self.left_value?),
            Operations::AND=> {
                if self.right_value.is_none() {
                    return None;
                }
                return Some(self.left_value? & self.right_value?);
            },
            Operations::OR=> {
                if self.right_value.is_none() {
                    return None;
                }
                return Some(self.left_value? | self.right_value?);
            },
            Operations::LSHIFT => {
                if self.right_value.is_none() {
                    return None;
                }
                return Some(self.left_value? << self.right_value?);
            },
            Operations::RSHIFT => {
                if self.right_value.is_none() {
                    return None;
                }
                return Some(self.left_value? >> self.right_value?);
            }
        }
    }
}

fn is_numeric(s: &str) -> bool {
    return s.as_bytes()[0].is_ascii_digit();
}

/*
 - for each expression in input:
    - parse the expression into ParsedExpression
 - while ParsedExpressions available:
    - pop ParsedExpression from list
    - update expression with known values
    - If expression evalutes, add result to known values and drop
    - Else push expression to end of list
 - return value of wires['a']
 */
fn solve_part_one(input: &Vec<String>) -> u32 {
    let mut wires: HashMap<String, u32> = HashMap::new();
    let mut expressions: VecDeque<ParsedExpression> = input.iter().map(ParsedExpression::parse_expression).collect();
    while !expressions.is_empty() {
        let mut candidate = expressions.pop_front().unwrap();
        candidate.update_values(&wires);
        match candidate.evaluate() {
            Some(v) => {
                wires.insert(candidate.output, v);
            },
            None => expressions.push_back(candidate)
        }
    }
    return *wires.get(&String::from("a")).unwrap();
}
fn solve_part_two(input: &Vec<String>, b: u32) -> u32 {
    let mut wires: HashMap<String, u32> = HashMap::new();
    let mut expressions: VecDeque<ParsedExpression> = input.iter().map(ParsedExpression::parse_expression).collect();
    while !expressions.is_empty() {
        wires.insert(String::from("b"), b); 
        let mut candidate = expressions.pop_front().unwrap();
        candidate.update_values(&wires);
        match candidate.evaluate() {
            Some(v) => {
                wires.insert(candidate.output, v);
            },
            None => expressions.push_back(candidate)
        }
    }
    return *wires.get(&String::from("a")).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_example1() {
        let input = vec![String::from("123 -> a")];
        assert_eq!(solve_part_one(&input), 123);
    }

    #[test]
    fn test_p1_example2() {
        let input = vec![String::from("123 -> b"), String::from("b -> a")];
        assert_eq!(solve_part_one(&input), 123);
    }

    #[test]
    fn test_p1_example3() {
        let input = vec![String::from("123 -> b"), String::from("NOT b -> a")];
        assert_eq!(solve_part_one(&input), !123);
    }

    #[test]
    fn test_p1_example4() {
        let input = vec![String::from("1 -> b"), String::from("2 -> c"), String::from("b AND c -> a")];
        assert_eq!(solve_part_one(&input), 1 & 2);
    }

    #[test]
    fn test_p1_example5() {
        let input = vec![String::from("1 -> b"), String::from("2 -> c"), String::from("b OR c -> a")];
        assert_eq!(solve_part_one(&input), 1 | 2);
    }

    #[test]
    fn test_p1_example6() {
        let input = vec![String::from("1 -> b"), String::from("2 -> c"), String::from("b LSHIFT c -> a")];
        assert_eq!(solve_part_one(&input), 1 << 2);
    }

    #[test]
    fn test_p1_example7() {
        let input = vec![String::from("1 -> b"), String::from("2 -> c"), String::from("b RSHIFT c -> a")];
        assert_eq!(solve_part_one(&input), 1 >> 2);
    }
}