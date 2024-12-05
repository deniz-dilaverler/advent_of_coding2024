mod day1;
mod day2;
mod day3;

fn main() {
    let input = day3::read_str("./input/day3.txt");
    let result = day3::solve_problem3_2(&input);
    println!("{}", result);
}
