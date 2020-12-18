use std::fs;
use std::cmp::Ord;
use std::collections::BTreeSet;

// Part 1: 319 initial active coords
// Part 2: 2324 initial active coords

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Coord {
    fn new_2d(x: i32, y: i32) -> Coord {
        Coord{x: x, y: y, z: 0, w: 0}
    }

    fn new_3d(x: i32, y: i32, z: i32) -> Coord {
        Coord{x: x, y: y, z: z, w: 0}
    }

    fn new_4d(x: i32, y: i32, z: i32, w: i32) -> Coord {
        Coord{x: x, y: y, z: z, w: w}
    }

    fn plus(self: &Coord, other: &Coord) -> Coord {
        Coord{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
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
                active_coords.insert(Coord::new_2d(x as i32, y as i32));
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
    let mut w_min = i32::MAX;
    let mut w_max = i32::MIN;

    if coords.len() == 0 {
        return (Coord::new_4d(0, 0, 0, 0), Coord::new_4d(0, 0, 0, 0));
    }

    for c in coords {
        x_min = std::cmp::min(x_min, c.x);
        y_min = std::cmp::min(y_min, c.y);
        z_min = std::cmp::min(z_min, c.z);
        w_min = std::cmp::min(w_min, c.w);

        x_max = std::cmp::max(x_max, c.x);
        y_max = std::cmp::max(y_max, c.y);
        z_max = std::cmp::max(z_max, c.z);
        w_max = std::cmp::max(w_max, c.w);
    }

    return (
        Coord::new_4d(x_min, y_min, z_min, w_min),
        Coord::new_4d(x_max, y_max, z_max, w_max)
    );
}

struct NeighborCoords {
    x_off_min: i32,
    x_off_max: i32,
    y_off_min: i32,
    y_off_max: i32,
    z_off_min: i32,
    z_off_max: i32,
    w_off_min: i32,
    w_off_max: i32,
    x_curr: i32,
    y_curr: i32,
    z_curr: i32,
    w_curr: i32,
}

impl NeighborCoords {
    fn new_abs_3d(abs_range: i32) -> NeighborCoords {
        let neg = -abs_range.abs();
        let pos = abs_range.abs();
        NeighborCoords{
            x_off_min: neg,
            x_off_max: pos,
            y_off_min: neg,
            y_off_max: pos,
            z_off_min: neg,
            z_off_max: pos,
            w_off_min: 0,
            w_off_max: 0,
            x_curr: neg,
            y_curr: neg,
            z_curr: neg,
            w_curr: 0,
        }
    }

    fn new_abs_4d(abs_range: i32) -> NeighborCoords {
        let neg = -abs_range.abs();
        let pos = abs_range.abs();
        NeighborCoords{
            x_off_min: neg,
            x_off_max: pos,
            y_off_min: neg,
            y_off_max: pos,
            z_off_min: neg,
            z_off_max: pos,
            w_off_min: neg,
            w_off_max: pos,
            x_curr: neg,
            y_curr: neg,
            z_curr: neg,
            w_curr: neg,
        }
    }
}

impl Iterator for NeighborCoords {
    type Item = Coord;

    fn next(&mut self) -> Option<Coord> {
        if (self.x_curr > self.x_off_max) && (self.y_curr > self.y_off_max) && (self.z_curr > self.z_off_max) && (self.w_curr > self.w_off_max) {
            return None;
        }

        if (self.x_curr == 0) && (self.y_curr == 0) && (self.z_curr == 0) && (self.w_curr == 0) {
            self.x_curr += 1;
        }

        let result_coord: Coord = Coord::new_4d(self.x_curr, self.y_curr, self.z_curr, self.w_curr);

        if self.x_curr == self.x_off_max {
            if self.y_curr == self.y_off_max {
                if self.z_curr == self.z_off_max {
                    if self.w_curr == self.w_off_max {
                        // Invalidate so we'll return None on the subsequent call.
                        self.z_curr += 1;
                        self.x_curr += 1;
                        self.y_curr += 1;
                        self.w_curr += 1;
                    } else {
                        self.w_curr += 1;
                        self.z_curr = self.z_off_min;
                        self.y_curr = self.y_off_min;
                        self.x_curr = self.x_off_min;
                    }
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

    for w in min_coord.w-1..(max_coord.w + 2) {
        for z in min_coord.z-1..(max_coord.z + 2) {
            for y in min_coord.y-1..(max_coord.y + 2) {
                for x in min_coord.x-1..(max_coord.x + 2) {
                    let curr_coord = Coord::new_4d(x, y, z, w);
                    let active_neighbor_count: u32 = NeighborCoords::new_abs_4d(1).map(|nc| {
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
    }

    return new_coords;
}

fn main() {
    let mut active_coords = read_plane("input");
    for _ in 0..6 {
        active_coords = step_active_coords(&active_coords);
    }
    // See previous commit for part 1.
    println!("Part 2: {} initial active coords", active_coords.len());
}

mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut active_coords = read_plane("example1");
        assert_eq!(active_coords.contains(&Coord::new_3d(0, 0, 0)), false);
        assert_eq!(active_coords.contains(&Coord::new_3d(1, 0, 0)), true);
        assert_eq!(active_coords.contains(&Coord::new_3d(2, 0, 0)), false);

        assert_eq!(active_coords.contains(&Coord::new_3d(0, 1, 0)), false);
        assert_eq!(active_coords.contains(&Coord::new_3d(1, 1, 0)), false);
        assert_eq!(active_coords.contains(&Coord::new_3d(2, 1, 0)), true);

        assert_eq!(active_coords.contains(&Coord::new_3d(0, 2, 0)), true);
        assert_eq!(active_coords.contains(&Coord::new_3d(1, 2, 0)), true);
        assert_eq!(active_coords.contains(&Coord::new_3d(2, 2, 0)), true);

        for _ in 0..6 {
            active_coords = step_active_coords(&active_coords);
        }
        assert_eq!(active_coords.len(), 848);
    }

    #[test]
    fn test_coord_iter() {
        let seq: Vec<Coord> = NeighborCoords::new_abs_3d(1).collect();
        assert_eq!(seq.len(), 26);
    }

    #[test]
    fn test_bounding_cube() {
        let mut coords: BTreeSet<Coord> = BTreeSet::new();
        coords.insert(Coord::new_3d(-1, -1, -1));
        coords.insert(Coord::new_3d(1, 1, 1));
        let bounds1 = bounding_cube(&coords);
        assert_eq!(bounds1.0, Coord::new_3d(-1, -1, -1));
        assert_eq!(bounds1.1, Coord::new_3d(1, 1, 1));
        coords.insert(Coord::new_3d(-2, -2, -2));
        coords.insert(Coord::new_3d(2, 2, 2));
        let bounds2 = bounding_cube(&coords);
        assert_eq!(bounds2.0, Coord::new_3d(-2, -2, -2));
        assert_eq!(bounds2.1, Coord::new_3d(2, 2, 2));
    }
}
