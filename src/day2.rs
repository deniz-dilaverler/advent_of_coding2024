use std::fs::read_to_string;

pub fn read_input(path: &str) -> Vec<Vec<i32>> {
    let file_str = read_to_string(path).unwrap();

    let mut inputs: Vec<Vec<i32>> = Vec::new();
    for line in file_str.lines() {
        let values = line.split(" ").collect::<Vec<&str>>();
        assert!(values.len() > 0);
        let mut int_line: Vec<i32> = Vec::new();
        for x in values {
            int_line.push(x.parse::<i32>().unwrap());
        }

        inputs.push(int_line);
    }

    return inputs;
}

fn solve_decreasing(line: Vec<i32>) -> bool {
    for i in 0..line.len() - 1 {
        let dif = line[i] - line[i + 1];

        if dif > 3 || dif <= 0 {
            return false;
        }
    }
    return true;
}

fn solve_increasing(line: Vec<i32>) -> bool {
    for i in 0..line.len() - 1 {
        let dif = line[i + 1] - line[i];

        if dif > 3 || dif <= 0 {
            return false;
        }
    }
    return true;
}
pub fn solve_problem2_1(input: Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for line in input {
        let mut is_increasing = true;
        if line[0] > line[1] {
            is_increasing = false;
        }

        if is_increasing {
            count += if solve_increasing(line) { 1 } else { 0 };
        } else {
            count += if solve_decreasing(line) { 1 } else { 0 };
        }
    }

    return count;
}
