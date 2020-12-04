use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]
pub fn run() {
    println!("-------- Day 1 --------");
    let input = {
        // Loads the input and reads it to the string content.
        let mut file = File::open("src/day1/input.txt").expect("Failed to open input!");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Failed to read!");

        // Splits the string at each newline and parses all numbers to i64.
        let strings: Vec<&str> = content.split("\n").collect();
        let mut numbers = Vec::new();
        for s in strings {
            numbers.push(s.parse::<i64>().expect("Failed to parse"))
        }
        numbers
    };

    // Travels through the list testing each number against every other number to see if
    // the sum is 2020.
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

    // Same as above but tests 3 numbers at once.
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
