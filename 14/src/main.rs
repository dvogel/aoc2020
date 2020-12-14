use std::fs;
use std::collections::HashMap;
use regex::Regex;

// Part 1: sum = 13476250121721 answer

#[derive(Debug,Eq,PartialEq)]
enum Instr {
    SetMask{ mask_str: String, and_mask: u64, or_mask: u64, },
    SetMem{ addr: u64, val: u64 },
}

impl Instr {
    fn new_and_mask(mask_str: &str) -> u64 {
        let mut mask: u64 = u64::MAX;
        for (idx, ch) in mask_str.chars().rev().enumerate() {
            if ch == '0' {
                let update_mask = (u64::MAX - 1).rotate_left(idx as u32) & (2_u64.pow(36) - 1);
                mask = mask & update_mask;
            }
        }
        return mask;
    }

    fn new_or_mask(mask_str: &str) -> u64 {
        let mut mask: u64 = 0;
        for (idx, ch) in mask_str.chars().rev().enumerate() {
            if ch == '1' {
                let update_mask = 1_u64.rotate_left(idx as u32);
                mask = mask | update_mask;
            }
        }
        return mask;
    }

    fn new_set_mask(mask_str: &str) -> Instr {
        Instr::SetMask{
            mask_str: mask_str.to_string(),
            and_mask: Instr::new_and_mask(mask_str),
            or_mask: Instr::new_or_mask(mask_str),
        }
    }
}

fn slurp_input(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).unwrap().to_string();
    let lines = contents.lines().map(|x| x.to_string()).collect();
    return lines;
}

fn parse_instr(ln: &String) -> Instr {
    let mask_pattern = Regex::new(r"^mask = ([X01]+)$").unwrap();
    let mem_pattern = Regex::new(r"^mem\[([0-9]+)\] = ([0-9]+)$").unwrap();

    for cap in mask_pattern.captures_iter(ln.as_str()) {
        return Instr::new_set_mask(&cap[1]);
    }

    for cap in mem_pattern.captures_iter(ln.as_str()) {
        return Instr::SetMem{
            addr: cap[1].parse::<u64>().unwrap(),
            val: cap[2].parse::<u64>().unwrap(),
        }
    }

    panic!("Unrecognized instruction: {:?}", ln);
}

fn run_program(program: &Vec<Instr>) -> HashMap<u64, u64> {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut and_mask: u64 = 0;
    let mut or_mask: u64 = 0;
    for instr in program {
        match instr {
            Instr::SetMask{ and_mask: and_mask1, or_mask: or_mask1, .. } => {
                and_mask = *and_mask1;
                or_mask = *or_mask1;
            },
            Instr::SetMem{ addr, val, } => {
                let modified_val = val & and_mask | or_mask;
                mem.insert(*addr, modified_val);
            }
        }
    }
    return mem;
}

fn main() {
    let lines = slurp_input("input");
    let instructions: Vec<Instr> = lines.iter().map(|ln| parse_instr(&ln)).collect();
    let mem1 = run_program(&instructions);
    let answer: u64 = mem1.values().sum();
    println!("Part 1: sum = {} answer", answer);
}

mod tests {
    use super::*;

    #[test]
    fn test_instr_set_mask_and() {
        let mask = Instr::new_and_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(mask, 0b111111111111111111111111111111111101);
    }

    #[test]
    fn test_instr_set_mask_or() {
        let mask = Instr::new_or_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(mask, 0b000000000000000000000000000001000000);
    }

    #[test]
    fn test_example1() {
        let lines = slurp_input("example");
        let instructions: Vec<Instr> = lines.iter().map(|ln| parse_instr(&ln)).collect();
        assert_eq!(instructions[0], Instr::new_set_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"));
        assert_eq!(instructions[1], Instr::SetMem{ addr: 8, val: 11 });
        assert_eq!(instructions[2], Instr::SetMem{ addr: 7, val: 101 });

        let mem1 = run_program(&instructions);
        for (k, v) in mem1.iter() {
            println!("mem[{}] = {}", k, v);
        }
        assert_eq!(mem1.get(&7_u64).unwrap(), &101_u64);
        assert_eq!(mem1.get(&8_u64).unwrap(), &64_u64);
    }

    #[test]
    fn test_rotate() {
        let x: u8 = u8::MAX - 1;
        assert_eq!(x, 0b11111110);
        assert_eq!(x.rotate_left(1), 0b11111101);
    }
}
