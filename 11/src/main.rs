use std::fs;

// part 1: 2386
// part 2: 

fn slurp_input(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).unwrap().to_string();
    let lines = contents.lines().map(|x| x.to_string()).collect();
    return lines;
}

fn parse_floor_table(lines: &Vec<String>) -> Vec<Vec<char>> {
    return lines.iter().map(|ln| ln.chars().collect()).collect();
}

fn update_seats(table: &Vec<Vec<char>>) -> (u32, Vec<Vec<char>>) {
    let adj_coords = &[
        (-1, -1), (0, -1), (1, -1),
        (-1, 0),           (1, 0),
        (-1, 1),  (0, 1),  (1, 1)
    ];
    let mut updates: u32 = 0;
    let mut cloned = table.to_vec();
    let table_height = table.len() as i32;
    let table_width = table[0].len() as i32;
    for (y, row) in table.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '.' {
                // no-op
            } else if *cell == '#' || *cell == 'L' {
                let mut adj_occupied = 0;
                for (off_x, off_y) in adj_coords.iter() {
                    let tmp_x = (x as i32) + off_x;
                    let tmp_y = (y as i32) + off_y;
                    if (tmp_x >= 0) && (tmp_y >= 0) && (tmp_y < table_height) && (tmp_x < table_width) {
                        let adj_cell = table[tmp_y as usize][tmp_x as usize];
                        if adj_cell == '#' {
                            adj_occupied = adj_occupied + 1;
                        }
                    }
                }
                // println!("({}, {}) => {} adj", x, y, adj_occupied);
                if *cell == '#' && adj_occupied >= 4 {
                    cloned[y][x] = 'L';
                    updates = updates + 1;
                } else if *cell == 'L' && adj_occupied == 0 {
                    cloned[y][x] = '#';
                    updates = updates + 1;
                }
            } else {
                println!("unmatched input: {}", *cell)
            }
        }
    }
    return (updates, cloned);
}

fn update_until_stable(input_floor: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut floor: Vec<Vec<char>> = input_floor.clone();
    loop {
        let (updates, updated_floor) = update_seats(&floor);
        if updates == 0 {
            return updated_floor;
        }
        floor = updated_floor;
    }
}

fn count_occupied_seats(table: &Vec<Vec<char>>) -> u32 {
    let mut cnt = 0;
    table.iter().for_each(|row| {
        row.iter().for_each(|cell| {
            if *cell == '#' {
                cnt = cnt + 1;
            }
        });
    });
    return cnt;
}

fn show_seats(table: &Vec<Vec<char>>) {
    for row in table {
        for cell in row {
            print!("{}", cell);
        }
        println!("");
    }
}

fn main() {
    let floor: Vec<Vec<char>> = parse_floor_table(&slurp_input("input"));
    let stable = update_until_stable(&floor);
    let cnt = count_occupied_seats(&stable);
    println!("Occupied seats in stable arrangement: {}", cnt);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut floor: Vec<Vec<char>> = parse_floor_table(&slurp_input("example"));
        let stable = update_until_stable(&floor);
        assert_eq!(count_occupied_seats(&stable), 37);
    }
}
