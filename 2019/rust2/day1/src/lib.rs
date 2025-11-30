const INPUT_FILE: &str = "input1.txt";

pub fn solve() {

    let raw_input: Vec<String> = aoc_utils::input::load_input_lines(INPUT_FILE);
    let mut parsed_input = Vec::<u32>::new();
    for value in raw_input {
        parsed_input.push(value.parse::<u32>().unwrap());
    }

    solve_part_one(&parsed_input);
    solve_part_two(&parsed_input);
}

fn solve_part_one(input: &Vec<u32>) {
    let mut solution = 0;
    for val in input.to_owned().into_iter() {
        solution += calculate_fuel(val);
    }
    println!("Day 1 Part 1: {}", solution);
}

fn solve_part_two(input: &Vec<u32>) {
    let mut solution: u32 = 0;
    for val in input.to_owned().into_iter() {
        solution += calculate_recursive_fuel(val);
    }
    println!("Day 1 Part 2: {}", solution);
}

fn calculate_fuel(mass: u32) -> u32 {
    let fuel = mass / 3;
    if fuel > 2 {
        return fuel - 2;
    }
    return 0;
}

fn calculate_recursive_fuel(mass: u32) -> u32 {
    let mut total = 0;
    let mut fuel = calculate_fuel(mass);
    while fuel > 0 {
        total += fuel;
        fuel = calculate_fuel(fuel);
    }
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_fuel1() {
        let mass: u32 = 12;
        assert_eq!(calculate_fuel(mass), 2);
    }

    #[test]
    fn test_calculate_fuel2() {
        let mass: u32 = 14;
        assert_eq!(calculate_fuel(mass), 2);
    }

    #[test]
    fn test_calculate_fuel3() {
        let mass: u32 = 1969;
        assert_eq!(calculate_fuel(mass), 654);
    }

    #[test]
    fn test_calculate_fuel4() {
        let mass: u32 = 100756;
        assert_eq!(calculate_fuel(mass), 33583);
    }

    #[test]
    fn test_calculate_recursive_fuel1() {
        let mass: u32 = 14;
        assert_eq!(calculate_recursive_fuel(mass), 2);
    }

    #[test]
    fn test_calculate_recursive_fuel2() {
        let mass: u32 = 1969;
        assert_eq!(calculate_recursive_fuel(mass), 966);
    }

    #[test]
    fn test_calculate_recursive_fuel3() {
        let mass: u32 = 100756;
        assert_eq!(calculate_recursive_fuel(mass), 50346);
    }
}