use std::fs::read_to_string;

pub fn read_question1_input(path: &str) -> (Vec<i32>, Vec<i32>) {
    let file_str = read_to_string(path).unwrap();
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in file_str.lines() {
        let values = line.split("   ").collect::<Vec<&str>>();

        if values.len() != 2 {
            panic!("invalid length of ");
        }

        list1.push(values[0].parse::<i32>().unwrap());
        list2.push(values[1].parse::<i32>().unwrap());
    }

    return (list1, list2);
}

pub fn solve_question1(mut l1: Vec<i32>, mut l2: Vec<i32>) -> i32 {
    let mut count = 0;
    l1.sort();
    l2.sort();
    for (i, val1) in l1.iter().enumerate() {
        count += (val1 - l2[i]).abs();
    }
    return count;
}
