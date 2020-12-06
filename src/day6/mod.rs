use std::fs::File;
use std::io::prelude::*;

pub fn run() {
    let groups = {
        let mut f = File::open("src/day6/input.txt").expect("Failed to open input!");
        let mut s = String::new();
        f.read_to_string(&mut s).expect("Failed to load to string!");

        create_groups(s)
    };

    let clean: Vec<String> = groups.iter().map(|s| remove_dupes(s.to_string())).collect();
    let count = {
        let mut count = 0;
        for s in clean {
            count += s.len();
        }
        count
    };
    println!("Part 1: Vote count: {}", count);
}

fn create_groups(s: String) -> Vec<String> {
    let groups: Vec<&str> = s.split("\n\n").collect();
    let groups: Vec<Vec<&str>> = groups.iter().map(|s| s.split("\n").collect()).collect();
    let groups: Vec<String> = groups
        .iter()
        .map(|v| v.iter().map(|s| s.to_string()).collect())
        .collect();
    groups
}

fn remove_dupes(string: String) -> String {
    let chars: Vec<char> = string.chars().collect();
    let mut clean = String::new();

    for c in chars {
        if !clean.contains(c) {
            clean.push(c);
        }
    }
    clean
}
