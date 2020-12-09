use std::fs;

struct Instr {
    op: String,
    operand: i32,
    exec_count: u32,
}

impl Clone for Instr {
    fn clone(&self) -> Self {
        Instr{
            op: String::from(&self.op),
            operand: self.operand,
            exec_count: self.exec_count,
        }
    }    
}

impl Instr {
    fn reset(&mut self) {
        self.exec_count = 0;
    }

    fn switch_to(&mut self, op: &str) {
        self.op = String::from(op);
    }
}

fn slurp_input(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).unwrap().to_string();
    let lines = contents.lines().map(|x| x.to_string()).collect();
    return lines;
}

fn parse_instruction(ln: &String) -> Instr {
    let pair: Vec<&str> = ln.splitn(2, " ").collect();
    let operand: i32 = pair[1].parse::<i32>().unwrap();
    let instr = Instr{
        op: String::from(pair[0]),
        operand: operand,
        exec_count: 0,
    };
    // let instr = match pair[0] {
    //     "nop" => Instr{op: String::from("nop"), operand: pair[, exec_count: 0},
    //     "acc" => Instr{op: String::from("acc"), operand: pair[1].parse::<i32>().unwrap(), exec_count: 0},
    //     "jmp" => Instr{op: String::from("jmp"), operand: pair[1].parse::<i32>().unwrap(), exec_count: 0},
    //     &_ => panic!()
    // };
    return instr;
}

fn parse_program(lines: &Vec<String>) -> Vec<Instr> {
    lines.iter().map(|x| parse_instruction(&x)).collect()
}

fn run_program(prog: &mut Vec<Instr>, exec_limit: u32, debug: bool) -> (bool, i32) {
    let mut ip: i32 = 0;
    let mut accum: i32 = 0;
    let mut fault_flag: bool = false;
    loop {
        if ip as usize == prog.len() {
            break;
        }

        let instr = prog.get(ip as usize).unwrap().clone();
        if instr.exec_count >= exec_limit {
            fault_flag = true;
            break;
        }

        prog[ip as usize] = Instr{
            op: String::from(&instr.op),
            operand: instr.operand,
            exec_count: instr.exec_count + 1
        };
        if debug {
            println!("will exec {}", instr.op);
        }
        if instr.op == "acc" {
            accum += instr.operand;
            ip = ip + 1;
        } else if instr.op == "jmp" {
            ip = ip + instr.operand;
        } else if instr.op == "nop" {
            ip = ip + 1;
        } else {
            panic!();
        }
    }

    return (!fault_flag, accum);
}

fn reset_program(prog: &mut Vec<Instr>) {
    for x in prog {
        x.reset();
    }
}

fn fix_program(prog: Vec<Instr>) -> (bool, Vec<Instr>) {
    for offset in 0..prog.len() {
        let mut prog1: Vec<Instr> = prog.to_vec();
        let mut instr = prog1[offset].clone();
        if instr.op == "acc" {
            println!("ignoring instr {} as it is acc", offset);
            continue;
        } else if instr.op == "jmp" {
            instr.switch_to("nop");
            prog1[offset] = instr.clone();
            println!("switched instr {} from jmp to {}", offset, &instr.op);
        } else if instr.op == "nop" {
            instr.switch_to("jmp");
            prog1[offset] = instr.clone();
            println!("switched instr {} from nop to {}", offset, &instr.op);
        }

        let (normal_exit, _) = run_program(&mut prog1, 1, false);
        if normal_exit {
            reset_program(&mut prog1);
            return (true, prog1);
        }
    }
    return (false, vec![]);
}

fn list_program(prog: &Vec<Instr>) {
    prog.iter().for_each(|x| println!("{} {}", x.op, x.operand));
}

fn main() {
    let lines = slurp_input("input");
    let prog: Vec<Instr> = parse_program(&lines);

    let (normal_exit1, part1) = run_program(&mut prog.to_vec(), 1, true);
    println!("part1: {}, normal? {}", part1, normal_exit1);
    println!("--------");

    let (is_fixed, prog2) = fix_program(prog.to_vec());
    if is_fixed {
        println!("--------");
        list_program(&prog2);
        println!("Fixed program. Running...");
        let (normal_exit2, part2) = run_program(&mut prog2.to_vec(), 1, true);
        println!("part2: {}, normal? {}", part2, normal_exit2);
    } else {
        println!("Program is unfixable.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(1, 1);
    }
}
