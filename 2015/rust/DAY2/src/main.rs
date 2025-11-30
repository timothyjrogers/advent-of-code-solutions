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

fn solve_part_one(input: &Vec<String>) -> u32 {
    let mut solution = 0;
    for item in input {
        let dimensions: Vec<u32> = item.split("x").map(|x| x.parse::<u32>().unwrap()).collect();
        let length = dimensions[0];
        let width = dimensions[1];
        let height = dimensions[2];
        let surface_area = aoc_utils::math::rect_prism_area(length, width, height);
        let sides = vec![length * width, width * height, length * height];
        let slack = sides
            .iter()
            .min();
        solution = solution + surface_area + slack.unwrap();
    }
    return solution;
}

fn solve_part_two(input: &Vec<String>) -> u32 {
    let mut solution = 0;
    for item in input {
        let dimensions: Vec<u32> = item.split("x").map(|x| x.parse::<u32>().unwrap()).collect();
        let length = dimensions[0];
        let width = dimensions[1];
        let height = dimensions[2];
        let volume = aoc_utils::math::rect_prism_volume(length, width, height);
        let perimeters = vec![2 * length + 2 * width, 2 * width + 2 * height, 2 * length + 2 * height];
        let bow = perimeters
            .iter()
            .min();
        solution = solution + volume + bow.unwrap();
    }
    return solution;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_example1() {
        let input = vec![String::from("2x3x4")];
        assert_eq!(solve_part_one(&input), 58);
    }

    #[test]
    fn test_p1_example2() {
        let input = vec![String::from("1x1x10")];
        assert_eq!(solve_part_one(&input), 43);
    }

    #[test]
    fn test_p2_example1() {
        let input = vec![String::from("2x3x4")];
        assert_eq!(solve_part_two(&input), 34);
    }

    #[test]
    fn test_p2_example2() {
        let input = vec![String::from("1x1x10")];
        assert_eq!(solve_part_two(&input), 14);
    }
}