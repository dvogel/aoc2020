use std::fs;
use std::process;

use regex::Regex;

struct Policy {
    min_count: u8,
    max_count: u8,
    focus_char: char,
}

fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename).unwrap();
    let lines = contents.lines();
    let pattern = Regex::new(r"^([0-9]+)-([0-9]+) (.): (.+)$").unwrap();
    let mut error_count = 0;
    let mut line_count = 0;

    for ln in lines {
        line_count = line_count + 1;

        let cap = match pattern.captures(ln) {
            Some(cap) => cap,
            None => {
                eprintln!("error: no regex match for '{:?}'", ln);
                process::exit(1);
            }
        };

        let policy: Policy = Policy{
            min_count: cap.get(1).unwrap().as_str().parse::<u8>().unwrap(),
            max_count: cap.get(2).unwrap().as_str().parse::<u8>().unwrap(),
            focus_char: cap.get(3).unwrap().as_str().chars().nth(0).unwrap(),
        };
        let password = cap.get(4).unwrap().as_str();
        if check_policy_conformance(&policy, password) {
            println!("PRISTINE: {}", ln);
        } else {
            println!("CORRUPT: {}", ln);
            error_count = error_count + 1;
        }
    }

    println!("{} lines", line_count);
    println!("{} corrupt passwords", error_count);
    println!("{} pristine passwords", line_count - error_count);
}

fn check_policy_conformance(policy: &Policy, password: &str) -> bool {
    let mut count = 0;
    for ch in password.chars() {
        if ch ==policy.focus_char {
            count = count + 1;
        }
    }
    return (policy.min_count <= count) && (count <= policy.max_count);
}

