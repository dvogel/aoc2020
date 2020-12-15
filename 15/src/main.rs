use std::collections::VecDeque;
use std::collections::HashMap;

fn solve(input: &Vec<u32>, n_turns: u32) -> u32 {
    let mut mem: HashMap<u32, VecDeque<u32>> = HashMap::new();
    for (idx, &num) in input.iter().enumerate() {
        let mut turns: VecDeque<u32> = VecDeque::new();
        turns.push_front(idx as u32 + 1);
        mem.insert(num, turns);
    }

    let mut turn: u32 = input.len() as u32 + 1;
    let mut prev_value: u32 = input[input.len() - 1];
    while turn <= n_turns {
        let turns_for_prev = mem.get_mut(&prev_value).unwrap();
        let to_say = match turns_for_prev.len() {
            0 => panic!("This should never happen."),
            1 => 0,
            _ => {
                turns_for_prev[0] - turns_for_prev[1]
            }
        };

        match mem.get_mut(&to_say) {
            Some(turns_for_curr) => { turns_for_curr.push_front(turn); },
            None => { 
                let mut turns = VecDeque::new();
                turns.push_front(turn);
                mem.insert(to_say, turns);
            },
        };

        prev_value = to_say;
        turn += 1;
    }

    return prev_value;
}

fn main() {
    let input: Vec<u32> = vec![20,0,1,11,6,3];
    let answer1 = solve(&input, 2020);
    println!("Part 1: {}", answer1);

    let answer2 = solve(&input, 30000000);
    println!("Part 2: {}", answer2);
}

mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input: Vec<u32> = vec![0, 3, 6];
        let answer = solve(&input, 2020);
        assert_eq!(answer, 436);
    }

    #[test]
    fn test_example_2_1() {
        let input: Vec<u32> = vec![0, 3, 6];
        let answer = solve(&input, 30000000);
        assert_eq!(answer, 175594);
    }
}
