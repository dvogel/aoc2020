use std::fs;
use std::cmp::Ord;
use std::collections::BTreeSet;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord {
    fn new(x: i32, y: i32, z: i32) -> Coord {
        Coord{x: x, y: y, z: z}
    }

    fn plus(self: &Coord, other: &Coord) -> Coord {
        Coord{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

fn read_plane(filename: &str) -> BTreeSet<Coord> {
    let mut active_coords: BTreeSet<Coord> = BTreeSet::new();
    let contents = fs::read_to_string(filename).unwrap();
    let lines = contents.lines();
    for (y, row) in lines.enumerate() {
        for (x, state_ch) in row.chars().enumerate() {
            if state_ch == '#' {
                active_coords.insert(Coord::new(x as i32, y as i32, 0));
            }
        }
    }

    return active_coords;
}

fn bounding_cube(coords: &BTreeSet<Coord>) -> (Coord, Coord) {
    let mut x_min = i32::MAX;
    let mut x_max = i32::MIN;
    let mut y_min = i32::MAX;
    let mut y_max = i32::MIN;
    let mut z_min = i32::MAX;
    let mut z_max = i32::MIN;

    if coords.len() == 0 {
        return (Coord::new(0, 0, 0), Coord::new(0, 0, 0));
    }

    for c in coords {
        x_min = std::cmp::min(x_min, c.x);
        y_min = std::cmp::min(y_min, c.y);
        z_min = std::cmp::min(z_min, c.z);
        x_max = std::cmp::max(x_max, c.x);
        y_max = std::cmp::max(y_max, c.y);
        z_max = std::cmp::max(z_max, c.z);
    }

    return (
        Coord::new(x_min, y_min, z_min),
        Coord::new(x_max, y_max, z_max)
    );
}

struct NeighborCoords {
    x_off_min: i32,
    x_off_max: i32,
    y_off_min: i32,
    y_off_max: i32,
    z_off_min: i32,
    z_off_max: i32,
    x_curr: i32,
    y_curr: i32,
    z_curr: i32,
}

impl NeighborCoords {
    fn new_abs(abs_range: i32) -> NeighborCoords {
        let neg = -abs_range.abs();
        let pos = abs_range.abs();
        NeighborCoords{
            x_off_min: neg,
            x_off_max: pos,
            y_off_min: neg,
            y_off_max: pos,
            z_off_min: neg,
            z_off_max: pos,
            x_curr: neg,
            y_curr: neg,
            z_curr: neg,
        }
    }
}

impl Iterator for NeighborCoords {
    type Item = Coord;

    fn next(&mut self) -> Option<Coord> {
        if (self.x_curr > self.x_off_max) && (self.y_curr > self.y_off_max) && (self.z_curr > self.z_off_max) {
            return None;
        }

        if (self.x_curr == 0) && (self.y_curr == 0) && (self.z_curr == 0) {
            self.x_curr += 1;
        }

        let result_coord: Coord = Coord::new(self.x_curr, self.y_curr, self.z_curr);

        if self.x_curr == self.x_off_max {
            if self.y_curr == self.y_off_max {
                if self.z_curr == self.z_off_max {
                    // Invalidate so we'll return None on the subsequent call.
                    self.z_curr += 1;
                    self.x_curr += 1;
                    self.y_curr += 1;
                } else {
                    self.z_curr += 1;
                    self.y_curr = self.y_off_min;
                    self.x_curr = self.x_off_min;
                }
            } else {
                self.y_curr += 1;
                self.x_curr = self.x_off_min;
            }
        } else {
            self.x_curr += 1;
        }
 
        return Some(result_coord);
    }
}

fn step_active_coords(active_coords: &BTreeSet<Coord>) -> BTreeSet<Coord> {
    let mut new_coords: BTreeSet<Coord> = BTreeSet::new();
    let (min_coord, max_coord) = bounding_cube(&active_coords);

    for z in min_coord.z-1..(max_coord.z + 2) {
        for y in min_coord.y-1..(max_coord.y + 2) {
            for x in min_coord.x-1..(max_coord.x + 2) {
                let curr_coord = Coord::new(x, y, z);
                let active_neighbor_count: u32 = NeighborCoords::new_abs(1).map(|nc| {
                    match active_coords.contains(&nc.plus(&curr_coord)) {
                        true => 1,
                        false => 0,
                    }
                }).sum();

                let curr_is_active = active_coords.contains(&curr_coord);
                if curr_is_active && ((active_neighbor_count == 2) || (active_neighbor_count == 3)) {
                    new_coords.insert(curr_coord);
                } else if !curr_is_active && (active_neighbor_count == 3) {
                    new_coords.insert(curr_coord);
                }
            }
        }
    }

    return new_coords;
}

fn main() {
    let mut active_coords = read_plane("input");
    for _ in 0..6 {
        // let work_coords = active_coords.clone();
        active_coords = step_active_coords(&active_coords);
    }
    println!("Part 1: {} initial active coords", active_coords.len());
}

mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut active_coords = read_plane("example1");
        assert_eq!(active_coords.contains(&Coord::new(0, 0, 0)), false);
        assert_eq!(active_coords.contains(&Coord::new(1, 0, 0)), true);
        assert_eq!(active_coords.contains(&Coord::new(2, 0, 0)), false);

        assert_eq!(active_coords.contains(&Coord::new(0, 1, 0)), false);
        assert_eq!(active_coords.contains(&Coord::new(1, 1, 0)), false);
        assert_eq!(active_coords.contains(&Coord::new(2, 1, 0)), true);

        assert_eq!(active_coords.contains(&Coord::new(0, 2, 0)), true);
        assert_eq!(active_coords.contains(&Coord::new(1, 2, 0)), true);
        assert_eq!(active_coords.contains(&Coord::new(2, 2, 0)), true);

        for _ in 0..6 {
            active_coords = step_active_coords(&active_coords);
        }
        assert_eq!(active_coords.len(), 112);
    }

    #[test]
    fn test_coord_iter() {
        let seq: Vec<Coord> = NeighborCoords::new_abs(1).collect();
        assert_eq!(seq.len(), 26);
    }

    #[test]
    fn test_bounding_cube() {
        let mut coords: BTreeSet<Coord> = BTreeSet::new();
        coords.insert(Coord::new(-1, -1, -1));
        coords.insert(Coord::new(1, 1, 1));
        let bounds1 = bounding_cube(&coords);
        assert_eq!(bounds1.0, Coord::new(-1, -1, -1));
        assert_eq!(bounds1.1, Coord::new(1, 1, 1));
        coords.insert(Coord::new(-2, -2, -2));
        coords.insert(Coord::new(2, 2, 2));
        let bounds2 = bounding_cube(&coords);
        assert_eq!(bounds2.0, Coord::new(-2, -2, -2));
        assert_eq!(bounds2.1, Coord::new(2, 2, 2));
    }
}
