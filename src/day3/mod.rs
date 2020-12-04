use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]
pub fn run() {
    // Loads the map
    println!("-------- Day 3 --------");

    let coords = {
        let mut file = File::open("src/day3/input.txt").expect("Failed to open input!");
        let mut file_string = String::new();
        file.read_to_string(&mut file_string)
            .expect("Failed to read");
        let rows_string: Vec<String> = file_string.split("\n").map(|s| s.to_string()).collect();

        let mut rows = Vec::new();
        for r in rows_string {
            rows.push(r.chars().collect());
        }
        rows
    };

    let trees = trees_hit(3, 1, &coords);
    println!("PART 1: Trees: {}", trees);

    let mut new_trees = trees_hit(1, 1, &coords);
    new_trees *= trees_hit(3, 1, &coords);
    new_trees *= trees_hit(5, 1, &coords);
    new_trees *= trees_hit(7, 1, &coords);
    new_trees *= trees_hit(1, 2, &coords);
    println!("PART 2: Trees: {}\n", new_trees);
}

fn trees_hit(x_offset: usize, y_offset: usize, map: &Vec<Vec<char>>) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    while y < map.len() {
        if map[y][x] == '#' {
            trees += 1;
        }
        x = (x + x_offset) % map[y].len();
        y += y_offset;
    }
    trees
}
