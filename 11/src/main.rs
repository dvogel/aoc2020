use std::fs;

// part 1: 2386
// part 2: 2091

fn slurp_input(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).unwrap().to_string();
    let lines = contents.lines().map(|x| x.to_string()).collect();
    return lines;
}

fn parse_floor_table(lines: &Vec<String>) -> Vec<Vec<char>> {
    return lines.iter().map(|ln| ln.chars().collect()).collect();
}

fn update_seats(table: &Vec<Vec<char>>, adj_affinity: u32, horizon: u32) -> (u32, Vec<Vec<char>>) {
    let adj_coords = &[
        (-1, -1), (0, -1), (1, -1),
        (-1, 0),           (1, 0),
        (-1, 1),  (0, 1),  (1, 1)
    ];
    let mut updates: u32 = 0;
    let mut cloned = table.clone();
    let table_height = table.len() as i32;
    let table_width = table[0].len() as i32;
    for (y, row) in table.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '.' {
                // no-op
            } else if *cell == '#' || *cell == 'L' {
                let mut adj_occupied = 0;
                for (off_x, off_y) in adj_coords.iter() {
                    let mut tmp_x = x as i32;
                    let mut tmp_y = y as i32;
                    let mut steps = 0;
                    loop {
                        steps = steps + 1;
                        if horizon > 0 && steps > horizon {
                            break;
                        }

                        tmp_x = tmp_x + off_x;
                        tmp_y = tmp_y + off_y;
                        if (tmp_x < 0) || (tmp_y < 0) || (tmp_x >= table_width) || (tmp_y >= table_height) {
                            break;
                        }

                        let adj_cell = table[tmp_y as usize][tmp_x as usize];
                        if adj_cell == '#' {
                            adj_occupied = adj_occupied + 1;
                            break;
                        } else if adj_cell == 'L' {
                            break;
                        }

                    }
                }
                // println!("({}, {}) => {} adj", x, y, adj_occupied);
                if *cell == '#' && adj_occupied >= adj_affinity {
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

fn update_until_stable(input_floor: &Vec<Vec<char>>, adj_affinity: u32, horizon: u32) -> Vec<Vec<char>> {
    let mut floor: Vec<Vec<char>> = input_floor.clone();
    loop {
        let (updates, updated_floor) = update_seats(&floor, adj_affinity, horizon);
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
    let stable1 = update_until_stable(&floor, 4, 1);
    let cnt1 = count_occupied_seats(&stable1);
    println!("Occupied seats in stable arrangement, part 1: {}", cnt1);

    let stable2 = update_until_stable(&floor, 5, 0);
    let cnt2 = count_occupied_seats(&stable2);
    println!("Occupied seats in stable arrangement, part 2: {}", cnt2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut floor: Vec<Vec<char>> = parse_floor_table(&slurp_input("example"));
        let stable = update_until_stable(&floor, 4, 1);
        assert_eq!(count_occupied_seats(&stable), 37);
    }

    #[test]
    fn test_example2() {
        let mut floor: Vec<Vec<char>> = parse_floor_table(&slurp_input("example"));
        let stable = update_until_stable(&floor, 5, 0);
        assert_eq!(count_occupied_seats(&stable), 26);
    }
}
