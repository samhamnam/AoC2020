use crate::lib::load_input;

#[allow(dead_code)]
pub fn run() {
    // Am not too proud over this one to be honest.

    println!("-------- Day 7 --------");

    let bag = vec!["shiny gold".to_owned()];

    let strings: Vec<String> = load_input("input/day7")
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    let rules = create_rules(strings);

    let part1 = part1(bag.clone(), rules.clone());
    println!(
        "Part 1: Amount of bags that hold \"shiny gold\": {:?}",
        part1.len() - 1 // Remove one due to the result including "shiny gold";
    );

    let part2 = part2(bag[0].clone(), rules);
    println!(
        "Part 2: Amount of bags that \"shiny gold\" hold: {:?}",
        part2
    )
}

fn create_rules(strings: Vec<String>) -> Vec<(String, Vec<(i32, String)>)> {
    let mut rules = Vec::new();
    for s in strings {
        let s: Vec<&str> = s.split(" ").collect();
        let name = format!("{} {}", s[0], s[1]);
        let mut content = Vec::new();
        for i in (4..s.len()).step_by(4) {
            let name = format!("{} {}", s[i + 1], s[i + 2]);
            let amount = match s[i].parse::<i32>() {
                Err(_) => 0,
                _ => s[i].parse::<i32>().unwrap(),
            };
            if name != "other bags." {
                content.push((amount, format!("{} {}", s[i + 1], s[i + 2])))
            }
        }

        rules.push((name, content));
    }
    rules
}

fn part1(bags: Vec<String>, rules: Vec<(String, Vec<(i32, String)>)>) -> Vec<String> {
    fn reverse_refers(bags: Vec<String>, rules: Vec<(String, Vec<(i32, String)>)>) -> Vec<String> {
        let mut bags = bags.clone();
        let mut directs = Vec::new();
        for (name, content) in rules {
            for (_, c) in content {
                if bags.contains(&c) {
                    directs.push(name.clone());
                }
            }
        }
        bags.append(&mut directs);
        bags
    }

    let mut result = reverse_refers(bags.clone(), rules.clone());

    let mut last_size = 0;
    loop {
        let mut new_result = reverse_refers(result.clone(), rules.clone());

        new_result.sort();
        new_result.dedup();
        result = new_result;

        if result.len() == last_size {
            break;
        }

        last_size = result.len();
    }
    result
}

fn part2(bag: String, rules: Vec<(String, Vec<(i32, String)>)>) -> usize {
    fn recurse(bags: Vec<String>, rules: Vec<(String, Vec<(i32, String)>)>) -> Vec<String> {
        let mut result = Vec::new();
        for bag in bags {
            for (name, v) in rules.clone() {
                if bag == name {
                    for (a, n) in v {
                        for _ in 0..a {
                            result.push(n.clone());
                        }
                    }
                }
            }
        }

        result
    }

    let mut ans = vec![bag.clone()];
    let mut amount = 0;
    loop {
        let new_ans = recurse(ans.clone(), rules.clone());
        amount += new_ans.len();
        if new_ans.len() != 0 {
            ans = new_ans;
        } else {
            break;
        }
    }
    amount
}
