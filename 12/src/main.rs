use std::fs;
// use regex::Regex;

const NORTH: char = 'N';
const SOUTH: char = 'S';
const EAST: char = 'E';
const WEST: char = 'W';

const FORWARD: char = 'F';
const LEFT: char = 'L';
const RIGHT: char = 'R';

struct Step {
    direction: char,
    distance: i32,
}

fn slurp_input(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).unwrap().to_string();
    let lines = contents.lines().map(|x| x.to_string()).collect();
    return lines;
}

fn decode_step(ln: &String) -> Step {
    let mut ch_iter = ln.chars();
    let ch = ch_iter.next().unwrap();
    let num_str: String = ch_iter.collect();
    let num = num_str.parse::<i32>().unwrap();
    return match ch {
        NORTH | SOUTH | EAST | WEST => Step{direction: ch, distance: num},
        FORWARD | LEFT | RIGHT => Step{direction: ch, distance: num},
        _ => panic!()
    };
}

fn decode_steps(lines: &Vec<String>) -> Vec<Step> {
    let steps: Vec<Step> = lines.iter().map(|ln| decode_step(ln)).collect();
    return steps;
}

fn rotate_left_90(start_dir: char) -> char {
    match start_dir {
        NORTH => WEST,
        WEST => SOUTH,
        SOUTH => EAST,
        EAST => NORTH,
        _ => panic!()
    }
}

fn rotate_left(start_dir: char, degrees: i32) -> char {
    let mut dir = start_dir;
    let mut deg = degrees;
    while deg > 0 {
        dir = rotate_left_90(dir);
        deg = deg - 90;
    }
    return dir;
}

fn rotate_right(start_dir: char, degrees: i32) -> char {
    return rotate_left(start_dir, 360 - degrees);
}

fn exec_step(step: &Step, cur_dir: char, x: i32, y: i32) -> (char, i32, i32) {
    if step.direction == 'N' {
        return (cur_dir, x, y - step.distance);
    } else if step.direction == 'S' {
        return (cur_dir, x, y + step.distance);
    } else if step.direction == 'W' {
        return (cur_dir, x - step.distance, y);
    } else if step.direction == 'E' {
        return (cur_dir, x + step.distance, y);
    } else if step.direction == 'F' {
        return exec_step(&Step{direction: cur_dir, distance: step.distance}, cur_dir, x, y);
    } else if step.direction == 'L' {
        let new_dir = rotate_left(cur_dir, step.distance);
        return (new_dir, x, y);
    } else if step.direction == 'R' {
        let new_dir = rotate_right(cur_dir, step.distance);
        return (new_dir, x, y);
    } else {
        panic!();
    }
}

fn exec_steps(steps: &Vec<Step>) -> (char, i32, i32) {
    let mut dir = EAST;
    let mut x = 0;
    let mut y = 0;

    for step in steps {
        let (new_dir, new_x, new_y) = exec_step(step, dir, x, y);
        x = new_x;
        y = new_y;
        dir = new_dir;
    }

    return (dir, x, y);
}

fn main() {
    let lines = slurp_input("input");
    let steps: Vec<Step> = decode_steps(&lines);
    let (dir, x, y) = exec_steps(&steps);
    println!("Facing {} at ({}, {}), with distance of {}", dir, x, y, x.abs() + y.abs());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let lines = slurp_input("example");
        let steps: Vec<Step> = decode_steps(&lines);
        let (dir, x, y) = exec_steps(&steps);
        assert_eq!(x.abs() + y.abs(), 25);
    }
}
