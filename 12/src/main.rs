use std::fs;
use vecmath;

// part 1:Facing S at (658, 824), with distance of 1482

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

struct Pos {
    x: i32,
    y: i32,
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

fn exec_step(step: &Step, cur_dir: char, pos: Pos) -> (char, Pos) {
    if step.direction == NORTH {
        return (cur_dir, Pos{x: pos.x, y: pos.y - step.distance});
    } else if step.direction == SOUTH {
        return (cur_dir, Pos{x: pos.x, y: pos.y + step.distance});
    } else if step.direction == WEST {
        return (cur_dir, Pos{x: pos.x - step.distance, y: pos.y});
    } else if step.direction == EAST {
        return (cur_dir, Pos{x: pos.x + step.distance, y: pos.y});
    } else if step.direction == FORWARD {
        return exec_step(&Step{
            direction: cur_dir,
            distance: step.distance},
            cur_dir,
            pos);
    } else if step.direction == LEFT {
        let new_dir = rotate_left(cur_dir, step.distance);
        return (new_dir, pos);
    } else if step.direction == RIGHT {
        let new_dir = rotate_right(cur_dir, step.distance);
        return (new_dir, pos);
    } else {
        panic!();
    }
}

fn exec_steps(steps: &Vec<Step>) -> (char, Pos) {
    let mut dir = EAST;
    let mut pos = Pos{x: 0, y: 0};

    for step in steps {
        let (new_dir, new_pos) = exec_step(step, dir, pos);
        pos = new_pos;
        dir = new_dir;
    }

    return (dir, pos);
}

fn main() {
    let lines = slurp_input("input");
    let steps: Vec<Step> = decode_steps(&lines);
    let (dir, pos) = exec_steps(&steps);
    println!("Facing {} at ({}, {}), with distance of {}", dir, pos.x, pos.y, pos.x.abs() + pos.y.abs());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let lines = slurp_input("example");
        let steps: Vec<Step> = decode_steps(&lines);
        let (dir, pos) = exec_steps(&steps);
        assert_eq!(pos.x.abs() + pos.y.abs(), 25);
    }

    #[test]
    fn test2() {
        let lines = slurp_input("example");
        let steps: Vec<Step> = decode_steps(&lines);
        let (dir, pos) = exec_steps(&steps);
        assert_eq!(pos.x.abs() + pos.y.abs(), 286);
    }
}
