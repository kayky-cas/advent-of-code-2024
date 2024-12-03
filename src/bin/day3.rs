// Path: src/bin/day3.rs

use std::sync::OnceLock;

static R: OnceLock<regex::Regex> = OnceLock::new();

fn part1(input: &str) -> usize {
    R.get_or_init(|| regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap())
        .captures_iter(input)
        .filter_map(|cap| {
            let a = cap[1].parse::<usize>().ok();
            let b = cap[2].parse::<usize>().ok();
            Some(a? * b?)
        })
        .sum()
}

fn part2(mut input: &str) -> usize {
    const DONT: &str = "don't()";
    const DO: &str = "do()";

    let mut sum = 0;

    while !input.is_empty() {
        let j = input.find(DONT).unwrap_or(input.len());

        sum += part1(&input[..j]);

        if j == input.len() {
            break;
        }

        let padding = j + DONT.len();

        let Some(k) = &input[padding..].find(DO) else {
            break;
        };

        input = &input[k + DO.len() + padding..];
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("inputs/day3.txt").expect("Failed to read file");
    println!("Part 1 = {:?}", part1(&input));
    println!("Part 2 = {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str =
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 48);
    }
}
