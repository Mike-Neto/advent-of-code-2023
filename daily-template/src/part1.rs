pub fn process(_input: &str) -> anyhow::Result<String> {
    todo!("day {{project-name}} - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let input = include_str!("../example.txt");
        assert_eq!("", process(input)?);
        Ok(())
    }
}
