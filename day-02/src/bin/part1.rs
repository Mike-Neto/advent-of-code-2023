use day_02::part1::process;

fn main() {
    let file = include_str!("../../data.txt");
    let result = process(file);
    println!("The sum of the IDs of those games is: {result}");
    assert_eq!(result, 2207);
}
