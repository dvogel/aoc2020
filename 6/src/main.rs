use std::fs;
use std::collections::HashMap;


fn slurp_input() -> Vec<String> {
    let filename = "input";
    let contents = fs::read_to_string(filename).unwrap().to_string();
    let lines = contents.lines().map(|x| x.to_string()).collect();
    return lines;
}

fn group_lines(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut groups: Vec<Vec<String>> = vec![];
    let mut accum: Vec<String> = vec![];
    for ln in lines {
        if ln.len() == 0 {
            groups.push(accum);
            accum = vec![];
        } else {
            accum.push(ln);
        }
    }

    if accum.len() > 0 {
        groups.push(accum);
    }

    return groups;
}

fn count_answers_for_any(groups: &Vec<Vec<String>>) -> u32 {
    let mut accum: HashMap<char, (u32, usize)> = HashMap::new();
    for (grp_idx, grp) in groups.iter().enumerate() {
        let cur_idx1 = grp_idx + 1;
        for ln in grp {
            for ch in ln.chars() {
                let (cnt, last) = match accum.get(&ch) {
                    Some((x, y)) => (*x, *y),
                    None => (0, 0)
                };
                if cur_idx1 > last {
                    accum.insert(ch, (cnt + 1, cur_idx1));
                }
            }
        }
    }
    return accum.iter().map(|(ch, (cnt, _))| cnt).sum::<u32>();
}

fn count_answers_for_group(group: &Vec<String>) -> usize {
    let mut record_count: u32 = 0;
    let mut accum: HashMap<char, u32> = HashMap::new();
    for ln in group {
        let mut rec_chars: Vec<char> = ln.chars().collect();
        rec_chars.sort();
        rec_chars.dedup();
        for ch in rec_chars {
            let cnt = accum.get(&ch).cloned().unwrap_or(0);
            accum.insert(ch, cnt + 1);
        }
        record_count = record_count + 1;
    }

    return accum.values().cloned().filter(|x| *x == record_count).count();
}

fn count_answers_for_all(groups: &Vec<Vec<String>>) -> usize {
    return groups.iter().map(|grp| count_answers_for_group(grp)).sum();
}

fn main() {
    let lines = slurp_input();
    let groups = group_lines(lines);

    println!("Any: {}", count_answers_for_any(&groups));
    println!("All: {}", count_answers_for_all(&groups));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn countd_all_in_group_correctly() {
        let example: Vec<&str> = vec![
            "ab", "ac"
        ];
        let cnt = count_answers_for_group(&example.iter().map(|x| x.to_string()).collect());
        assert_eq!(cnt, 1);
    }
}
