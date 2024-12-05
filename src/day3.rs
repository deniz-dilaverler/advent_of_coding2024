use core::panic;
use regex::Regex;
use std::fs::read_to_string;

pub fn read_str(path: &str) -> String {
    let file_str = read_to_string(path).unwrap();
    return file_str;
}

pub fn solve_problem3_1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((?P<item1>[0-9]+),(?P<item2>[0-9]+)\)").unwrap();
    let mut sum = 0;

    let muls: Vec<(i32, i32)> = re
        .captures_iter(input)
        .map(|caps| {
            return (
                caps.name("item1").unwrap().as_str().parse::<i32>().unwrap(),
                caps.name("item2").unwrap().as_str().parse::<i32>().unwrap(),
            );
        })
        .collect();

    for mul_tuple in muls.iter() {
        sum += mul_tuple.0 * mul_tuple.1;
    }
    return sum;
}

pub fn solve_problem3_2(input: &str) -> i32 {
    let mut sum = 0;
    let pattern = r"do\(\)|don't\(\)|mul\((?P<item1>[0-9]+),(?P<item2>[0-9]+)\)";
    let re = Regex::new(pattern).unwrap();

    // Iterate over matches and collect their indexes
    let mut active = true;
    for mat in re.captures_iter(input) {
        if let Some(m) = mat.name("item1") {
            if !active {
                continue;
            }
            let val1 = m.as_str().parse::<i32>().unwrap();
            let val2 = mat.name("item2").unwrap().as_str().parse::<i32>().unwrap();
            sum += val1 * val2;
        } else if mat.get(0).unwrap().as_str() == "do()" {
            active = true;
        } else if mat.get(0).unwrap().as_str() == "don't()" {
            active = false;
        } else {
            println!("Invalid '{}'", mat.get(0).unwrap().as_str());
        }
    }
    return sum;
}
