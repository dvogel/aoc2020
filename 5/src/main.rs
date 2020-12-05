use std::fs;

fn slurp_input() -> Vec<String> {
    let filename = "input";
    let contents = fs::read_to_string(filename).unwrap().to_string();
    let lines = contents.lines().map(|x| x.to_string()).collect();
    return lines;
}

struct SeatCoord {
    row: u8,
    col: u8,
}

fn decode_seat(ln: &str) -> SeatCoord {
    let mut num: u16 = 0;
    for ch in ln.chars() {
        let bit = match ch {
            'B' => 1,
            'R' => 1,
            'F' => 0,
            'L' => 1,
            _ => panic!()
        };
        num = num | bit;
        num = num << 1;
    }
    return SeatCoord{
        row: (num >> 3) as u8,
        col: (num & 0x07) as u8,
    }
}

fn main() {
    let lines = slurp_input();
}
