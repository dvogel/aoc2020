use std::fs;
use itertools::Itertools;

fn slurp_input(filename: &str) -> Vec<u128> {
    let contents = fs::read_to_string(filename).unwrap().to_string();
    let numbers = contents.lines().map(|x| x.parse::<u128>().unwrap()).collect();
    return numbers;
}

fn validate_seq(numbers: &Vec<u128>, win_size: usize) -> (bool, u128) {
    let mut last: u128;
    'outer: for idx in win_size..(numbers.len() - win_size) {
        last = numbers[idx];
        for pair in numbers[(idx - win_size)..idx].into_iter().combinations(2) {
            if pair[0] + pair[1] == last {
                continue 'outer;
            }
        }
        println!("BROKEN WINDOW:");
        for x in &numbers[(idx - win_size)..idx] {
            println!("{}", x);
        }
        return (false, last);
    }

    return (true, 0);
}

fn main() {
    let numbers = slurp_input("input");
    let (valid, last_num) = validate_seq(&numbers, 25);
    if valid {
        println!("All valid.");
    } else {
        println!("Invalid sequence. First invalid value: {}", last_num);
    }
}
