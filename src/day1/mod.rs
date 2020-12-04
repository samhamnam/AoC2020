use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]
pub fn run() {
    println!("-------- Day 1 --------");
    let input = {
        let mut file = File::open("src/day1/input.txt").expect("Failed to open input!");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Failed to read!");

        let strings = content.split("\n").collect::<Vec<&str>>().clone();
        let mut numbers = Vec::new();
        for s in strings {
            numbers.push(s.parse::<i64>().expect("Failed to parse"))
        }
        numbers
    };

    let mut ans = 0;
    for i in 0..input.len() {
        for j in 0..input.len() {
            let val = input[i] + input[j];
            if val == 2020 {
                ans = input[i] * input[j];
            }
        }
    }
    println!("PART 1: Product: {}", ans);

    for i in 0..input.len() {
        for j in 0..input.len() {
            for k in 0..input.len() {
                let val = input[i] + input[j] + input[k];
                if val == 2020 {
                    ans = input[i] * input[j] * input[k];
                }
            }
        }
    }
    println!("PART 2: Product: {}\n", ans);
}
