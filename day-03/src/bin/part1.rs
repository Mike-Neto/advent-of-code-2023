use day_03::part1::process;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../data.txt");
    let result = process(file)?;
    println!("The sum of all of the part numbers in the engine schematic is: {result}");
    assert!(result.parse::<usize>().unwrap_or_default() > 515938);
    assert!(result.parse::<usize>().unwrap_or_default() > 529673);
    assert_eq!(result.parse::<usize>().unwrap_or_default(), 530495);
    Ok(())
}
