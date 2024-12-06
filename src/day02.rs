use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day02;

fn is_safe(report: &Vec<i8>) -> bool {
    let distances = report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(current, next)| next - current);
    !(distances
        .clone()
        .any(|distance| -3 > distance || 3 < distance || distance == 0)
        || distances.clone().any(|distance| distance < 0)
            && distances.clone().any(|distance| distance > 0))
}

impl Solution for Day02 {
    type ParsedInput = Vec<Vec<i8>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.

        input_lines
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect()
            })
            .collect()
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        parsed_input
            .iter()
            .map(is_safe)
            .filter(|b| *b)
            .count()
            .to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        parsed_input
            .iter()
            .map(|report| {
                let mut possible_removal_lists = vec![];
                for index in 0..report.len() {
                    let mut new_vec = report.clone();
                    new_vec.remove(index);
                    possible_removal_lists.push(new_vec);
                }
                possible_removal_lists
            })
            .map(|report_permutations| report_permutations.iter().map(is_safe).any(|b| b))
            .filter(|b| *b)
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day02_part1_case1() {
        assert_eq!(
            Day02::solve_part_one(
                "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            ),
            "2".to_string()
        )
    }

    #[test]
    fn check_day02_part2_case1() {
        assert_eq!(
            Day02::solve_part_two(
                "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            ),
            "4".to_string()
        )
    }

    #[test]
    fn check_day02_both_case1() {
        assert_eq!(Day02::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
