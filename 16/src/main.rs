use std::fs;
use regex::Regex;

// Part 1: error_rate = 20048

struct PuzzleInput {
    categories: Vec<(String, Vec<(u32, u32)>)>,
    tickets: Vec<Vec<u32>>,
}

fn extract_category_name(ln: &str) -> String {
    let category_name_extract_pattern = Regex::new(r"(\w+):").expect("Valid regex.");
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
    let category_range_extract_pattern = Regex::new(r"\d+[-]\d+").expect("Valid regex.");

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

fn calc_puzzle_error_rate(puz: &PuzzleInput) -> u32 {
    let mut accum: u32 = 0;
    for ticket in &puz.tickets {
        'bump: for &x in ticket {
            for (_, cat_ranges) in &puz.categories {
                for range in cat_ranges {
                    if range.0 <= x && x <= range.1 {
                        continue 'bump;
                    }
                }
            }
            println!("ticket value {} is invalid", x);
            accum = accum + x;
        }
    }
    return accum;
}

fn main() {
    let puz = read_puzzle("input");
    let error_rate = calc_puzzle_error_rate(&puz);
    println!("Part 1: error_rate = {}", error_rate);
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
}

