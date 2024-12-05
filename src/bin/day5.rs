use std::collections::HashMap;

// Path: src/bin/day5.rs
fn part1(input: &str) -> usize {
    let (rules, updates) = input.split_once("\n\n").expect("Invalid input");

    let rules = rules
        .lines()
        .flat_map(|line| {
            line.split_once('|')
                .and_then(|(r, l)| Some((r.parse().ok()?, l.parse().ok()?)))
        })
        .fold(HashMap::<usize, Vec<usize>>::new(), |mut acc, (l, r)| {
            acc.entry(r).and_modify(|v| v.push(l)).or_insert(vec![l]);
            acc
        });

    updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|u| u.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|v| {
            for i in 0..v.len() {
                let Some(r) = rules.get(&v[i]) else { continue };
                for y in v.iter().skip(i + 1) {
                    if r.iter().any(|z| z == y) {
                        return false;
                    }
                }
            }

            true
        })
        .map(|v| v[v.len() / 2])
        .sum()
}

fn part2(input: &str) -> usize {
    let (rules, updates) = input.split_once("\n\n").expect("Invalid input");

    let rules = rules
        .lines()
        .flat_map(|line| {
            line.split_once('|')
                .and_then(|(r, l)| Some((r.parse().ok()?, l.parse().ok()?)))
        })
        .fold(HashMap::<usize, Vec<usize>>::new(), |mut acc, (l, r)| {
            acc.entry(r).and_modify(|v| v.push(l)).or_insert(vec![l]);
            acc
        });

    updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|u| u.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter_map(|mut v| {
            let mut change = false;

            for i in 0..v.len() {
                for y in i..v.len() {
                    let Some(r) = rules.get(&v[i]) else { continue };
                    if r.iter().any(|z| z == &v[y]) {
                        v.swap(i, y);
                        change = true;
                    }
                }
            }

            if change {
                Some(v)
            } else {
                None
            }
        })
        .map(|v| v[v.len() / 2])
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("inputs/day5.txt").expect("Failed to read file");
    println!("Part 1 = {:?}", part1(&input));
    println!("Part 2 = {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 123);
    }
}
