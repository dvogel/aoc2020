use std::fs;
use cgmath::Vector2;
use cgmath::Point2;
use cgmath::Deg;
use cgmath::{Rotation, Rotation2, Basis2};

// part 1:Facing S at (658, 824), with distance of 1482

const NORTH: char = 'N';
const SOUTH: char = 'S';
const EAST: char = 'E';
const WEST: char = 'W';

const FORWARD: char = 'F';
const LEFT: char = 'L';
const RIGHT: char = 'R';


enum Step {
    Move { vec: Vector2<i32>, },
    Rotate { deg: Deg<i32>, },
    Thrust { coeff: i32, },
}

impl Step {
    fn new_movement(vec: Vector2<i32>) -> Step {
        Step::Move{
            vec: vec,
        }
    }

    fn new_rotation(deg: i32) -> Step {
        Step::Rotate{
            deg: Deg(deg),
        }
    }

    fn new_forward(distance: i32) -> Step {
        Step::Thrust{
            coeff: distance,
        }
    }
}

fn new_compass_vec(dir: char, distance: i32) -> Vector2<i32> {
    match dir {
        NORTH => Vector2{x: 0, y: -distance},
        SOUTH => Vector2{x: 0, y: distance},
        WEST => Vector2{x: -distance, y: 0},
        EAST => Vector2{x: distance, y: 0},
        _ => panic!(),
    }
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
        NORTH | SOUTH | WEST | EAST => Step::new_movement(new_compass_vec(ch, num)),
        LEFT => Step::new_rotation(360 - num),
        RIGHT => Step::new_rotation(num),
        FORWARD => Step::new_forward(num),
        _ => panic!()
    };
}

fn decode_steps(lines: &Vec<String>) -> Vec<Step> {
    let steps: Vec<Step> = lines.iter().map(|ln| decode_step(ln)).collect();
    return steps;
}

fn rotate_right(start_dir: Vector2<i32>, degrees: Deg<i32>) -> Vector2<i32> {
    let basis: Basis2<f32> = Rotation2::from_angle(Deg(degrees.0 as f32));
    let tmp_vec = Vector2{x: start_dir.x as f32, y: start_dir.y as f32};
    let rot_vec = basis.rotate_vector(tmp_vec);
    return Vector2{x: rot_vec.x as i32, y: rot_vec.y as i32};
}

fn exec_step(step: &Step, cur_dir: Vector2<i32>, pos: Point2<i32>) -> (Vector2<i32>, Point2<i32>) {
    match step {
        Step::Move{ vec } => (cur_dir, pos + vec),
        Step::Rotate{ deg} => (rotate_right(cur_dir, *deg), pos),
        Step::Thrust{ coeff } => {
            let new_step = Step::new_movement(cur_dir * *coeff);
            exec_step(&new_step, cur_dir, pos)
        },
        _ => panic!()
    }
}

fn exec_steps(steps: &Vec<Step>) -> (Vector2<i32>, Point2<i32>) {
    let mut dir = new_compass_vec(EAST, 1);
    let mut pos = Point2{x: 0, y: 0};

    for step in steps {
        let (new_dir, new_pos) = exec_step(step, dir, pos);
        println!("({}, {}) ==> ({}, {})", pos.x, pos.y, new_pos.x, new_pos.y);
        pos = new_pos;
        dir = new_dir;
    }

    return (dir, pos);
}

fn main() {
    let lines = slurp_input("input");
    let steps: Vec<Step> = decode_steps(&lines);
    let (dir, pos) = exec_steps(&steps);
    println!("Facing ({}, {}) at ({}, {}), with distance of {}", dir.x, dir.y, pos.x, pos.y, pos.x.abs() + pos.y.abs());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let lines = slurp_input("example");
        let steps: Vec<Step> = decode_steps(&lines);
        let (dir, pos) = exec_steps(&steps);
        assert_eq!(pos.x.abs(), 17);
        assert_eq!(pos.y.abs(), 8);
        assert_eq!(pos.x.abs() + pos.y.abs(), 25);
    }
}
