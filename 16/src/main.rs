use std::fs;
use regex::Regex;

// Part 1: error_rate = 20048

struct PuzzleInput {
    categories: Vec<(String, Vec<(u32, u32)>)>,
    tickets: Vec<Vec<u32>>,
}

fn extract_category_name(ln: &str) -> String {
    let category_name_extract_pattern = Regex::new(r"([\w ]+):").expect("Valid regex.");
    let name_match = category_name_extract_pattern.captures(ln);
    let cap = name_match.expect("A category name match at the beginning of the string.");
    return cap[1].to_string();
}

fn extract_category_ranges(ln: &str) -> Vec<(u32, u32)> {
    let mut accum: Vec<(u32, u32)> = Vec::new();
    let category_range_extract_pattern = Regex::new(r"(\d+)[-](\d+)").expect("Valid regex.");
    for cap in category_range_extract_pattern.captures_iter(ln) {
        let range_start = cap[1].parse::<u32>().expect("Integer string.");
        let range_end = cap[2].parse::<u32>().expect("Integer string.");
        accum.push((range_start, range_end));
    }
    return accum;
}

fn build_category(ln: &str) -> (String, Vec<(u32, u32)>) {
    let name = extract_category_name(ln);
    let ranges = extract_category_ranges(ln);
    return (name, ranges);
}

fn build_numeric_series(ln: &str) -> Vec<u32> {
    return ln.trim().split(",").map(|s| {
        s.parse::<u32>().expect("Integer string")
    }).collect();
}

fn read_puzzle(filename: &str) -> PuzzleInput {
    let numeric_series_pattern = Regex::new(r"\n((?:[,]?\d+)+)").expect("Valid regex.");
    let category_line_pattern = Regex::new(r"(?:[\w ]+): (?:(?: or )?(?:\d+[-]\d+))+").expect("Valid regex.");

    let contents = fs::read_to_string(filename).unwrap();
    let categories: Vec<(String, Vec<(u32, u32)>)> = category_line_pattern.captures_iter(&contents).map(|cap| {
        build_category(cap[0].trim())
    }).collect();

    let numeric_series: Vec<Vec<u32>> = numeric_series_pattern.captures_iter(&contents).map(|cap| {
        build_numeric_series(cap[0].trim())
    }).collect();

    PuzzleInput{
        categories: categories,
        tickets: numeric_series,
    }
}

fn value_is_valid(x: u32, categories: &Vec<(String, Vec<(u32, u32)>)>) -> bool {
    for (_, cat_ranges) in categories {
        for range in cat_ranges {
            if range.0 <= x && x <= range.1 {
                return true;
            }
        }
    }
    return false;
}

fn ticket_is_valid(ticket: &Vec<u32>, categories: &Vec<(String, Vec<(u32, u32)>)>) -> bool {
    for x in ticket.iter().copied() {
        if !value_is_valid(x, &categories) {
            return false;
        }
    }
    return true;
}

// TODO: A ticket_is_valid function needs to be extracted from this and then used to filter out
// invalid tickets for part 2.
fn calc_puzzle_error_rate(puz: &PuzzleInput) -> u32 {
    let mut accum: u32 = 0;
    for ticket in &puz.tickets {
        for x in ticket.iter().copied() {
            if value_is_valid(x, &puz.categories) {
                continue;
            }
            println!("ticket value {} is invalid", x);
            accum += x;
        }
    }
    return accum;
}

fn solve_for_field_positions(puz: &PuzzleInput) -> Vec<String> {
    let cat_names: Vec<String> = puz.categories.iter().map(|(cat_name, _)| cat_name.clone()).collect();
    let mut possible_fields_by_pos: Vec<Vec<String>> = (0..puz.categories.len()).map(|_| {
        cat_names.clone()
    }).collect();

    let valid_tickets: Vec<&Vec<u32>> = puz.tickets.iter().filter(|t| {
        ticket_is_valid(t, &puz.categories)
    }).collect();

    for ticket in &valid_tickets {
        for (idx, x) in ticket.iter().enumerate() {
            'cat_bump: for (cat_name, cat_ranges) in &puz.categories {
                for (range_start, range_end) in cat_ranges {
                    if range_start <= x && x <= range_end {
                        continue 'cat_bump;
                    }
                }
                println!("--------");
                println!("Value {} in position {} not valid for any range of category {}",
                    x, idx, cat_name);
                let fields_for_pos = possible_fields_by_pos[idx].clone();
                possible_fields_by_pos[idx] = fields_for_pos.into_iter().filter(|cn| {
                    cn != cat_name
                }).collect();
                for (idx, cat_names) in possible_fields_by_pos.iter().enumerate() {
                    println!("{}: {:?}", idx, cat_names);
                }
            }
        }
    }

    let mut updates = 1;
    while updates > 0 {
        updates = 0;

        let unique_fields: Vec<(usize, String)> = possible_fields_by_pos.iter().enumerate()
            .filter(|(_, cat_names)| cat_names.len() == 1)
            .map(|(idx, cat_names)| (idx, cat_names[0].clone()))
            .collect();

        for (unique_idx, unique_cat_name) in &unique_fields {
            for idx in 0..possible_fields_by_pos.len() {
                if idx != *unique_idx {
                    let pruned_possibilities: Vec<String> = possible_fields_by_pos[idx].clone().into_iter().filter(|cat_name| {
                        cat_name != unique_cat_name
                    }).collect();
                    if pruned_possibilities.len() < possible_fields_by_pos[idx].len() {
                        println!("Removing category {} from set for position {} because it is unique to position {}", unique_cat_name, idx, unique_idx);
                        possible_fields_by_pos[idx] = pruned_possibilities;
                        updates += 1;
                        println!("updates = {}", updates);
                    }
                }
            }
        }
    }

    let mut result: Vec<String> = Vec::new();
    for (idx, field_list) in possible_fields_by_pos.iter().enumerate() {
        if field_list.len() == 0 {
            panic!("Position {} has no possible categories.", idx);
        } else if field_list.len() > 1 {
            panic!("Position {} has more than 1 category ({})", idx, field_list.len());
        }
        result.push(field_list[0].clone());
    }

    return result;
}

fn main() {
    let puz = read_puzzle("input");
    let error_rate = calc_puzzle_error_rate(&puz);
    println!("Part 1: error_rate = {}", error_rate);
    
    let fields_by_pos = solve_for_field_positions(&puz);
    let mut accum: u64 = 1;
    for (idx, cat_name) in fields_by_pos.iter().enumerate() {
        if cat_name.starts_with("departure") {
            let value = puz.tickets[0][idx];
            println!("my ticket: {} = {}", cat_name, value);
            accum *= (value as u64);
        }
    }
    println!("Part 2: product = {}", accum);
}

mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let puz = read_puzzle("example1");
        for (nm, ranges) in &puz.categories {
            println!("name = {}", nm);
        }
        assert_eq!(puz.categories.len(), 3);
        assert_eq!(puz.tickets.len(), 5);

        let error_rate = calc_puzzle_error_rate(&puz);
        assert_eq!(error_rate, 71);
    }

    #[test]
    fn test_example1_build_category() {
        let ln = "class: 1-3 or 5-7";
        let (nm, ranges) = build_category(&ln);
        assert_eq!(nm, "class");
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0], (1, 3));
        assert_eq!(ranges[1], (5, 7));
    }

    #[test]
    fn test_example1_build_numeric_series1() {
        let series = build_numeric_series("7,1,14");
        assert_eq!(series.len(), 3);
        assert_eq!(series[0], 7_u32);
        assert_eq!(series[1], 1_u32);
        assert_eq!(series[2], 14_u32);
    }

    #[test]
    fn test_example2() {
        let puz = read_puzzle("example2");
        let solution = solve_for_field_positions(&puz);
        assert_eq!(solution[0], "row".to_string());
        assert_eq!(solution[1], "class".to_string());
        assert_eq!(solution[2], "seat".to_string());
    }
}

