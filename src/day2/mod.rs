use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
struct Password {
    pub min: u32,
    pub max: u32,
    pub letter: String,
    pub pass: String,
}

#[allow(dead_code)]
pub fn run() {
    let passwords = {
        let mut file = File::open("src/day2/input.txt").expect("Failed to open input!");
        let mut file_string = String::new();
        file.read_to_string(&mut file_string)
            .expect("Failed to read");
        let rows: Vec<&str> = file_string.split("\n").collect();

        let mut passwords = Vec::new();

        for r in rows {
            let parts: Vec<&str> = r.split(" ").collect();

            let (min, max) = {
                let text: Vec<&str> = parts[0].split("-").collect();
                (
                    text[0].parse::<u32>().expect("Failed to parse"),
                    text[1].parse::<u32>().expect("Failed to parse"),
                )
            };
            let letter = parts[1].split(":").collect::<Vec<&str>>()[0].to_owned();
            let pass = parts[2].to_owned();

            passwords.push(Password {
                min,
                max,
                letter,
                pass,
            });
        }
        passwords
    };

    println!("-------- Day 2 --------");
    let mut total_amount = 0;
    for p in passwords.clone() {
        let split: Vec<&str> = p.pass.split("").collect();
        let mut amount = 0;
        for l in split {
            if l == p.letter {
                amount += 1;
            }
        }

        if amount >= p.min && amount <= p.max {
            total_amount += 1;
        }
    }
    println!("PART 1: Valid passwords: {}", total_amount);

    let mut total_amount = 0;
    for p in passwords {
        let split: Vec<&str> = p.pass.split("").collect();

        let min = (p.min) as usize;
        let max = (p.max) as usize;

        let l1 = split[min];
        let l2 = split[max];

        if l1 == p.letter && l2 != p.letter || l2 == p.letter && l1 != p.letter {
            total_amount += 1;
        }
    }
    println!("PART 2: Valid passwords: {}\n", total_amount);
}
