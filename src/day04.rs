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
        let Some(row) = haystack.get(i) else {
            return false;
        };
        let Some(cursor) = row.get(j) else {
            return false;
        };
        if *cursor != matching_character {
            return false;
        }
    }
    true
}

fn is_x_mas(
    wordsearch: &Grid,
    row_index: usize,
    column_index: usize,
    rows: usize,
    columns: usize,
) -> bool {
    // Guarantee all corners are contained within the grid - necessary for a valid X-MAS
    if row_index as isize - 1 < 0
        || row_index + 1 >= rows
        || column_index as isize - 1 < 0
        || column_index + 1 >= columns
    {
        return false;
    }

    let row_above = &wordsearch[row_index - 1];
    let row_below = &wordsearch[row_index + 1];

    match (
        row_above[column_index - 1],
        row_above[column_index + 1],
        row_below[column_index - 1],
        row_below[column_index + 1],
    ) {
        ('M', 'M', 'S', 'S')
        | ('M', 'S', 'M', 'S')
        | ('S', 'M', 'S', 'M')
        | ('S', 'S', 'M', 'M') => true,
        _ => false,
    }
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
                        if search("MAS", parsed_input.to_vec(), i, j, delta_i, delta_j) {
                            count += 1;
                        }
                    }
                }
            }
        }
        count.to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        // Assume "rectangular" input with at least 1 row
        let rows = parsed_input.len();
        let cols = parsed_input[0].len();
        let mut count = 0;
        for i in 0..rows {
            for j in 0..cols {
                let cursor = parsed_input[i][j];
                if cursor != 'A' {
                    continue;
                }
                if is_x_mas(parsed_input, i, j, rows, cols) {
                    count += 1;
                }
            }
        }
        count.to_string()
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
        assert_eq!(
            Day04::solve_part_two(
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
            "9".to_string()
        )
    }

    #[test]
    fn check_day04_both_case1() {
        assert_eq!(Day04::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
