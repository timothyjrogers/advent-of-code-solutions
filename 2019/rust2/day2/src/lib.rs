const INPUT_FILE: &str = "input2.txt";

pub fn solve() {
    let raw_input: String = aoc_utils::input::load_input_line(INPUT_FILE);
    let input: Vec<&str> = raw_input.split(",").collect();

    solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_one(input: &Vec<&str>) {
    let mut memory: Vec<i32> = input.to_owned().into_iter().map(|s| s.parse::<i32>().unwrap()).collect();
    memory[1] = 12;
    memory[2] = 2;
    memory = execute_intcode_program(&memory);
    println!("Day 2 Part 1: {}", memory[0]);
}

fn solve_part_two(input: &Vec<&str>) {
    let mut initial_memory: Vec<i32> = input.to_owned().into_iter().map(|s| s.parse::<i32>().unwrap()).collect();
    for offset1 in 0..100 {
        for offset2 in 0..100 {
            let mut memory = initial_memory.clone();
            memory[1] = offset1;
            memory[2] = offset2;
            memory = execute_intcode_program(&memory);
            if memory[0] == 19690720 {
                println!("Day 2 Part 2: {}", 100 * offset1 + offset2);
                break;
            }
        }
    }
}

fn execute_intcode_program(initial_memory: &Vec<i32>) -> Vec<i32> {
    let mut memory = initial_memory.to_owned();
    let mut instruction_pointer = 0;
    while memory[instruction_pointer] != 99 {
        let operand1_addr = memory[instruction_pointer + 1] as usize;
        let operand2_addr = memory[instruction_pointer + 2] as usize;
        let destination_addr = memory[instruction_pointer + 3] as usize;
        match memory[instruction_pointer] {
            1 => {
                memory[destination_addr] = memory[operand1_addr] + memory[operand2_addr];
            },
            2 => {
                memory[destination_addr] = memory[operand1_addr] * memory[operand2_addr];
            },
            _ => {
                panic!("Invalid INTCODE operation");
            }
        }
        instruction_pointer += 4;
    }
    return memory;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intcode_example1() {
        let memory: Vec<i32> = vec![1,0,0,0,99];
        assert_eq!(execute_intcode_program(&memory), vec![2,0,0,0,99]);
    }
    
    #[test]
    fn test_intcode_example2() {
        let memory: Vec<i32> = vec![2,3,0,3,99];
        assert_eq!(execute_intcode_program(&memory), vec![2,3,0,6,99]);
    }

    #[test]
    fn test_intcode_example3() {
        let memory: Vec<i32> = vec![2,4,4,5,99,0];
        assert_eq!(execute_intcode_program(&memory), vec![2,4,4,5,99,9801]);
    }

    #[test]
    fn test_intcode_example4() {
        let memory: Vec<i32> = vec![1,1,1,4,99,5,6,0,99];
        assert_eq!(execute_intcode_program(&memory), vec![30,1,1,4,2,5,6,0,99]);
    }
}