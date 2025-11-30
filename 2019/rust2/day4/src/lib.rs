const INPUT_FILE: &str = "input4.txt";

pub fn solve() {
    let raw_input: String = aoc_utils::input::load_input_line(INPUT_FILE);
    let input: Vec<u32> = raw_input.split("-").map(|v| v.parse::<u32>().unwrap()).collect();

    solve_part_one(&input);
    solve_part_two(&input);
}


fn solve_part_one(input: &Vec<u32>) {
    let mut valid_passwords = 0;
    for password in *input.get(0).unwrap()..*input.get(1).unwrap()+1 {
        let digits: Vec<u32> = password.to_string()
                        .chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect();
        let mut never_decreasing = true;
        let mut has_duplicate = false;
        for i in 0..digits.len()-1 {
            // two adjacent numbers are the same
            // digits never decrease
            let cur = digits.get(i).unwrap();
            let next = digits.get(i+1).unwrap();
            if next < cur {
                never_decreasing = false;
                break;
            }
            if cur == next {
                has_duplicate = true;
            }
        }
        if never_decreasing && has_duplicate {
            valid_passwords += 1;
        }
    }
    println!("Day 4 Part 1: {}", valid_passwords);
}

fn solve_part_two(input: &Vec<u32>) {
    let mut valid_passwords = 0;
    for password in *input.get(0).unwrap()..*input.get(1).unwrap()+1 {
        let digits: Vec<u32> = password.to_string()
                        .chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect();
        let mut never_decreasing = true;
        let mut has_duplicate = false;
        let mut no_triplicate = true;
        
        let mut in_group = false
        let mut depth = 0;
        let mut start = digits.get(0).unwrap();
        for index in 1..digits.len() {
            let cur = digits.get(i).unwrap();
            
        }
    }
    println!("Day 4 Part 2: {}", valid_passwords);
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn test_is_vertical_positive() {
        let segment = Segment::new((0, 10), (0, 20));
        assert_eq!(segment.is_vertical(), true);
    }
        */
}