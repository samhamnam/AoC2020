use crate::lib::load_input;

pub fn run() {
    println!("-------- Day 6 --------");

    let groups = {
        let s = load_input("src/day6/input.txt");
        create_groups(s)
    };

    part_one(groups.clone());
    part_two(groups);
}

fn part_two(groups: Vec<String>) {
    let groups: Vec<Vec<&str>> = groups.iter().map(|s| s.split("\n").collect()).collect();

    let mut count = 0;
    for g in groups.clone() {
        let mut checked_chars = String::new();
        for i in 0..g.len() {
            for c in g[i].chars() {
                if !checked_chars.contains(c) {
                    let mut vote = true;
                    for j in 0..g.len() {
                        if j == i {
                            continue;
                        }
                        vote = vote && g[j].contains(c);
                    }

                    if vote {
                        count += 1;
                        checked_chars.push(c);
                    }
                }
            }
        }
    }

    println!("Part 2: Vote count: {:?}\n", count);
}

fn part_one(groups: Vec<String>) {
    let groups: Vec<Vec<&str>> = groups.iter().map(|s| s.split("\n").collect()).collect();
    let groups: Vec<String> = groups
        .iter()
        .map(|v| v.iter().map(|s| s.to_owned()).collect())
        .collect();
    let clean: Vec<String> = groups.iter().map(|s| remove_dupes(s.clone())).collect();
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
    let groups: Vec<String> = groups.iter().map(|s| s.to_string()).collect();
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
