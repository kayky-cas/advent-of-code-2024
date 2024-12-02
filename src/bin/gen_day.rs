use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let day = std::env::args().nth(1).context("Please provide a day")?;

    let file = format!("src/bin/day{}.rs", day);

    let content = format!(
        r##"// Path: src/bin/day{}.rs
fn part1(_input: &str) {{
}}
    
fn part2(_input: &str) {{
}}

fn main() {{
    let input = std::fs::read_to_string("inputs/day{}.txt").expect("Failed to read file");
    println!("Part 1 = {{:?}}", part1(&input));
    println!("Part 2 = {{:?}}", part2(&input));
}}

#[cfg(test)]
mod tests {{
    use crate::*;

    const INPUT: &str = r#""#;

    #[test]
    fn test_part1() {{
        assert_eq!(part1(INPUT), ());
    }}

    #[test]
    fn test_part2() {{
        assert_eq!(part2(INPUT), ());
    }}
}}
        "##,
        day, day
    );

    std::fs::write(&file, content).context("Failed to write file")?;

    Ok(())
}
