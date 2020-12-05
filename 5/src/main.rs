use std::fs;

fn slurp_input() -> Vec<String> {
    let filename = "input";
    let contents = fs::read_to_string(filename).unwrap().to_string();
    let lines = contents.lines().map(|ln| ln.to_string()).collect();
    return lines;
}

struct SeatCoord {
    src: String,
    row: u8,
    col: u8,
}

impl SeatCoord {
    fn seat_id(&self) -> u32 {
        (self.row as u32) * 8 + (self.col as u32)
    }
}

fn seat_id(row: u8, col: u8) -> u32 {
    (row as u32) * 8 + (col as u32)
}

fn decode_seat(ln: &str) -> SeatCoord {
    let mut num: u16 = 0;
    for ch in ln.chars() {
        num = num << 1;
        let bit: u16 = match ch {
            'B' => 1,
            'R' => 1,
            'F' => 0,
            'L' => 0,
            _ => panic!()
        };
        num = num | bit;
    }
    return SeatCoord{
        src: ln.to_string(),
        row: (num >> 3) as u8,
        col: (num & 0x0007) as u8,
    }
}

fn main() {
    let lines = slurp_input();
    let coords: Vec<SeatCoord> = lines.iter().map(|ln| decode_seat(ln)).collect();

    for coord in coords.iter() {
        println!("{}: {} * 8 + {} = {}", coord.src, coord.row, coord.col, coord.seat_id());
    }

    let min_row = coords.iter().map(|coord| coord.row).min().unwrap();
    let max_row = coords.iter().map(|coord| coord.row).max().unwrap();

    let mut seat_ids: Vec<u32> = coords.iter()
        .map(|coord| coord.seat_id())
        .collect();

    let max_seat_id = seat_ids.iter().max().unwrap();

    println!("max: {}", max_seat_id);

    let min_valid_seat_id = seat_id(min_row, 0);
    let max_valid_seat_id = seat_id(max_row, 7);
    seat_ids.sort_unstable();
    let valid_seat_ids: Vec<u32> = seat_ids.into_iter()
        .filter(|x| x >= &min_valid_seat_id && x <= &max_valid_seat_id)
        .collect();
    let seat_id_deltas: Vec<(&u32, &u32)> =
        valid_seat_ids.iter()
        .zip(valid_seat_ids.iter().skip(1))
        .filter(|(&x, &y)| (y - x) == 2)
        .collect();

    println!("rows: {} - {}", min_row, max_row);
    seat_id_deltas.iter()
        .for_each(|(&x, &y)| println!("{} ***{}*** {}", x, x + 1, y));

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_example_correctly() {
        let example = "FBFBBFFRLR";
        let coord = decode_seat(example);
        assert_eq!(coord.src, example);
        assert_eq!(coord.row, 44);
        assert_eq!(coord.col, 5);
        assert_eq!(coord.seat_id(), 357);
    }
}
