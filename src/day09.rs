use itertools::Itertools;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day09;

impl Solution for Day09 {
    type ParsedInput = Vec<i64>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let mut file = true;
        let mut id = 0;
        input_lines
            .chars()
            .flat_map(|ch| {
                let digit = ch.to_digit(10).unwrap();
                if file {
                    file = false;
                    vec![id; digit as usize]
                } else {
                    file = true;
                    id += 1;
                    vec![-1; digit as usize]
                }
            })
            .collect_vec()
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        let file_blocks = parsed_input.iter().filter(|id| **id != -1).count();
        let is_compacted = |disk: Vec<i64>| {
            disk.iter()
                .position(|id| *id == -1)
                .is_some_and(|idx| idx == file_blocks)
        };
        while !is_compacted(parsed_input.clone()) {
            let first_empty_block = parsed_input.iter().position(|id| *id == -1).unwrap();
            let last_file_block = parsed_input.iter().rposition(|id| *id != -1).unwrap();
            parsed_input.swap(first_empty_block, last_file_block);
        }
        parsed_input
            .into_iter()
            .filter(|id| **id != -1)
            .enumerate()
            .fold(0, |acc, (idx, id)| acc + idx as i64 * *id)
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
    fn check_day09_part1_case1() {
        assert_eq!(
            Day09::solve_part_one("2333133121414131402"),
            "1928".to_string()
        )
    }

    #[test]
    fn check_day09_part2_case1() {
        assert_eq!(Day09::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day09_both_case1() {
        assert_eq!(Day09::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
