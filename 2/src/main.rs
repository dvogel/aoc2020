use std::process;
use std::fs;

use itertools::Itertools;

fn main() {
    let filename = "input";
    let contents = match fs::read_to_string(filename) {
        Ok(istr) => istr,
        Err(err) => {
            eprintln!("error: {:?}", err);
            process::exit(1);
        }
    };

    let numbers: Vec<i32> = contents.split_whitespace().map(
        |istr|

        match istr.parse::<i32>() {
            Ok(n) => n.clone(),
            Err(err) => {
                eprintln!("error: {:?}", err);
                process::exit(1);
            }
        }
    ).collect();

    for candidates in numbers.into_iter().combinations(3) {
        let sum: i32 = candidates.clone().into_iter().sum();
        if sum == 2020 {
            let product: i32 = candidates.clone().into_iter().product();
            let str_candidates: Vec<String> = candidates.iter().map(|n| n.to_string()).collect();
            println!("{} = {}", str_candidates.join(" * "), product);
            process::exit(0);
        }
    }

    eprintln!("error: no answer found.");
    process::exit(1);
}
