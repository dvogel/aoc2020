use std::fs;
use std::collections::HashMap;
use regex::Regex;

// Part 1: sum = 13476250121721
// Part 2: sum = 4463708436768

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

fn run_program_v1(program: &Vec<Instr>) -> HashMap<u64, u64> {
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

fn x_mask_for_addr(mask: &String, addr: u64) -> Vec<char> {
    let addr1 = addr | Instr::new_or_mask(mask);

    let padded_mask = format!("{:0>36}", mask);
    let mask_chars = padded_mask.chars();
    let addr1_bin = format!("{:036b}", addr1);
    let addr1_bin_chars = addr1_bin.chars();

    let mut new_chars: Vec<char> = vec![];
    for (ac, mc) in addr1_bin_chars.zip(mask_chars) {
        if mc == '0' {
            new_chars.push(ac);
        } else if mc == '1' {
            new_chars.push('1');
        } else if mc == 'X' {
            new_chars.push('X');
        }
    }

    return new_chars;
}

fn enumerate_addresses(mask: &String, addr: u64) -> Vec<u64> {
    let x_mask = x_mask_for_addr(&mask, addr);
    let mut accum = vec![];
    enumerate_addresses_rec(&x_mask, 0, 0, &mut accum);
    return accum;
}

fn enumerate_addresses_rec(x_mask: &Vec<char>, pos: usize, num: u64, mut accum: &mut Vec<u64>) {
    if pos == x_mask.len() {
        let num_copy: u64 = num;
        accum.push(num_copy);
        return;
    }

    if x_mask[pos] == 'X' || x_mask[pos] == '0' {
        let new_num0 = num << 1;
        enumerate_addresses_rec(&x_mask, pos + 1, new_num0, &mut accum);
    }

    if x_mask[pos] == 'X' || x_mask[pos] == '1' {
        let new_num1 = num << 1 | 1;
        enumerate_addresses_rec(&x_mask, pos + 1, new_num1, &mut accum);
    }
}

fn run_program_v2(program: &Vec<Instr>) -> HashMap<u64, u64> {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask: String = String::new();
    for instr in program {
        match instr {
            Instr::SetMask{ mask_str: mask_str1, ..} => {
                mask = mask_str1.clone();
            },
            Instr::SetMem{ addr, val, } => {
                let addrs = enumerate_addresses(&mask, *addr);
                for addr in addrs {
                    mem.insert(addr, *val);
                }
            }
        }
    }
    return mem;
}

fn main() {
    let lines = slurp_input("input");
    let instructions: Vec<Instr> = lines.iter().map(|ln| parse_instr(&ln)).collect();
    let mem1 = run_program_v1(&instructions);
    let answer1: u64 = mem1.values().sum();
    println!("Part 1: sum = {}", answer1);

    let mem2 = run_program_v2(&instructions);
    let answer2: u64 = mem2.values().sum();
    println!("Part 2: sum = {}", answer2);
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

        let mem1 = run_program_v1(&instructions);
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

    #[test]
    fn test_enumerate_addresses_rec1() {
        let x_mask: Vec<char> = "X1101X".chars().collect();
        let mut addrs: Vec<u64> = vec![];
        enumerate_addresses_rec(&x_mask, 0, 0, &mut addrs);
        addrs.sort();
        assert_eq!(addrs.len(), 4);
        assert_eq!(addrs[0], 26);
        assert_eq!(addrs[1], 27);
        assert_eq!(addrs[2], 58);
        assert_eq!(addrs[3], 59);
    }

    #[test]
    fn test_enumerate_addresses_rec2() {
        let x_mask: Vec<char> = "1X0XX".chars().collect();
        let mut addrs: Vec<u64> = vec![];
        enumerate_addresses_rec(&x_mask, 0, 0, &mut addrs);
        addrs.sort();
        assert_eq!(addrs.len(), 8);
        assert_eq!(addrs[0], 16);
        assert_eq!(addrs[1], 17);
        assert_eq!(addrs[2], 18);
        assert_eq!(addrs[3], 19);
        assert_eq!(addrs[4], 24);
        assert_eq!(addrs[5], 25);
        assert_eq!(addrs[6], 26);
        assert_eq!(addrs[7], 27);
    }

    #[test]
    fn test_x_mask_for_addr() {
        let mask = "X1001X".to_string();
        let addr = 42;
        let x_mask: Vec<char> = x_mask_for_addr(&mask, addr);
        println!("x_mask = {:?}", x_mask);
        assert_eq!(x_mask.len(), 36);
        assert_eq!(x_mask[30], 'X');
        assert_eq!(x_mask[31], '1');
        assert_eq!(x_mask[32], '1');
        assert_eq!(x_mask[33], '0');
        assert_eq!(x_mask[34], '1');
        assert_eq!(x_mask[35], 'X');
    }

    #[test]
    fn test_example2() {
        let lines = slurp_input("example2");
        let instructions: Vec<Instr> = lines.iter().map(|ln| parse_instr(&ln)).collect();
        assert_eq!(instructions[0], Instr::new_set_mask("000000000000000000000000000000X1001X"));
        assert_eq!(instructions[1], Instr::SetMem{ addr: 42, val: 100 });
        assert_eq!(instructions[2], Instr::new_set_mask("00000000000000000000000000000000X0XX"));
        assert_eq!(instructions[3], Instr::SetMem{ addr: 26, val: 1 });

        let mem1 = run_program_v2(&instructions);
        assert_eq!(mem1.get(&58_u64).unwrap(), &100_u64);
        assert_eq!(mem1.get(&59_u64).unwrap(), &100_u64);
        assert_eq!(mem1.get(&24_u64).unwrap(), &1_u64);
        assert_eq!(mem1.get(&25_u64).unwrap(), &1_u64);
        assert_eq!(mem1.get(&26_u64).unwrap(), &1_u64);
        assert_eq!(mem1.get(&27_u64).unwrap(), &1_u64);
    }
}
