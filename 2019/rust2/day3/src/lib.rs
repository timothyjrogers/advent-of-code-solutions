use std::cmp::{min, max};

const INPUT_FILE: &str = "input3.txt";

pub fn solve() {
    let raw_input: Vec<String> = aoc_utils::input::load_input_lines(INPUT_FILE);

    solve_part_one(&raw_input);
    solve_part_two(&raw_input);
}

struct Segment {
    start: (i32, i32),
    end: (i32, i32),
}

impl Segment {
    fn new(start: (i32, i32), end: (i32, i32)) -> Self {
        Segment { start, end, }
    }

    fn is_vertical(&self) -> bool {
        return self.start.0 == self.end.0;
    }
    
    fn is_horizontal(&self) -> bool {
        return self.start.1 == self.end.1;
    }

    fn contains_point(&self, point: (i32, i32)) -> bool {
        return point.0 >= self.start.0 && point.0 <= self.end.0 && point.1 >= self.start.1 && point.1 <= self.end.1;
    }

    fn length(&self) -> u32 {
        if self.is_horizontal() {
            return (self.start.0 - self.end.0).abs() as u32;
        }
        return (self.start.1 - self.end.1).abs() as u32;
    }
}


fn solve_part_one(input: &Vec<String>) {
    let mut paths = parse_input_lines(input);

    let mut closest_intersection_distance = u32::MAX;
    for intersection in get_all_intersections(paths.get(0).unwrap(), paths.get(1).unwrap()) {
        let distance = aoc_utils::math::manhattan_distance((0, 0), intersection);
        if distance < closest_intersection_distance {
            closest_intersection_distance = distance;
        }
    }
    println!("Day 3 Part 1: {}", closest_intersection_distance);
}

fn solve_part_two(input: &Vec<String>) {
    let mut paths = parse_input_lines(input);
    let intersections = get_all_intersections(paths.get(0).unwrap(), paths.get(1).unwrap());
    let mut shortest_path = u32::MAX;
    for intersection in intersections {
        let distance = steps_to_point(paths.get(0).unwrap(), intersection) + steps_to_point(paths.get(1).unwrap(), intersection);
        if distance < shortest_path {
            shortest_path = distance;
        }
    }
    println!("Day 3 Part 2: {}", shortest_path);
}

fn parse_input_lines(input: &Vec<String>) -> Vec<Vec<Segment>> {
    let mut paths: Vec<Vec<Segment>> = Vec::new();
    for line in input {
        let steps: Vec<&str> = line.split(",").collect();
        paths.push(create_path(steps));
    }
    return paths;
}

fn parse_step(step: &str) -> (i32, i32) {
    let distance = step[1..].parse::<i32>().unwrap();
    match step.get(0..1).unwrap() {
        "R" => return (distance, 0),
        "L" => return (distance * -1, 0),
        "U" => return (0, distance),
        "D" => return (0, distance * -1),
        _ => panic!("Unexpected step direction"),
    }
}

fn create_path(steps: Vec<&str>) -> Vec<Segment> {
    let mut path: Vec<Segment> = Vec::new();
    let mut cur_point: (i32, i32) = (0, 0);
    let mut prev_point: (i32, i32);
    for step in steps {
        prev_point = cur_point;
        let (dx, dy) = parse_step(step);
        cur_point = (cur_point.0 + dx, cur_point.1 + dy);
        let segment = Segment::new(prev_point, cur_point);
        path.push(segment);
    }
    return path;
}

fn get_all_intersections(path1: &Vec<Segment>, path2: &Vec<Segment>) -> Vec<(i32, i32)> {
    let mut intersections: Vec<(i32, i32)> = Vec::new();
    for segment1 in path1 {
        for segment2 in path2 {
            match get_segment_intersection(segment1, segment2) {
                Some(v) => {
                    if v != (0, 0) {
                        intersections.push(v);
                    }
                },
                None => ()
            }
        }
    }
    return intersections;
}

fn get_segment_intersection(segment1: &Segment, segment2: &Segment) -> Option<(i32,i32)> {
    if segment1.is_vertical() && segment2.is_vertical() {
        return None;
    }
    if segment1.is_horizontal() && segment2.is_horizontal() {
        return None;
    }
    if segment1.is_horizontal() {
        if segment2.start.0 >= min(segment1.start.0, segment1.end.0) && segment2.start.0 <= max(segment1.start.0, segment1.end.0) && segment1.start.1 >= min(segment2.start.1, segment2.end.1) && segment1.start.1 <= max(segment2.start.1, segment2.end.1) {
            return Some((segment2.start.0, segment1.start.1));
        }
    } else {
        if segment1.start.0 >= min(segment2.start.0, segment2.end.0) && segment1.start.0 <= max(segment2.start.0, segment2.end.0) && segment2.start.1 >= min(segment1.start.1, segment1.end.1) && segment2.start.1 <= max(segment1.start.1, segment1.end.1) {
            return Some((segment1.start.0, segment2.start.1));
        }
    }
    return None;
}

fn steps_to_point(path: &Vec<Segment>, point: (i32, i32)) -> u32 {
    let mut steps = 0;
    for segment in path {
        if segment.contains_point(point) {
            if segment.is_horizontal() {
                steps = steps + ((point.0 - segment.start.0).abs() as u32);
            } else {
                steps = steps + ((point.1 - segment.start.1).abs() as u32);
            }
            return steps;
        } else {
            steps = steps + segment.length();
        }
    }
    return steps;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_vertical_positive() {
        let segment = Segment::new((0, 10), (0, 20));
        assert_eq!(segment.is_vertical(), true);
    }

    #[test]
    fn test_is_vertical_negative() {
        let segment = Segment::new((0, 10), (10, 20));
        assert_eq!(segment.is_vertical(), false);
    }

    #[test]
    fn test_is_horizontal_positive() {
        let segment = Segment::new((10, 0), (20, 0));
        assert_eq!(segment.is_horizontal(), true);
    }

    #[test]
    fn test_is_horizontal_negative() {
        let segment = Segment::new((10, 0), (20, 10));
        assert_eq!(segment.is_horizontal(), false);
    }

    #[test]
    fn test_get_segment_intersection_positive() {
        let segment1 = Segment::new((0,0), (0,10));
        let segment2 = Segment::new((-5,5), (5,5));
        assert_eq!(get_segment_intersection(&segment1, &segment2), Some((0,5)));
    }
    
    #[test]
    fn test_get_segment_intersection_negative() {
        let segment1 = Segment::new((0,0), (0,10));
        let segment2 = Segment::new((-5,-5), (5,-5));
        assert_eq!(get_segment_intersection(&segment1, &segment2), None);
    }

    #[test]
    fn test_get_segment_intersection_both_vertical() {
        let segment1 = Segment::new((0,0), (0,10));
        let segment2 = Segment::new((5, 0), (5, 10));
        assert_eq!(get_segment_intersection(&segment1, &segment2), None);
    }

    #[test]
    fn test_get_segment_intersection_both_horizontal() {
        let segment1 = Segment::new((0, 0), (10, 0));
        let segment2 = Segment::new((0, 5), (10, 5));
        assert_eq!(get_segment_intersection(&segment1, &segment2), None);
    }

    #[test]
    fn test_parse_step_right() {
        let step = "R999";
        assert_eq!(parse_step(step), (999, 0));
    }

    #[test]
    fn test_parse_step_left() {
        let step = "L86";
        assert_eq!(parse_step(step), (-86, 0));
    }

    #[test]
    fn test_parse_step_up() {
        let step = "U464";
        assert_eq!(parse_step(step), (0, 464));
    }

    #[test]
    fn test_parse_step_down() {
        let step = "D666";
        assert_eq!(parse_step(step), (0, -666));
    }

    #[test]
    fn test_contains_point_positive() {
        let segment: Segment = Segment::new((0, 0), (0, 10));
        assert_eq!(segment.contains_point((0, 5)), true);
    }

    #[test]
    fn test_contains_point_negative() {
        let segment: Segment = Segment::new((0, 0), (0, 10));
        assert_eq!(segment.contains_point((0, 11)), false);
    }
}