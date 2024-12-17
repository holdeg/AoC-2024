use once_cell::sync::Lazy;
use regex::Regex;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day03;

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());

fn sum_multiplications(input: &str) -> u32 {
    RE.captures_iter(input)
        .map(|captures| {
            captures.iter().skip(1).fold(1u32, |prod, el| {
                prod * el.unwrap().as_str().parse::<u32>().unwrap()
            })
        })
        .sum()
}

impl Solution for Day03 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let mut input = input_lines.to_string();
        input.retain(|char| !char.is_whitespace());
        input
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        sum_multiplications(parsed_input).to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        sum_multiplications(
            &parsed_input
                .split("do()")
                .map(|enabled_set| {
                    enabled_set
                        .split("don't()")
                        .next()
                        .unwrap_or(enabled_set)
                        .to_string()
                })
                .collect::<Vec<String>>()
                .join(""),
        )
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day03_part1_case1() {
        assert_eq!(
            Day03::solve_part_one(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            ),
            "161".to_string()
        )
    }

    #[test]
    fn check_day03_part2_case1() {
        assert_eq!(
            Day03::solve_part_two(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            ),
            "48".to_string()
        )
    }

    #[test]
    fn check_day03_both_case1() {
        assert_eq!(Day03::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
