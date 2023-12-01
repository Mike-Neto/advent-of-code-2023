use day_01::part1::process;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../data.txt");
    let result = process(file)?;
    println!("The sum all the calibration values is: {result}");
    assert_eq!(result, 57346);
    Ok(())
}
