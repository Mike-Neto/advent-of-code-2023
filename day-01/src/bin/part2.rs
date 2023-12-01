use day_01::part2::process;

fn main() {
    let file = include_str!("../../data.txt");
    let result = process(file);
    println!("The sum all the calibration values is: {result}");
    assert_eq!(result, 57345);
}
