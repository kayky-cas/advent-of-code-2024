use std::str::Chars;

// Path: src/bin/day4.rs
fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(str::chars).map(Chars::collect).collect();
    let directions = [(1, 0), (0, 1), (1, 1), (1, -1)];

    let words = ["XMAS", "SAMX"];

    let rows = grid.len();
    let cols = grid.first().map_or(0, Vec::len);

    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &directions {
                'words: for word in &words {
                    for (k, ch) in word.chars().enumerate() {
                        let ni = i as isize + k as isize * dx;
                        let nj = j as isize + k as isize * dy;

                        if !(0..rows as isize).contains(&ni)
                            || !(0..cols as isize).contains(&nj)
                            || grid[ni as usize][nj as usize] != ch
                        {
                            continue 'words;
                        }
                    }

                    count += 1;
                }
            }
        }
    }
    count
}

fn part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(str::chars).map(Chars::collect).collect();

    let words = ["MAS", "SAM"];

    let rows = grid.len();
    let cols = grid.first().map_or(0, Vec::len);

    let mut count = 0;

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            if grid[i][j] != 'A' {
                continue;
            }

            let mut first_word = String::with_capacity(3);

            first_word.push(grid[i - 1][j - 1]);
            first_word.push(grid[i][j]);
            first_word.push(grid[i + 1][j + 1]);

            if !words.contains(&first_word.as_str()) {
                continue;
            }

            let mut second_word = String::with_capacity(3);
            second_word.push(grid[i - 1][j + 1]);
            second_word.push(grid[i][j]);
            second_word.push(grid[i + 1][j - 1]);

            if words.contains(&second_word.as_str()) {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let input = std::fs::read_to_string("inputs/day4.txt").expect("Failed to read file");
    println!("Part 1 = {:?}", part1(&input));
    println!("Part 2 = {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 9);
    }
}
