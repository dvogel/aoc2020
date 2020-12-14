use std::fs;
use std::cmp::Ordering;
use num::integer::lcm;

// Part 1: earliest valid departure time: 4938
// Part 2: earliest aligned departure: 230903629977901

fn slurp_input1(filename: &str) -> (u32, Vec<u32>) {
    let contents = fs::read_to_string(filename).unwrap().to_string();
    let mut lines = contents.lines();
    let arrival_time: u32 = lines.next()
        .expect("Arrival time required on first line.")
        .parse::<u32>()
        .expect("Numeric arrival time required.");

    let bus_times: Vec<u32> = lines.next()
        .expect("Bus departure times on second line.")
        .split_terminator(",")
        .map(|s| s.parse::<u32>())
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect();
    return (arrival_time, bus_times);
}

struct BusSched {
    bus_id: u32,
    offset: u32,
}

fn slurp_input2(filename: &str) -> Vec<BusSched> {
    let contents = fs::read_to_string(filename).unwrap().to_string();
    let bus_times: Vec<BusSched> = contents.lines().nth(1)
        .expect("Bus departure times on second line.")
        .split_terminator(",")
        .map(|s| s.parse::<u32>())
        .enumerate()
        .filter(|(idx, r)| r.is_ok())
        .map(|(idx, r)| {
            BusSched{ bus_id: r.unwrap(), offset: idx as u32  }
        })
        .collect();
    return bus_times;
}

fn find_earliest_valid_departure(arrival_time: u32, departure_times: &Vec<u32>) -> (u32, u32) {
    let valid_departures: Vec<(u32, u32)> = departure_times.iter()
        .map(|n| {
            let f_arrival = arrival_time as f64;
            let f_n = *n as f64;
            let next_dep = ((f_arrival / f_n).ceil() * f_n) as u32;
            (*n, next_dep)
        })
        .filter(|(_, next_dep)| next_dep >= &arrival_time)
        .collect();

    let (id, dep_time) = valid_departures.into_iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .expect("At least one departure time needed.");
    return (id, dep_time);
}

fn find_earliest_solution(bus_scheds: &Vec<BusSched>) -> u64 {
    let mut t: u64 = 0;
    let mut step: u64 = bus_scheds[0].bus_id as u64;

    for nth_sched in bus_scheds {
        while (t + nth_sched.offset as u64) % (nth_sched.bus_id as u64) != 0 {
            t = t + step
        }
        println!("t = {}", t);
        step = lcm(step, nth_sched.bus_id as u64);
        println!("step = {}", step);
    }

    return t;
}

fn main() {
    let (arrival, bus_times) = slurp_input1("input");
    let (id, dep_time) = find_earliest_valid_departure(arrival, &bus_times);
    let wait = dep_time - arrival;
    println!("Part 1: earliest valid departure time: {}", wait * id);

    let bus_scheds = slurp_input2("input");
    let solution2 = find_earliest_solution(&bus_scheds);
    println!("Part 2: earliest aligned departure: {}", solution2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let (arrival, bus_times) = slurp_input1("example");
        let (id, dep_time) = find_earliest_valid_departure(arrival, &bus_times);
        assert_eq!(arrival, 939);
        assert_eq!(dep_time, 944);
        let wait = dep_time - arrival;
        assert_eq!(wait, 5);
        let answer = wait * id;
        assert_eq!(answer, 295);
    }

    #[test]
    fn test_example2() {
        let bus_scheds = slurp_input2("example");
        assert_eq!(bus_scheds.len(), 5);
        assert_eq!(bus_scheds[0].bus_id, 7);
        assert_eq!(bus_scheds[0].offset, 0);
        assert_eq!(bus_scheds[bus_scheds.len() - 1].bus_id, 19);
        assert_eq!(bus_scheds[bus_scheds.len() - 1].offset, 7);
    }

    #[test]
    fn test_solver2_1() {
        let bus_scheds = slurp_input2("example");
        let solution = find_earliest_solution(&bus_scheds);
        assert_eq!(solution, 1068781);
    }

    #[test]
    fn test_solver2_2() {
        let bus_scheds = slurp_input2("example2");
        let solution = find_earliest_solution(&bus_scheds);
        assert_eq!(solution, 3417);
    }

    #[test]
    fn test_solver2_3() {
        let bus_scheds = slurp_input2("example3");
        let solution = find_earliest_solution(&bus_scheds);
        assert_eq!(solution, 754018);
    }

    #[test]
    fn test_solver2_4() {
        let bus_scheds = slurp_input2("example4");
        let solution = find_earliest_solution(&bus_scheds);
        assert_eq!(solution, 779210);
    }
}
