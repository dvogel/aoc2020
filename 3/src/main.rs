use std::fs;

fn slurp_input() -> Vec<String> {
    let filename = "input";
    let contents = fs::read_to_string(filename).unwrap().to_string();
    let lines = contents.lines().map(|x| x.to_string()).collect();
    return lines;
}

fn count_trees_on_slope(lines: &Vec<String>, dx: usize, dy: usize) -> u32 {

    let row_count = lines.len();
    let col_count = lines.first().unwrap().len();

    let mut tree_count = 0;
    let mut x = 0;
    let mut y = 0;
    while y < row_count {
        let row = &lines[y];
        let ch = row.chars().nth(x).unwrap();
        if ch == '#' {
            tree_count = tree_count + 1;
        }

        x = (x + dx) % col_count;
        y = y + dy;
    }

    return tree_count;
}

fn main() {
    let lines = slurp_input();

    let slopes: [(usize, usize); 5] = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)
    ];
    let mut accum: u32 = 1;
    for (dx, dy) in slopes.iter() {
        let tree_count = count_trees_on_slope(&lines, *dx, *dy);
        println!("trees: {} for {} x {}", tree_count, dx, dy);
        accum = accum * tree_count;
    }

    println!("tree accum: {}", accum);
}
