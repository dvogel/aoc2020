use std::fs;
use std::collections::HashMap;
use regex::Regex;

// Part 1: valid count = 248

#[derive(Clone, Debug, Eq, PartialEq)]
enum Rule {
    Alt{ choices: Vec<Rule> },
    Seq{ rules: Vec<String> },
    Val{ letter: String },
}

struct Puzzle {
    rules: HashMap<String, Rule>,
    exprs: Vec<String>,
}

impl Puzzle {
    fn check_validity(&self, subject: &String) -> bool {
        let rule0 = self.rules.get(&String::from("0")).expect("Rule zero.");
        let (valid_prefix, remaining) = check_validity(&self.rules, &rule0, subject);
        return valid_prefix && remaining == String::from("");
    }
}

fn parse_rule(text: &String) -> Rule {
    let alterantives: Vec<&str> = text.split(r"|").collect();
    let alt_rules: Vec<Rule> = alterantives.iter().map(|alt_text| {
        if alt_text.len() == 3 && alt_text[0..1] == *"\"" && alt_text[2..3] == *"\"" {
            return Rule::Val{ letter: alt_text[1..2].to_string() };
        }

        let seq_rule_keys: Vec<String> = alt_text.trim().split_whitespace().map(|s| s.to_string() ).collect();
        Rule::Seq{ rules: seq_rule_keys }
    }).collect();

    if alt_rules.len() == 0 {
        panic!();
    } else if alt_rules.len() == 1 {
        return alt_rules[0].clone();
    } else {
        return Rule::Alt{
            choices: alt_rules,
        };
    }
}

fn read_rules(contents: &str) -> HashMap<String, Rule> {
    let rule_pattern = Regex::new(r"(\d+): (.*)").expect("Valid regex.");

    let mut rules: HashMap<String, Rule> = HashMap::new();
    for cap in rule_pattern.captures_iter(&contents) {
        // let idx = cap[1].parse::<u32>().expect("Positive number.");
        rules.insert(cap[1].to_string(), parse_rule(&cap[2].to_string()));
    }

    return rules
}

fn read_exprs(contents: &str) -> Vec<String> {
    let exprs: Vec<String> = contents.lines()
        .skip_while(|ln| ln.len() > 0)
        .skip_while(|ln| ln.len() == 0)
        .map(|ln| ln.to_string())
        .collect();
    exprs
}

fn read_puzzle(filename: &str) -> Puzzle {
    let contents = fs::read_to_string(filename).unwrap();
    let rules = read_rules(&contents);
    let exprs = read_exprs(&contents);
    Puzzle{
        rules: rules,
        exprs: exprs,
    }
}

fn check_validity(rules: &HashMap<String, Rule>, rule: &Rule, subject: &String) -> (bool, String) {
    match rule {
        Rule::Val{ letter, .. } => {
            if letter == &subject[0..1].to_string() {
                return (true, subject[1..].to_string());
            } else {
                return (false, subject.clone());
            }
        },
        Rule::Seq{ rules: sub_rules, .. } => {
            let mut subject1: String = subject.clone();
            for k in sub_rules {
                let rule1 = rules.get(k).expect("Known rule.");
                let (success, remaining) = check_validity(rules, &rule1, &subject1);
                if success {
                    subject1 = remaining;
                } else {
                    return (false, subject.clone());
                }
            }
            return (true, subject1);
        },
        Rule::Alt{ choices, .. } => {
            for r in choices {
                let (success, remaining) = check_validity(rules, &r, &subject);
                if success {
                    return (true, remaining);
                }
            }
            return (false, subject.clone());
        },
    }
}

fn main() {
    let puz = read_puzzle("input");
    let mut accum = 0;
    for e in &puz.exprs {
        if puz.check_validity(e) {
            accum += 1;
        }
    }

    println!("Part 1: valid count = {}", accum);
}

mod tests {
    use super::*;

    #[test]
    fn test_example_rules() {
        let puz = read_puzzle("example");
        assert_eq!(puz.rules.len(), 6);

        let rule0 = puz.rules.get(&String::from("0")).unwrap();
        assert_eq!(*rule0, Rule::Seq{ rules: vec![
            String::from("4"), String::from("1"), String::from("5"),
        ]});

        let rule5 = puz.rules.get(&String::from("5")).unwrap();
        assert_eq!(*rule5, Rule::Val{ letter: String::from("b") });

        assert_eq!(puz.exprs[0], "ababbb");

        let (validity_good0, _) = check_validity(&puz.rules, &rule0, &String::from("ababbb"));
        assert_eq!(validity_good0, true);

        let (validity_good1, _) = check_validity(&puz.rules, &rule0, &String::from("abbbab"));
        assert_eq!(validity_good1, true);

        let (validity_bad0, _) = check_validity(&puz.rules, &rule0, &String::from("bababa"));
        assert_eq!(validity_bad0, false);

        let (validity_bad1, _) = check_validity(&puz.rules, &rule0, &String::from("aaabbb"));
        assert_eq!(validity_bad1, false);

        let (validity_bad2, remaining) = check_validity(&puz.rules, &rule0, &String::from("aaaabbb"));
        assert_eq!(validity_bad2, true);
        assert_eq!(remaining, String::from("b"));

        let validity_bad2_prime = puz.check_validity(&String::from("aaaabbb"));
        assert_eq!(validity_bad2_prime, false);
    }

    #[test]
    fn test_parse_rule_val() {
        let rule = parse_rule(&String::from("\"a\""));
        assert_eq!(rule, Rule::Val{ letter: "a".to_string() });
    }

    #[test]
    fn test_parse_rule_seq() {
        let rule = parse_rule(&String::from("1 2"));
        assert_eq!(rule, Rule::Seq{ rules: vec![
            String::from("1"),
            String::from("2"),
        ]});
    }

    #[test]
    fn test_parse_rule_alt() {
        let rule = parse_rule(&String::from("1 2 | 2 1"));
        assert_eq!(rule, Rule::Alt{ choices: vec![
            Rule::Seq{ rules: vec![
                String::from("1"),
                String::from("2"),
            ]},
            Rule::Seq{ rules: vec![
                String::from("2"),
                String::from("1"),
            ]},
        ]});
    }
}

