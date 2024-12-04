mod day1;
mod day2;

fn main() {
    let inputs = day2::read_input("./input/day2.txt");
    let result = day2::solve_problem2_1(inputs);
    println!("{}", result);
}
