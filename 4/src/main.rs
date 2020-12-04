use regex::Regex;
use std::fs;
use std::collections::HashMap;

fn slurp_input() -> Vec<String> {
    let filename = "input";
    let contents = fs::read_to_string(filename).unwrap().to_string();
    let lines = contents.lines().map(|x| x.to_string()).collect();
    return lines;
}

fn parse_fields(ln: &String) -> HashMap<&str, &str> {
    let mut accum: HashMap<&str, &str> = HashMap::new();

    let fields: Vec<&str> = ln.split_terminator(' ').collect();
    for fld in fields.iter() {
        let pair: Vec<&str> = fld.splitn(2, ':').collect();
        accum.insert(pair[0], pair[1]);
    }

    return accum;
}

fn print_hash_map(map: &HashMap<&str, &str>) {
    for (k, v) in map.iter() {
        println!("{}: {}", k, v);
    }
}

fn validate_int_field(fld_val: &str, min: u32, max: u32) -> bool {
    let num = fld_val.parse::<u32>();
    match num {
        Ok(x) => return (min <= x) && (x <= max),
        Err(_) => false
    }
}

fn validate_byr(fld_val: &str) -> bool {
    return validate_int_field(fld_val, 1920, 2002);
}

fn validate_iyr(fld_val: &str) -> bool {
    return validate_int_field(fld_val, 2010, 2020);
}

fn validate_eyr(fld_val: &str) -> bool {
    return validate_int_field(fld_val, 2020, 2030);
}

fn validate_hgt(fld_val: &str) -> bool {
    let pattern = Regex::new(r"^([0-9]+)(cm|in)$").unwrap();
    let (num_str, unit) = match pattern.captures(fld_val) {
        Some(cap) => (cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str()),
        None => return false
    };

    let num_val = match num_str.parse::<u32>() {
        Ok(x) => x,
        Err(_) => return false
    };

    if unit == "cm" {
        return (150 <= num_val) && (num_val <= 193);
    } else if unit == "in" {
        return (59 <= num_val) && (num_val <= 76);
    } else {
        panic!()
    }
}

fn validate_hcl(fld_val: &str) -> bool {
    let pattern = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    return pattern.is_match(fld_val);
}

fn validate_ecl(fld_val: &str) -> bool {
    let pattern = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    return pattern.is_match(fld_val);
}

fn validate_pid(fld_val: &str) -> bool {
    let pattern = Regex::new(r"^\d{9}$").unwrap();
    return pattern.is_match(fld_val);
}

fn main() {
    let lines = slurp_input();
    let mut entries: Vec<HashMap<&str, &str>> = vec![];

    let mut accum: HashMap<&str, &str> = HashMap::new();
    // let accum_field = |k: &str, v: &str| { accum.insert(k, v); (); };

    for ln in lines.iter() {
        if ln.len() == 0 {
            // print_hash_map(&accum);
            entries.push(accum);
            accum = HashMap::new();
        } else {
            let entry_fields = parse_fields(ln);
            for (k, v) in entry_fields.iter() {
                accum.insert(k, v);
            }
        }
    }

    if accum.len() > 0 {
        entries.push(accum);
    }

    let mut required_fields: Vec<(&str, Box<dyn Fn(&str) -> bool>)> = Vec::new();
    required_fields.push(("byr", Box::new(&validate_byr)));
    required_fields.push(("iyr", Box::new(&validate_iyr)));
    required_fields.push(("eyr", Box::new(&validate_eyr)));
    required_fields.push(("hgt", Box::new(&validate_hgt)));
    required_fields.push(("hcl", Box::new(&validate_hcl)));
    required_fields.push(("ecl", Box::new(&validate_ecl)));
    required_fields.push(("pid", Box::new(&validate_pid)));

    let mut valid_count = 0;
    for entry in entries.iter() {
        let mut fld_count = 0;
        for (fld, validator) in required_fields.iter() {
            match entry.get(fld) {
                Some(x) => {
                    if validator(x) {
                        fld_count = fld_count + 1;
                    } else {
                        println!("E:{}:{}", fld, x);
                    }
                },
                None => {}
            }
        }
        if fld_count == required_fields.len() {
            valid_count = valid_count + 1;
        }
    }

    println!("valid: {} of {}", valid_count, entries.len());
}
