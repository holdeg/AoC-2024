use itertools::Itertools;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day05;

#[derive(Debug, Default)]
pub struct Parsed {
    ruleset: RuleSet,
    updates: Vec<Update>,
}

#[derive(Debug, Default)]
struct RuleSet {
    rules: Vec<Rule>,
}

impl RuleSet {
    fn rules_involving(&self, pages: &Vec<u32>) -> Vec<Rule> {
        self.rules
            .iter()
            .filter(|rule| pages.contains(&rule.before) && pages.contains(&rule.after))
            .map(|rule| *rule)
            .collect()
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
struct Rule {
    before: u32,
    after: u32,
}

impl From<Rule> for [u32; 2] {
    fn from(value: Rule) -> Self {
        [value.before, value.after]
    }
}

impl Update {
    fn obeys(&self, rule: &Rule) -> bool {
        let mut copy = self.0.clone();
        copy.retain(|el| *el == rule.before || *el == rule.after);
        return copy.len() < 2 || copy[0] == rule.before;
    }

    fn satisfies(&self, ruleset: &RuleSet) -> bool {
        !ruleset.rules.iter().any(|rule| !self.obeys(rule))
    }

    fn with_rules(&self, ruleset: &RuleSet) -> Self {
        if self.satisfies(ruleset) {
            return self.clone();
        }

        let rules = ruleset.rules_involving(&self.0);

        let mut remaining_rules = rules.clone();
        let mut ordered = vec![];

        let tail = rules
            .clone()
            .iter()
            .map(|rule| rule.after)
            .find(|page| !rules.iter().map(|rule| rule.before).contains(&page))
            .expect("No last page found")
            .to_owned();

        while remaining_rules.len() > 0 {
            let head = remaining_rules
                .clone()
                .iter()
                .map(|rule| rule.before)
                .find(|page| {
                    !remaining_rules
                        .iter()
                        .map(|rule| rule.after)
                        .contains(&page)
                })
                .expect("No first page found")
                .to_owned();
            ordered.push(head);
            remaining_rules.retain(|rule| rule.before != head);
        }
        ordered.push(tail);

        Self(ordered)
    }
}

impl From<(u32, u32)> for Rule {
    fn from(value: (u32, u32)) -> Self {
        Self {
            before: value.0,
            after: value.1,
        }
    }
}

impl From<(&str, &str)> for Rule {
    fn from(value: (&str, &str)) -> Self {
        (value.0.parse::<u32>().unwrap(), value.1.parse().unwrap()).into()
    }
}

#[derive(Debug, Default, Clone)]
struct Update(Vec<u32>);

impl Solution for Day05 {
    type ParsedInput = Parsed;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let mut lines = input_lines.lines();
        let mut rules: Vec<Rule> = vec![];
        while let Some(line) = lines.next() {
            if line.trim().is_empty() {
                break;
            }
            let pages = line.split("|").collect::<Vec<_>>();
            rules.push((pages[0], pages[1]).into());
        }
        Parsed {
            ruleset: RuleSet { rules },
            updates: lines
                .map(|update| {
                    Update(
                        update
                            .split(",")
                            .map(|page| page.parse::<u32>().unwrap())
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
        }
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        parsed_input
            .updates
            .iter()
            .filter(|update| update.satisfies(&parsed_input.ruleset))
            .filter_map(|update| {
                // Assume odd length
                let half = update.0.len() / 2;
                update.0.iter().skip(half).next()
            })
            .sum::<u32>()
            .to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        parsed_input
            .updates
            .iter()
            .filter(|update| !update.satisfies(&parsed_input.ruleset))
            .map(|update| update.with_rules(&parsed_input.ruleset))
            .filter_map(|update| {
                // Assume odd length
                let half = update.0.len() / 2;
                update.0.iter().skip(half).next().map(|i| *i)
            })
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day05_part1_case1() {
        assert_eq!(
            Day05::solve_part_one(
                "47|53
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
97,13,75,29,47"
            ),
            "143".to_string()
        )
    }

    #[test]
    fn check_day05_part2_case1() {
        assert_eq!(
            Day05::solve_part_two(
                "47|53
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
97,13,75,29,47"
            ),
            "123".to_string()
        )
    }

    #[test]
    fn check_day05_both_case1() {
        assert_eq!(Day05::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
