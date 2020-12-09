use crate::lib::load_input;

pub fn run() {
    println!("-------- Day 9 --------");
    let input: Vec<i64> = load_input("input/day9")
        .split("\n")
        .map(|s| match s.parse::<i64>() {
            Err(_) => 0,
            _ => s.parse::<i64>().unwrap(),
        })
        .filter(|i| i != &0)
        .collect();

    let invalid_num = find_non_preamble_nums(input.clone(), 25)[0];
    println!("PART 1: First invalid num: {}", invalid_num);

    let mut continous_sum = find_continous(input, invalid_num);
    continous_sum.sort();
    println!(
        "PART 2: Encryption weakness: {}\n",
        continous_sum[0] + continous_sum[continous_sum.len() - 1]
    );
}

fn find_non_preamble_nums(numbers: Vec<i64>, pre_amble: usize) -> Vec<i64> {
    let mut return_vec = Vec::new();
    for i in pre_amble..numbers.len() {
        let sum = numbers[i];
        let mut found_ans = false;

        for x in (i - pre_amble)..(i) {
            for y in (i - pre_amble)..(i) {
                if (numbers[x] + numbers[y]) == sum {
                    found_ans = true;
                    break;
                }
            }
            if found_ans {
                break;
            }
        }

        if !found_ans {
            return_vec.push(sum);
        }
    }
    return_vec
}

fn find_continous(numbers: Vec<i64>, num: i64) -> Vec<i64> {
    for x in 0..numbers.len() {
        let mut found_ans = false;

        let mut ans = vec![numbers[x]];
        let mut sum = numbers[x];

        for y in (x + 1)..numbers.len() {
            ans.push(numbers[y]);
            sum += numbers[y];

            if sum == num {
                found_ans = true;
                break;
            }
        }

        if found_ans {
            return ans;
        }
    }
    vec![0]
}
