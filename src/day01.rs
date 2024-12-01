use itertools::Itertools;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day01;

impl Solution for Day01 {
    type ParsedInput = [Vec<u32>; 2];

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        let mut list1 = Vec::new();
        let mut list2 = Vec::new();
        for line in input_lines.lines() {
            // Assume 2 parts per line.
            let mut parts = line.split_whitespace();
            list1.push(parts.next().unwrap().parse().unwrap());
            list2.push(parts.next().unwrap().parse().unwrap());
        }
        [list1, list2]
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        parsed_input[0]
            .iter()
            .sorted()
            .zip(parsed_input[1].iter().sorted())
            .fold(0, |sum, (list1_item, list2_item)| {
                sum + list1_item.abs_diff(*list2_item)
            })
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
    fn check_day01_part1_case1() {
        assert_eq!(
            Day01::solve_part_one(
                "3   4
4   3
2   5
1   3
3   9
3   3"
            ),
            "11".to_string()
        )
    }

    #[test]
    fn check_day01_part2_case1() {
        assert_eq!(Day01::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day01_both_case1() {
        assert_eq!(Day01::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
