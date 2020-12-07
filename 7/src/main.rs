use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn slurp_input(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).unwrap().to_string();
    let lines = contents.lines().map(|x| x.to_string()).collect();
    return lines;
}

fn parse_single_bag_rule(ln: String) -> (String, Vec<(u32, String)>) {
    let mut color: String = "".to_string();
    let mut valid_contents: Vec<(u32, String)> = vec![];
    let pattern = Regex::new(r"([0-9]+? )?((?:\w+) (?:\w+)) bag[s]?").unwrap();

    for (idx, cap) in pattern.captures_iter(ln.as_str()).enumerate() {
        if idx == 0 {
            color = cap[2].to_string();
        } else if &cap[2] == "no other" {
            // no-op special case.
        } else {
            let cnt: u32 = cap[1].trim().parse::<u32>().unwrap();
            valid_contents.push((cnt, cap[2].to_string()));
        }
    }
    return (color, valid_contents);
}

fn parse_bag_rules(lines: &Vec<String>) -> HashMap<String, Vec<(u32, String)>> {
    let mut validity_mapping: HashMap<String, Vec<(u32, String)>> = HashMap::new();
    for ln in lines {
        let (color, valid_contents) = parse_single_bag_rule(ln.clone());
        validity_mapping.insert(color, valid_contents);
    }
    return validity_mapping;
}

fn search_rule(rules: &HashMap<String, Vec<(u32, String)>>, search_color: &String, candidate_color: &String) -> u32 {
    let valid_contents: &Vec<(u32, String)> = match rules.get(candidate_color) {
        Some(x) => x,
        None => { return 0; }
    };

    for (cnt, color) in valid_contents {
        if color == search_color {
            return *cnt;
        }
    }

    for (cnt, color) in valid_contents {
        let sub_cnt = search_rule(rules, search_color, color);
        if sub_cnt > 0 {
            return cnt * sub_cnt;
        }
    }

    return 0;
}

fn accum_path_stats(rules: &HashMap<String, Vec<(u32, String)>>, candidate_color: &String, coeff: u32) -> u32 {
    let mut current_color = candidate_color;

    let valid_contents: &Vec<(u32, String)> = match rules.get(current_color) {
        Some(x) => x,
        None => { return coeff; }
    };

    let mut summables: Vec<u32> = vec![];
    for (cnt, color) in valid_contents {
        let sub_accum = accum_path_stats(rules, color, coeff * cnt);
        summables.push(sub_accum);
    }

    let sum: u32 = summables.iter().sum();
    if sum == 0 {
        return coeff;
    } else {
        return coeff + sum;
    };
}


fn main() {
    let lines = slurp_input("input");
    let validity_mapping = parse_bag_rules(&lines);
    let search_term = "shiny gold".to_string();
    let mut count: u32 = 0;
    for color in validity_mapping.keys().into_iter() {
        let cnt_on_path = search_rule(&validity_mapping, &search_term, color);
        if cnt_on_path > 0 {
            count = count + 1;
        }
    }
    println!("count: {}", count);

    let accum: u32 = accum_path_stats(&validity_mapping, &search_term, 1);
    println!("accum: {}", accum - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_rule_parse_test_no_others() {
        let (color, valid_contents) = parse_single_bag_rule("faded blue bags contain no other bags".to_string());
        assert_eq!(color, "faded blue");
        assert_eq!(valid_contents.len(), 0);
    }

    #[test]
    fn single_rule_parse_test_single_bag_contents() {
        let (color, valid_contents) = parse_single_bag_rule("bright white bags contain 1 shiny gold bags".to_string());
        assert_eq!(color, "bright white");
        assert_eq!(valid_contents.len(), 1);
    }

    #[test]
    fn single_rule_parse_test_multiple_bag_contents() {
        let (color, valid_contents) = parse_single_bag_rule("dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string());
        assert_eq!(color, "dark orange");
        assert_eq!(valid_contents.len(), 2);
        assert_eq!(valid_contents[0].1, "bright white".to_string());
        assert_eq!(valid_contents[1].1, "muted yellow".to_string());
    }

    #[test]
    fn accum_path_stats_test_example() {
        // TODO: This test fails because accum_path_stats is counting the root bag and is thus off
        // by 1.
        let rules = parse_bag_rules(&slurp_input("example"));
        let accum = accum_path_stats(&rules, &("shiny gold".to_string()), 1);
        assert_eq!(126, accum);
    }

    #[test]
    fn accum_path_stats_test_example1() {
        let rules = parse_bag_rules(&slurp_input("example1"));
        let accum = accum_path_stats(&rules, &("light red".to_string()), 1);
        assert_eq!(27, accum);
    }
}
