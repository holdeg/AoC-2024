use std::collections::HashSet;

use itertools::Itertools;

use crate::{day06::Grid, Solution};

#[derive(Clone, Debug)]
pub struct Day08;

impl Solution for Day08 {
    type ParsedInput = (Grid<char>, HashSet<char>);

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let grid: Grid<char> = input_lines.parse().unwrap();
        let unique_chars = grid
            .iter()
            .flatten()
            .filter(|ch| **ch != '.')
            .map(|el| *el)
            .unique()
            .collect();

        (grid, unique_chars)
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        let (grid, unique_chars) = parsed_input;
        let (rows, cols) = grid.dimensions();
        unique_chars
            .iter()
            .flat_map(|antenna_type| {
                grid.locate_all(|ch| ch == antenna_type)
                    .into_iter()
                    .map(|(row_index, col_index, _)| (row_index, col_index))
                    .combinations(2)
                    .flat_map(|combination| {
                        let a = combination[0];
                        let b = combination[1];
                        let ab = (b.0 as isize - a.0 as isize, b.1 as isize - a.1 as isize);
                        let a_node = (a.0 as isize - ab.0, a.1 as isize - ab.1);
                        let b_node = (b.0 as isize + ab.0, b.1 as isize + ab.1);
                        let mut valid_nodes = vec![];
                        for node in [a_node, b_node] {
                            if node.0 >= 0
                                && node.0 < rows as isize
                                && node.1 >= 0
                                && node.1 < cols as isize
                            {
                                valid_nodes.push(node);
                            }
                        }
                        valid_nodes
                    })
                    .collect_vec()
            })
            .unique()
            .count()
            .to_string()
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
    fn check_day08_part1_case1() {
        assert_eq!(
            Day08::solve_part_one(
                "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
            ),
            "14".to_string()
        )
    }

    #[test]
    fn check_day08_part2_case1() {
        assert_eq!(Day08::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day08_both_case1() {
        assert_eq!(Day08::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
