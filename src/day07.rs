use crate::Solution;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Operator {
    Plus,
    Multiply,
    Concatenate,
}

impl Operator {
    pub fn apply(&self, op1: i64, op2: i64) -> i64 {
        match self {
            Self::Plus => op1 + op2,
            Self::Multiply => op1 * op2,
            Self::Concatenate => (op1.to_string() + &op2.to_string()).parse().unwrap(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Day07;

impl Solution for Day07 {
    type ParsedInput = Vec<(i64, Vec<i64>)>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|line| {
                let mut parts = line.split(":");
                (
                    parts.next().unwrap().parse().unwrap(),
                    parts
                        .next()
                        .unwrap()
                        .split_whitespace()
                        .map(|operand| operand.parse().unwrap())
                        .collect(),
                )
            })
            .collect()
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        parsed_input
            .iter()
            .filter(|(goal, operands)| {
                let num_operators = operands.len() - 1;
                let mut operator_permutations = vec![vec![]];
                for _ in 0..num_operators {
                    let mut next_wave = vec![];
                    while let Some(proto) = operator_permutations.pop() {
                        for operator in [Operator::Multiply, Operator::Plus] {
                            let mut new_vector = proto.clone();
                            new_vector.push(operator);
                            next_wave.push(new_vector);
                        }
                    }
                    operator_permutations = next_wave;
                }

                operator_permutations.iter().any(|permutation| {
                    let operands_iter = operands.iter().map(|el| *el);
                    let mut operator_iter = permutation.into_iter();
                    operands_iter
                        .reduce(|acc, operand| operator_iter.next().unwrap().apply(acc, operand))
                        .unwrap()
                        == *goal
                })
            })
            .map(|(goal, _)| goal)
            .sum::<i64>()
            .to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        parsed_input
            .iter()
            .filter(|(goal, operands)| {
                let num_operators = operands.len() - 1;
                let mut operator_permutations = vec![vec![]];
                for _ in 0..num_operators {
                    let mut next_wave = vec![];
                    while let Some(proto) = operator_permutations.pop() {
                        for operator in [Operator::Multiply, Operator::Plus, Operator::Concatenate]
                        {
                            let mut new_vector = proto.clone();
                            new_vector.push(operator);
                            next_wave.push(new_vector);
                        }
                    }
                    operator_permutations = next_wave;
                }

                operator_permutations.iter().any(|permutation| {
                    let operands_iter = operands.iter().map(|el| *el);
                    let mut operator_iter = permutation.into_iter();
                    operands_iter
                        .reduce(|acc, operand| operator_iter.next().unwrap().apply(acc, operand))
                        .unwrap()
                        == *goal
                })
            })
            .map(|(goal, _)| goal)
            .sum::<i64>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day07_part1_case1() {
        assert_eq!(
            Day07::solve_part_one(
                "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            ),
            "3749".to_string()
        )
    }

    #[test]
    fn check_day07_part2_case1() {
        assert_eq!(
            Day07::solve_part_two(
                "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            ),
            "11387".to_string()
        )
    }

    #[test]
    fn check_day07_both_case1() {
        assert_eq!(Day07::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
