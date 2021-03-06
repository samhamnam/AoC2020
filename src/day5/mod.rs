use crate::lib::load_input;

#[allow(dead_code)]
pub fn run() {
    println!("-------- Day 5 --------");
    let passes = {
        let string = load_input("input/day5");
        let split: Vec<String> = string.split("\n").map(|s| s.to_string()).collect();
        split
    };

    let mut ids: Vec<i32> = passes.iter().map(|p| find_seat_id(&p)).collect();

    ids.sort();

    println!("PART 1: Highest ID: {}", ids[ids.len() - 1]);

    for i in 1..ids.len() {
        let dif = ids[i] - ids[i - 1];
        if dif > 1 {
            println!("PART 2: Missing ID: {}\n", ids[i] - 1);
            break;
        }
    }
}

fn find_seat_id(seat: &str) -> i32 {
    let split: Vec<char> = seat.chars().collect();
    let mut rows: Vec<i32> = (0..127).collect();
    let mut collumns: Vec<i32> = (0..8).collect();

    for c in split {
        match c {
            'F' => rows = split_vec(rows, false),
            'B' => rows = split_vec(rows, true),
            'R' => collumns = split_vec(collumns, true),
            'L' => collumns = split_vec(collumns, false),
            _ => {}
        }
    }

    rows[0] * 8 + collumns[0]
}

fn split_vec<T: Clone + std::fmt::Debug>(vec: Vec<T>, right: bool) -> Vec<T> {
    let chunks = vec
        .chunks((vec.len() + 1) / 2)
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<T>>>();

    chunks[right as usize].clone()
}
