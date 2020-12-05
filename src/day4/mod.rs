use std::fs::File;
use std::io::prelude::*;

// Splits a length into (length, unit), "421cm" -> (421, "cm") etc.
fn split_length(length: &str) -> Option<(i32, String)> {
    let s: Vec<char> = length.chars().collect();

    let mut t: String = "".to_owned();
    if s[s.len() - 1] == 'm' {
        t = "cm".to_owned();
    } else if s[s.len() - 1] == 'n' {
        t = "in".to_owned()
    };

    let numbers = "1234567890".to_owned();
    let mut number_array = Vec::new();
    for c in s {
        if numbers.contains(c) {
            number_array.push(c)
        }
    }

    let n = number_array
        .iter()
        .collect::<String>()
        .parse::<i32>()
        .expect("Failed to parse!");

    if t == "" {
        None
    } else {
        Some((n, t))
    }
}

#[derive(Clone)]
struct Passport {
    byr: Option<i32>,
    iyr: Option<i32>,
    eyr: Option<i32>,
    hgt: Option<(i32, String)>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

impl Passport {
    pub fn valid(&self) -> bool {
        // Just checks if no member is equal to None, which would make it invalid
        self.byr != None
            && self.iyr != None
            && self.eyr != None
            && self.hgt != None
            && self.hcl != None
            && self.ecl != None
            && self.pid != None
    }

    pub fn valid_p2(&self) -> bool {
        if self.valid() {
            let byr = self.byr.unwrap();
            let iyr = self.iyr.unwrap();
            let eyr = self.eyr.unwrap();
            let (hgt_num, hgt_postfix) = self.hgt.clone().unwrap();
            let hcl = self.hcl.clone().unwrap();
            let ecl = self.ecl.clone().unwrap();
            let pid = self.pid.clone().unwrap();

            let mut ans = true;
            ans = ans && byr >= 1920 && byr <= 2002; // BYR
            ans = ans && iyr >= 2010 && iyr <= 2020; // IYR
            ans = ans && eyr >= 2020 && eyr <= 2030; // EYR

            // HGT
            if hgt_postfix == "in" {
                ans = ans && hgt_num >= 59 && hgt_num <= 76;
            } else if hgt_postfix == "cm" {
                ans = ans && hgt_num >= 150 && hgt_num <= 193;
            }

            // HCL
            ans = {
                let valid_chars = "abcdef1234567890";
                let chars: Vec<char> = hcl.chars().collect();
                let mut valid = true;
                if chars.len() == 7 {
                    for i in 0..chars.len() {
                        if i == 0 {
                            valid = chars[i] == '#';
                        } else {
                            valid = valid && valid_chars.contains(chars[i]);
                        }
                    }
                } else {
                    valid = false;
                }

                ans && valid
            };

            //ECL
            ans = {
                let t = std::str::from_utf8(&ecl.as_bytes()).unwrap();
                let valid = match t {
                    "amb" => true,
                    "blu" => true,
                    "brn" => true,
                    "gry" => true,
                    "grn" => true,
                    "hzl" => true,
                    "oth" => true,
                    _ => false,
                };

                ans && valid
            };

            //PID
            ans = {
                let mut valid = true;
                let numbers = "1234567890";
                if pid.len() == 9 {
                    let chars: Vec<char> = pid.chars().collect();
                    for c in chars {
                        valid = valid && numbers.contains(c);
                    }
                } else {
                    valid = false
                }

                ans && valid
            };

            ans
        } else {
            false
        }
    }
}

#[allow(dead_code)]
pub fn run() {
    println!("-------- Day 4 --------");

    let passports = {
        let mut file = File::open("src/day4/input.txt").expect("Failed to read input!");
        let mut file_string = String::new();
        file.read_to_string(&mut file_string)
            .expect("Failed to read string!");

        let mut split_data: Vec<String> =
            file_string.split("\n\n").map(|s| s.to_string()).collect();
        split_data = split_data.iter().map(|s| s.replace("\n", " ")).collect();

        let mut data: Vec<Vec<String>> = Vec::new();
        for s in split_data {
            let pass: Vec<String> = s.split(" ").map(|s| s.to_string()).collect();
            data.push(pass);
        }

        let mut passports: Vec<Passport> = Vec::new();
        for d in data {
            let mut byr: Option<i32> = None;
            let mut iyr: Option<i32> = None;
            let mut eyr: Option<i32> = None;
            let mut hgt: Option<(i32, String)> = None;
            let mut hcl: Option<String> = None;
            let mut ecl: Option<String> = None;
            let mut pid: Option<String> = None;

            for k in d {
                let split: Vec<&str> = k.split(":").collect();
                match split[0] {
                    "byr" => byr = Some(split[1].parse::<i32>().unwrap()),
                    "iyr" => iyr = Some(split[1].parse::<i32>().unwrap()),
                    "eyr" => eyr = Some(split[1].parse::<i32>().unwrap()),
                    "hgt" => hgt = split_length(split[1]),
                    "hcl" => hcl = Some(split[1].to_owned()),
                    "ecl" => ecl = Some(split[1].to_owned()),
                    "pid" => pid = Some(split[1].to_owned()),
                    _ => {}
                }
            }

            passports.push(Passport {
                byr,
                iyr,
                eyr,
                hgt,
                hcl,
                ecl,
                pid,
            });
        }
        passports
    };

    let mut valid = 0;
    for p in passports.clone() {
        if p.valid() {
            valid += 1;
        }
    }
    println!("PART 1: Valid passes: {}", valid);

    valid = 0;
    for p in passports {
        if p.valid_p2() {
            valid += 1;
        }
    }
    println!("PART 2: Valid passes: {}\n", valid);
}
