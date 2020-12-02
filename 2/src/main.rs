use std::fs;
use std::process;
use std::str;

use regex::Captures;
use regex::Regex;


struct FreqPolicy {
    min_count: u8,
    max_count: u8,
    focus_char: char,
}

struct PositionPolicy {
    pos0: usize,
    pos1: usize,
    focus_char: char,
}

struct EvalResult {
    eval_count: u32,
    error_count: u32,
}

trait Policy {
    fn check_conformance(&self, password: &str) -> bool;
}

fn make_eval_result() -> EvalResult {
    return EvalResult{
        eval_count: 0,
        error_count: 0
    };
}

impl EvalResult {
    fn eval<F>(&mut self, callback: F) -> bool
        where F: Fn() -> bool
    {

        self.eval_count = self.eval_count + 1;
        let result = callback();
        if !result {
            self.error_count = self.error_count + 1;
        }
        return result;
    }

    fn report(&self, prefix: &str) {
        println!("{}{} evals with {} successes and {} errors",
                 prefix,
                 self.eval_count,
                 self.eval_count - self.error_count,
                 self.error_count);
    }
}

fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename).unwrap();
    let lines = contents.lines();
    let pattern = Regex::new(r"^([0-9]+)-([0-9]+) (.): (.+)$").unwrap();
    let mut freq_results = make_eval_result();
    let mut pos_results = make_eval_result();

    for ln in lines {
        let cap = match pattern.captures(ln) {
            Some(cap) => cap,
            None => {
                eprintln!("error: no regex match for '{:?}'", ln);
                process::exit(1);
            }
        };

        let freq_pol: FreqPolicy = make_freq_policy(&cap);
        let pos_pol: PositionPolicy = make_position_policy(&cap);
        let password = cap.get(4).unwrap().as_str();

        freq_results.eval(|| freq_pol.check_conformance(password));
        pos_results.eval(|| pos_pol.check_conformance(password));
    }

    freq_results.report("FREQ: ");
    pos_results.report("POS: ");
}

fn make_position_policy(cap: &Captures) -> PositionPolicy {
    return PositionPolicy{
        pos0: cap.get(1).unwrap().as_str().parse::<usize>().unwrap(),
        pos1: cap.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        focus_char: cap.get(3).unwrap().as_str().chars().nth(0).unwrap(),
    };
}

impl Policy for PositionPolicy {
    fn check_conformance(&self, password: &str) -> bool {
        let mut matches = 0;

        for (pos, ch) in password.char_indices() {
            let norm_pos = pos + 1;
            if (ch == self.focus_char) && ((norm_pos == self.pos0) || (norm_pos == self.pos1)) {
                matches = matches + 1;
            }
        }

        return matches == 1;
    }
}

fn make_freq_policy(cap: &Captures) -> FreqPolicy {
    let policy: FreqPolicy = FreqPolicy{
        min_count: cap.get(1).unwrap().as_str().parse::<u8>().unwrap(),
        max_count: cap.get(2).unwrap().as_str().parse::<u8>().unwrap(),
        focus_char: cap.get(3).unwrap().as_str().chars().nth(0).unwrap(),
    };

    return policy;
}

impl Policy for FreqPolicy {
    fn check_conformance(&self, password: &str) -> bool {
        let mut count = 0;
        for ch in password.chars() {
            if ch == self.focus_char {
                count = count + 1;
            }
        }
        return (self.min_count <= count) && (count <= self.max_count);
    }
}

