// Path: src/bin/day2.rs
fn is_safe(numbers: &[isize]) -> bool {
    let mut numbers = numbers.windows(2);

    let first_pair = numbers.next().unwrap();

    if !(1..=3).contains(&(first_pair[0] - first_pair[1]).abs()) {
        return false;
    }

    let is_increasing = first_pair[0] < first_pair[1];

    numbers.all(|pair| {
        let diff = (pair[0] - pair[1]).abs();
        (1..=3).contains(&diff) && (is_increasing == (pair[0] < pair[1]))
    })
}

fn to_level(line: &str) -> Vec<isize> {
    line.split_whitespace()
        .flat_map(str::parse::<isize>)
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(to_level)
        .filter(|numbers| is_safe(numbers))
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(to_level)
        .filter(|numbers| {
            if is_safe(numbers) {
                return true;
            }

            (0..numbers.len()).any(|i| {
                let mut numbers = numbers.clone();
                numbers.remove(i);
                is_safe(&numbers)
            })
        })
        .count()
}

fn main() {
    let input = std::fs::read_to_string("inputs/day2.txt").expect("Failed to read file");
    println!("Part 1 = {:?}", part1(&input));
    println!("Part 2 = {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 4);
    }
}
