// Path: src/bin/day1.rs

fn insert_sorted<T: Ord>(vec: &mut Vec<T>, value: T) {
    let index = match vec.binary_search(&value) {
        Ok(index) => index,
        Err(index) => index,
    };
    vec.insert(index, value);
}

fn part1(input: &str) -> isize {
    let mut first_column = Vec::with_capacity(input.len());
    let mut second_column = Vec::with_capacity(input.len());

    for line in input.lines() {
        let mut numbers = line.split_whitespace();

        let first: isize = numbers.next().unwrap().parse().unwrap();
        let second: isize = numbers.next().unwrap().parse().unwrap();

        insert_sorted(&mut first_column, first);
        insert_sorted(&mut second_column, second);
    }

    first_column
        .iter()
        .zip(second_column.iter())
        .fold(0, |acc, (f, s)| acc + (f - s).abs())
}

fn part2(input: &str) -> usize {
    let mut first_column = Vec::with_capacity(input.len());
    let mut second_column = Vec::with_capacity(input.len());

    let mut sum = 0;

    for line in input.lines() {
        let mut numbers = line.split_whitespace();

        let first: usize = numbers.next().unwrap().parse().unwrap();
        let second: usize = numbers.next().unwrap().parse().unwrap();

        first_column.push(first);
        second_column.push(second);
    }

    for first in first_column.iter() {
        for second in second_column.iter() {
            if first == second {
                sum += first;
            }
        }
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("inputs/day1.txt").expect("Failed to read file");
    println!("Part 1 = {:?}", part1(&input));
    println!("Part 2 = {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn test_part2() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        assert_eq!(part2(input), 31);
    }
}

