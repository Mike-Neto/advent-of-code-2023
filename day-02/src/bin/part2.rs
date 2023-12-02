use day_02::part2::process;

fn main() {
    let file = include_str!("../../data.txt");
    let result = process(file);
    println!("The sum of the power of these sets is: {result}");
    assert_eq!(result, 62241);
}
