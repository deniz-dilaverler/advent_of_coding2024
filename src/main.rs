mod solution1;

fn main() {
    let (l1, l2) = solution1::read_question1_input("./input1.txt");
    let result = solution1::solve_question1_2(l1, l2);
    println!("{}", result);
}
