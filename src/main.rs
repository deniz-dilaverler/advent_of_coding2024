mod day1;

fn main() {
    let (l1, l2) = day1::read_question1_input("./input1.txt");
    let result = day1::solve_question1_2(l1, l2);
    println!("{}", result);
}
