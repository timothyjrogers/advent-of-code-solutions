use std::collections::HashSet;
use std::time::Instant;

const INPUT_FILE: &str = "input15.txt";

#[derive(Clone)]
struct Warehouse {
    walls: HashSet<(isize,isize)>,
    boxes: HashSet<(isize,isize)>,
    robot: (isize, isize)
}

pub fn solve() {
    let mut start = Instant::now();
    let raw_input: Vec<String> = aoc_utils::input::load_input_lines(INPUT_FILE);
    let mut section = 0;
    let mut walls: HashSet<(isize,isize)> =  HashSet::new();
    let mut boxes: HashSet<(isize, isize)> = HashSet::new();
    let mut robot: (isize, isize) = (-1, -1);
    let mut moves: Vec<char> = Vec::new();
    for (y, line) in raw_input.iter().enumerate() {
        if line.len() == 0 {
            section = 1;
            continue;
        }
        if section == 0 {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    walls.insert((x as isize, y as isize));
                } else if c == 'O' {
                    boxes.insert((x as isize, y as isize));
                } else if c == '@' {
                    robot = (x as isize, y as isize);
                }
            }   
        } else {
            let mut cur_moves: Vec<char> = line.chars().collect();
            moves.append(&mut cur_moves);
        }
    }
    let warehouse = Warehouse { walls, boxes, robot };
    let setup_time = start.elapsed();
    println!("Setup: {:.2?}", setup_time);
    
    start = Instant::now();
    solve_part_one(&mut warehouse.clone(), &moves);
    let p1_time = start.elapsed();
    println!("Part 1: {:.2?}", setup_time + p1_time);
    
    start = Instant::now();
    solve_part_two(&mut warehouse.clone(), &moves);
    let p2_time = start.elapsed();
    println!("Part 2: {:.2?}", setup_time + p2_time);
}

fn solve_part_one(warehouse: &mut Warehouse, moves: &Vec<char>) {
    let mut solution = 0;
    for m in moves {
        let diff: (isize, isize) = match m {
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            _ => panic!("Invalid move"),
        };
        let mut shift_boxes: HashSet<(isize, isize)> = HashSet::new();
        let mut next_pos = (warehouse.robot.0 + diff.0, warehouse.robot.1 + diff.1);
        loop {
            if warehouse.boxes.contains(&next_pos) {
                shift_boxes.insert(next_pos);
                next_pos = (next_pos.0 + diff.0, next_pos.1 + diff.1);
            } else if warehouse.walls.contains(&next_pos) {
                shift_boxes.clear();
                break;
            } else {
                warehouse.robot = (warehouse.robot.0 + diff.0, warehouse.robot.1 + diff.1);
                break;
            }
        }
        for b in &shift_boxes {
            warehouse.boxes.remove(b);
        }

        for b in &shift_boxes {
            warehouse.boxes.insert((b.0 + diff.0, b.1 + diff.1));
        }
    }
    for b in &warehouse.boxes {
        solution += 100 * b.1 + b.0;
    }
    println!("Day 14 Part 1: {}", solution);
}

fn solve_part_two(warehouse: &mut Warehouse, moves: &Vec<char>) {
    let mut solution = 0;

    println!("Day 14 Part 2: {}", solution);
}

#[cfg(test)]
mod tests {
    use super::*;

}