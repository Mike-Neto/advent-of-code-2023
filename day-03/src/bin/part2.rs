use day_03::part2::process;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../data.txt");
    let result = process(file)?;
    println!(
        "The sum of all gear ratios in the engine schematic is: {}",
        result
    );
    assert_eq!(result.parse::<usize>().unwrap_or_default(), 80253814);
    Ok(())
}
