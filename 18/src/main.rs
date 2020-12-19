use std::fs;

// Part 1: sum is 8929569623593

fn slurp_input(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).unwrap().to_string();
    let lines = contents.lines().map(|x| x.to_string()).collect();
    return lines;
}

fn eval_ltr_chars(chars: &Vec<char>, mut offset: &mut usize) -> i128
{
    let mut val: i128 = 0;
    let mut curr_op = '+';

    while *offset < chars.len() {
        let ch = chars[*offset];
        match ch {
            '(' => {
                *offset += 1;
                if curr_op == '*' {
                    let mult_sub = eval_ltr_chars(&chars, &mut offset);
                    println!("{} = {} * {}", val * mult_sub, val, mult_sub);
                    val = val * mult_sub;
                } else if curr_op == '+' {
                    let add_sub = eval_ltr_chars(&chars, &mut offset);
                    println!("{} = {} + {}", val + add_sub, val, add_sub);
                    val = val + add_sub;
                }
            },
            ')' => {
                return val;
            },
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let next_val: i128 = (ch as i128) - ('0' as i128);
                if curr_op == '*' {
                    println!("{} = {} * {}", val * next_val, val, next_val);
                    val = val * next_val;
                } else if curr_op == '+' {
                    println!("{} = {} + {}", val + next_val, val, next_val);
                    val = val + next_val;
                }
            },
            '*' | '+' => curr_op = ch,
            ' ' => { /* no-op */ },
            _ => panic!("This should not happen."),
        }
        *offset += 1;
    }
    return val;
}

fn eval_ltr(text: &String) -> i128 {
    let chars: Vec<char> = text.chars().collect();
    return eval_ltr_chars(&chars, &mut 0);
}

fn main() {
    let lines = slurp_input("input");
    let answer1: i128 = lines.iter().map(|ln| {
        let val = eval_ltr(ln);
        println!("{} = {}", val, ln);
        return val;
    }).sum();
    println!("Part 1: sum is {}", answer1);

    let mut sum: i128 = 0;
    for ln in lines {
        let val = eval_ltr(&ln);
        sum += val;
    }
    println!("Part 1: sum is {}", sum);
}

mod tests {
    use super::*;

    #[test]
    fn test_example_basic_0() {
        let expr_text = "(2 * 3)";
        let result = eval_ltr(&expr_text.to_string());
        assert_eq!(result, 6);
    }

    #[test]
    fn test_example_0() {
        let expr_text = "1 + (2 * 3) + (4 * (5 + 6))";
        let result = eval_ltr(&expr_text.to_string());
        assert_eq!(result, 51);
    }

    #[test]
    fn test_example_1() {
        let expr_text = "2 * 3 + (4 * 5)";
        let result = eval_ltr(&expr_text.to_string());
        assert_eq!(result, 26);
    }

    #[test]
    fn test_example_2() {
        let expr_text = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let result = eval_ltr(&expr_text.to_string());
        assert_eq!(result, 437);
    }

    #[test]
    fn test_example_3() {
        let expr_text = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        let result = eval_ltr(&expr_text.to_string());
        assert_eq!(result, 12240);
    }

    #[test]
    fn test_example_4() {
        let expr_text = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let result = eval_ltr(&expr_text.to_string());
        assert_eq!(result, 13632);
    }

    #[test]
    fn test_example_real_last() {
        let expr_text = "4 + (8 * (4 + 5 + 5) + (4 + 8 * 5 + 2 * 5 * 8) + 2 * 9) * 9";
        let result = eval_ltr(&expr_text.to_string());
        assert_eq!(result, 210150);
    }

    #[test]
    fn test_example_real_random() {
        let expr_text = "7 * 8 + (4 + 2 + (8 + 9 * 7 * 3 + 6 * 9)) + 5 * 7";
        let result = eval_ltr(&expr_text.to_string());
        assert_eq!(result, 23338);
    }
}

