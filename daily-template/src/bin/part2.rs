use {{crate_name}}::part2::process;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../data.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}
