use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day04;

type Grid = Vec<Vec<char>>;

fn search(
    needle: &str,
    haystack: Grid,
    initial_row: usize,
    initial_column: usize,
    row_delta: isize,
    column_delta: isize,
) -> bool {
    let mut i = initial_row;
    let mut j = initial_column;
    for matching_character in needle.chars() {
        let Some(row) = haystack.get(i) else {
            return false;
        };
        let Some(cursor) = row.get(j) else {
            return false;
        };
        if *cursor != matching_character {
            return false;
        }
        if let Ok(new_i) = usize::try_from(i as isize + row_delta) {
            i = new_i;
        } else {
            return false;
        }
        if let Ok(new_j) = usize::try_from(j as isize + column_delta) {
            j = new_j;
        } else {
            return false;
        }
    }
    true
}

impl Solution for Day04 {
    type ParsedInput = Grid;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        input_lines
            .to_string()
            .lines()
            .map(|line| line.chars().collect())
            .collect()
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        // Assume "rectangular" input with at least 1 row
        let rows = parsed_input.len();
        let cols = parsed_input[0].len();
        let mut count = 0;
        for i in 0..rows {
            for j in 0..cols {
                let cursor = parsed_input[i][j];
                if cursor != 'X' {
                    continue;
                }
                // search all directions
                for delta_i in -1..2 {
                    for delta_j in -1..2 {
                        if delta_i == delta_j && delta_i == 0 {
                            continue;
                        }
                        if search("XMAS", parsed_input.to_vec(), i, j, delta_i, delta_j) {
                            count += 1;
                            println!("found at {i}, {j}")
                        }
                    }
                }
            }
        }
        count.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        // TODO: implement part two
        0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day04_part1_case1() {
        assert_eq!(
            Day04::solve_part_one(
                "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            ),
            "18".to_string()
        )
    }

    #[test]
    fn check_day04_part2_case1() {
        assert_eq!(Day04::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day04_both_case1() {
        assert_eq!(Day04::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
