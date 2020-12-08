use crate::lib::load_input;

type Rows = Vec<(String, i32)>;

#[allow(dead_code)]
pub fn run() {
    println!("-------- Day 8 --------");
    // Splits and parses the input into [...,(type, amount),...]
    let rows: Rows = load_input("input/day8")
        .split("\n")
        .map(|s| {
            let split: Vec<&str> = s.split(" ").collect();
            (
                split[0].to_owned(),
                split[1].parse::<i32>().expect("Failed to parse"),
            )
        })
        .collect();

    println!(
        "PART 1: Accumulator value before loop: {}",
        break_code_on_loop(rows.clone())
    );

    println!(
        "PART 2: Accumulator value after fixed loop: {:?}\n",
        execute_code_and_fix(rows)
    );
}

// Executes until a loop is hit.
fn break_code_on_loop(rows: Rows) -> i32 {
    let mut accumulator = 0;
    let mut i: i32 = 0;

    let mut touched_indexes = Vec::new();

    while (i as usize) < rows.len() && !touched_indexes.contains(&i) {
        touched_indexes.push(i);

        let (ins, amount) = rows[i as usize].clone();

        if ins == "nop" {
            i += 1;
        } else if ins == "acc" {
            accumulator += amount;
            i += 1;
        } else if ins == "jmp" {
            i += amount;
        }
    }

    accumulator
}

// Finds the faulty line using bruteforce and fixes it.
fn execute_code_and_fix(rows: Rows) -> i32 {
    let jump_indexes = jump_indexes(rows.clone());
    let mut acc = 0;

    for index in jump_indexes {
        let mut rows = rows.clone();
        let mut touched_indexes = Vec::new();

        rows[index].0 = "nop".to_string();

        let mut accumulator = 0;
        let mut i: i32 = 0;

        let mut correct = true;

        while (i as usize) < rows.len() {
            if touched_indexes.contains(&i) {
                correct = false;
                break;
            }

            touched_indexes.push(i);

            let (ins, amount) = rows[i as usize].clone();

            if ins == "nop" {
                i += 1;
            } else if ins == "acc" {
                accumulator += amount;
                i += 1;
            } else if ins == "jmp" {
                i += amount;
            }
        }
        if correct {
            acc = accumulator;
        }
    }

    acc
}

// Gives all the indexes for all "jmp" instances
fn jump_indexes(rows: Rows) -> Vec<usize> {
    let mut indexes = Vec::new();
    for (i, (t, _)) in rows.iter().enumerate() {
        if t == "jmp" {
            indexes.push(i);
        }
    }
    indexes
}
