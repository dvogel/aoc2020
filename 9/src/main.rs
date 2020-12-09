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

fn find_window_summing_to(numbers: &Vec<u128>, win_size: usize, expected_sum: u128) -> (bool, usize) {
    for (idx, win) in numbers.windows(win_size).enumerate() {
        if win.iter().sum::<u128>() == expected_sum {
            return (true, idx);
        }
    }
    return (false, 0);
}

fn find_variable_window_summing_to(numbers: &Vec<u128>, expected_sum: u128) -> (bool, usize, usize) {
    for win_size in 2..999 {
        let (found, offset) = find_window_summing_to(numbers, win_size, expected_sum);
        if found {
            return (true, win_size, offset);
        }
    }
    return (false, 0, 0);
}

fn main() {
    let numbers = slurp_input("input");
    let (valid, last_num) = validate_seq(&numbers, 25);
    if valid {
        println!("All valid.");
    } else {
        println!("Invalid sequence. First invalid value: {}", last_num);
    }

    let (found, win_size, offset) = find_variable_window_summing_to(&numbers, last_num);
    if found {
        println!("TARGET WINDOW:");
        let target_win = &numbers[offset..(offset + win_size)];
        let min = target_win.iter().min().unwrap();
        let max = target_win.iter().max().unwrap();
        for x in target_win {
            println!("{}", x);
        }
        println!("{} + {} = {}", min, max, min + max);
    } else {
        println!("NO TARGET WINDOW FOUND");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let numbers = slurp_input("example");
        let (valid, last_num) = validate_seq(&numbers, 5);
        assert_eq!(valid, false);
        assert_eq!(last_num, 127);

        let (found, offset) = find_window_summing_to(&numbers, 4, last_num);
        assert_eq!(found, true);
        assert_eq!(offset, 2);

        let (found1, win_size, offset1) = find_variable_window_summing_to(&numbers, last_num);
        assert_eq!(win_size, 4);
        assert_eq!(offset, 2);
    }
}
