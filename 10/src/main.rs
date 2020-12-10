use std::fs;

fn slurp_input(filename: &str) -> Vec<u32> {
    let contents = fs::read_to_string(filename).unwrap().to_string();
    let numbers = contents.lines().map(|x| x.parse::<u32>().unwrap()).collect();
    return numbers;
}

fn find_seq(joltages: &Vec<u32>) -> (bool, Vec<u32>) {
    let mut seq = joltages.to_vec();
    seq.sort();
    let mut prev_j: u32 = 0;
    for j in &seq {
        if j - prev_j > 3 {
            return (false, seq);
        }
        prev_j = *j;
    }
    seq.push(prev_j + 3);
    return (true, seq);
}

fn delta_histo(joltages: &Vec<u32>) -> Vec<u32> {
    let mut histo: Vec<u32> = vec![0, 0, 0];
    let mut prev_j: u32 = 0;
    for j in joltages {
        let delta = (j - prev_j) as usize;
        if delta >= 1 && delta <= 3 {
            histo[delta - 1] = histo[delta - 1] + 1;
        }
        prev_j = *j;
    }
    return histo;
}

fn main() {
    let joltages = slurp_input("input");
    let expected_seq = vec![1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22];
    let (valid, seq) = find_seq(&joltages);
    assert_eq!(valid, true);
    let histo: Vec<u32> = delta_histo(&seq);
    for (idx, n) in histo.iter().enumerate() {
        println!("{}: {}", idx + 1, n);
    }
    println!("{} * {} = {}", histo[0], histo[2], histo[0] * histo[2]);
    return;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let joltages = slurp_input("example1");
        let expected_seq = vec![1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22];
        let (valid, seq) = find_seq(&joltages);
        assert_eq!(valid, true);
        assert_eq!(seq, expected_seq);
        let histo: Vec<u32> = delta_histo(&seq);
        assert_eq!(histo[0], 7);
        assert_eq!(histo[1], 0);
        assert_eq!(histo[2], 5);
    }

    #[test]
    fn test_example2() {
        let joltages = slurp_input("example2");
        let (valid, seq) = find_seq(&joltages);
        assert_eq!(valid, true);
        let histo: Vec<u32> = delta_histo(&seq);
        assert_eq!(histo[0], 22);
        assert_eq!(histo[1], 0);
        assert_eq!(histo[2], 10);
    }
}
