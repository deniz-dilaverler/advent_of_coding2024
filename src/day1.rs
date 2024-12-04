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

pub fn solve_question1_2(mut l1: Vec<i32>, mut l2: Vec<i32>) -> i32 {
    let mut similarity_score = 0;
    l1.sort();
    l2.sort();

    let mut last = (-1, -1);
    let mut ptr2 = 0;
    for val1 in l1.iter() {
        if *val1 == last.0 {
            similarity_score += last.1;
            continue;
        }
        last.0 = *val1;

        let mut count = 0;
        while ptr2 < l2.len() && l2[ptr2] < *val1 {
            ptr2 += 1;
        }

        while ptr2 < l2.len() && l2[ptr2] == *val1 {
            count += 1;
            ptr2 += 1;
        }

        last.1 = count * val1;
        similarity_score += last.1;
    }

    return similarity_score;
}
