use crate::lib::load_input;

#[derive(Clone)]
struct Password {
    pub min: u32,
    pub max: u32,
    pub letter: String,
    pub pass: String,
}

#[allow(dead_code)]
pub fn run() {
    println!("-------- Day 2 --------");

    let passwords = {
        let file_string = load_input("src/day2/input.txt");
        let rows: Vec<&str> = file_string.split("\n").collect();

        // Creates an empty Password Vector
        let mut passwords = Vec::new();

        for r in rows {
            let parts: Vec<&str> = r.split(" ").collect();

            // Splits the row gathering all the data about the password.
            let (min, max) = {
                let text: Vec<&str> = parts[0].split("-").collect();
                (
                    text[0].parse::<u32>().expect("Failed to parse"),
                    text[1].parse::<u32>().expect("Failed to parse"),
                )
            };
            let letter = parts[1].split(":").collect::<Vec<&str>>()[0].to_owned();
            let pass = parts[2].to_owned();

            // Push a Password instance containing the password data to the Vector created above.
            passwords.push(Password {
                min,
                max,
                letter,
                pass,
            });
        }
        passwords
    };

    let mut valid_amount = 0;
    for p in passwords.clone() {
        let split: Vec<&str> = p.pass.split("").collect();
        let mut amount = 0;
        for l in split {
            if l == p.letter {
                amount += 1;
            }
        }

        if amount >= p.min && amount <= p.max {
            valid_amount += 1;
        }
    }
    println!("PART 1: Valid passwords: {}", valid_amount);

    let mut valid_amount_2 = 0;
    for p in passwords {
        let split: Vec<&str> = p.pass.split("").collect();

        let min = (p.min) as usize;
        let max = (p.max) as usize;

        let l1 = split[min];
        let l2 = split[max];

        if l1 == p.letter && l2 != p.letter || l2 == p.letter && l1 != p.letter {
            valid_amount_2 += 1;
        }
    }
    println!("PART 2: Valid passwords: {}\n", valid_amount_2);
}
