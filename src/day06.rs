use std::{
    fmt::{Display, Write},
    str::FromStr,
};

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day06;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum MapElement {
    Guard(Direction),
    Obstacle,
    Empty,
}

impl Display for MapElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Guard(Direction::Up) => f.write_char('^'),
            Self::Guard(Direction::Down) => f.write_char('v'),
            Self::Guard(Direction::Left) => f.write_char('<'),
            Self::Guard(Direction::Right) => f.write_char('>'),
            Self::Obstacle => f.write_char('#'),
            Self::Empty => f.write_char('.'),
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct ParseMapElementError;

impl FromStr for MapElement {
    type Err = ParseMapElementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "^" => Ok(Self::Guard(Direction::Up)),
            "v" => Ok(Self::Guard(Direction::Down)),
            "<" => Ok(Self::Guard(Direction::Left)),
            ">" => Ok(Self::Guard(Direction::Right)),
            "#" => Ok(Self::Obstacle),
            "." => Ok(Self::Empty),
            _ => Err(ParseMapElementError),
        }
    }
}

pub struct Grid<T>(Vec<Vec<T>>);

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for element in row {
                f.write_str(&element.to_string())?;
            }
            f.write_str("\n")?
        }
        Ok(())
    }
}

impl<T: FromStr> FromStr for Grid<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|row| {
                    row.chars()
                        .map(|character| character.to_string().parse())
                        .collect::<Result<_, _>>()
                })
                .collect::<Result<_, _>>()?,
        ))
    }
}

impl Solution for Day06 {
    type ParsedInput = Grid<MapElement>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines.parse().expect("Couldn't parse input")
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        println!("{parsed_input}");
        0.to_string()
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
    fn check_day06_part1_case1() {
        assert_eq!(
            Day06::solve_part_one(
                "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            ),
            "0".to_string()
        )
    }

    #[test]
    fn check_day06_part2_case1() {
        assert_eq!(Day06::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day06_both_case1() {
        assert_eq!(Day06::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
