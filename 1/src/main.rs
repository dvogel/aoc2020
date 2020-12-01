use std::process;
use std::fs;

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
            Ok(n) => n,
            Err(err) => {
                eprintln!("error: {:?}", err);
                process::exit(1);
            }
        }
    ).collect();

    for i in numbers.iter() {
        for j in numbers.iter() {
            if (i + j == 2020) {
                let k = i * j;
                println!("{} * {} = {}", i, j, k);
                process::exit(0);
            }
        }
    }

    eprintln!("error: no answer found.");
    process::exit(1);
}
