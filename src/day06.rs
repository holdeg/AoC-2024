use std::{
    collections::HashSet,
    fmt::{Display, Write},
    ops::Index,
    slice::{Iter, SliceIndex},
    str::FromStr,
};

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day06;

#[derive(Clone)]
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

impl<T> Grid<T> {
    pub fn get(&self, row_index: usize, col_index: usize) -> Option<&T> {
        self.0.get(row_index).and_then(|row| row.get(col_index))
    }

    pub fn get_pos(&self, pos: (usize, usize)) -> Option<&T> {
        self.get(pos.0, pos.1)
    }

    pub fn put(&mut self, row_index: usize, col_index: usize, element: T) -> bool {
        self.0
            .get_mut(row_index)
            .and_then(|row| {
                row.get_mut(col_index).and_then(|old| {
                    *old = element;
                    Some(())
                })
            })
            .is_some()
    }

    pub fn iter(&self) -> Iter<'_, Vec<T>> {
        self.0.iter()
    }

    pub fn dimensions(&self) -> (usize, usize) {
        // Assume rectangular, but cope with empty data structure
        (self.0.len(), self.0.get(0).map_or(0, Vec::len))
    }

    /// Convenience function for row by index.
    pub fn row(&self, row_index: usize) -> Option<Vec<&T>> {
        self.0.get(row_index).map(|row| row.iter().collect())
    }

    /// Constructs the column of this grid specified by the column index
    /// and returns it.
    pub fn column(&self, col_index: usize) -> Option<Vec<&T>> {
        self.iter().map(|row| row.get(col_index)).collect()
    }

    /// Searches for the first (row-major ordering) element contained in
    /// the grid that satisfies a predicate, and return it with its location
    /// if found.
    ///
    /// Compare [`Iterator::find`], which does the same thing over a normal
    /// [Iterator] (without returning index) - this wraps that operation
    /// with [`Iterator::enumerate`] over two dimensions to return the position.
    /// Compare also [`Iterator::position`], but this does not return the element
    /// itself (though it could be subsequently obtained).
    pub fn locate<P>(&self, mut predicate: P) -> Option<(usize, usize, &T)>
    where
        P: FnMut(&T) -> bool,
    {
        self.iter()
            .enumerate()
            .filter_map(|(row_index, row)| {
                row.iter()
                    .enumerate()
                    .find(|(_, element)| predicate(element))
                    .map(|(col_index, element)| (row_index, col_index, element))
            })
            .next()
    }

    /// Return all elements (with their positions) contained in the grid that satisfiy a
    /// predicate.
    ///
    /// Compare [`Iterator::filter`], which does the same thing over a normal
    /// [Iterator] (without returning index) - this wraps that operation
    /// with [`Iterator::enumerate`] over two dimensions to return the position.
    pub fn locate_all<P>(&self, mut predicate: P) -> Vec<(usize, usize, &T)>
    where
        P: FnMut(&T) -> bool,
    {
        self.iter()
            .enumerate()
            .flat_map(|(row_index, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, element)| predicate(element))
                    .map(|(col_index, element)| (row_index, col_index, element))
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

impl<T> Grid<T> {
    pub fn walk(
        &self,
        row_index: usize,
        col_index: usize,
        direction: &Direction,
    ) -> Option<(usize, usize, &T)> {
        match direction {
            Direction::Up => match row_index.overflowing_sub(1) {
                (_, true) => None,
                (result, false) => self
                    .get(result, col_index)
                    .map(|el| (result, col_index, el)),
            },
            Direction::Down => self
                .get(row_index + 1, col_index)
                .map(|el| (row_index + 1, col_index, el)),
            Direction::Left => match col_index.overflowing_sub(1) {
                (_, true) => None,
                (result, false) => self
                    .get(row_index, result)
                    .map(|el| (row_index, result, el)),
            },
            Direction::Right => self
                .get(row_index, col_index + 1)
                .map(|el| (row_index, col_index + 1, el)),
        }
    }
}

impl<T, Idx: SliceIndex<[Vec<T>], Output = Vec<T>>> Index<Idx> for Grid<T> {
    type Output = Vec<T>;

    fn index(&self, index: Idx) -> &Self::Output {
        self.0.index(index)
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = Vec<T>;
    type IntoIter = <Vec<Vec<T>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

fn is_looped(
    grid: &Grid<MapElement>,
    row_index: usize,
    col_index: usize,
    mut direction: Direction,
) -> bool {
    let mut walked_positions: HashSet<(usize, usize, Direction)> = HashSet::new();
    let mut i = row_index;
    let mut j = col_index;

    loop {
        if walked_positions.contains(&(i, j, direction)) {
            return true;
        }
        walked_positions.insert((i, j, direction));
        match grid.walk(i, j, &direction) {
            None => return false,
            Some((_, _, MapElement::Obstacle)) => direction = direction.turn_right(),
            Some((next_i, next_j, _)) => (i, j) = (next_i, next_j),
        }
    }
}

impl Solution for Day06 {
    type ParsedInput = Grid<MapElement>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines.parse().expect("Couldn't parse input")
    }

    fn part_one(grid: &mut Self::ParsedInput) -> String {
        let (row, col, guard) = grid
            .locate(|element| matches!(element, MapElement::Guard(..)))
            .expect("Could not find the guard");

        let mut direction = *match guard {
            MapElement::Guard(direction) => direction,
            _ => unreachable!("Guard is not a guard"),
        };
        let mut walked_positions: HashSet<(usize, usize)> = HashSet::new();
        let mut i = row;
        let mut j = col;

        // The following loop assumes we do terminate - i.e., no positions where the
        // guard is in some sense "surrounded" by obstacles, and thus will at some point
        // exit the grid.
        loop {
            walked_positions.insert((i, j));
            match grid.walk(i, j, &direction) {
                None => break,
                Some((_, _, MapElement::Obstacle)) => direction = direction.turn_right(),
                Some((next_i, next_j, _)) => (i, j) = (next_i, next_j),
            }
        }

        walked_positions.len().to_string()
    }

    fn part_two(grid: &mut Self::ParsedInput) -> String {
        let (row, col, guard) = grid
            .locate(|element| matches!(element, MapElement::Guard(..)))
            .expect("Could not find the guard");

        let initial_direction = match guard {
            MapElement::Guard(direction) => direction,
            _ => unreachable!("Guard is not a guard"),
        };

        let mut direction = *initial_direction;

        let mut potential_obstacles: HashSet<(usize, usize)> = HashSet::new();
        let mut i = row;
        let mut j = col;

        // The following loop assumes we do terminate - i.e., no positions where the
        // guard is in some sense "surrounded" by obstacles, and thus will at some point
        // exit the grid.
        loop {
            match grid.walk(i, j, &direction) {
                None => break,
                Some((_, _, MapElement::Obstacle)) => direction = direction.turn_right(),
                Some((next_i, next_j, _)) => {
                    let mut grid_with_obstacle = grid.clone();
                    grid_with_obstacle.put(next_i, next_j, MapElement::Obstacle);
                    // This sucks. The code naively checks the entire resulting grid
                    // for a loop to decide if this is a potential obstacle spot,
                    // which is super expensive. But it's Christmas Day and I can't be
                    // bothered to think of a better way to do it right now.
                    //
                    // So there.
                    if is_looped(&grid_with_obstacle, row, col, *initial_direction) {
                        potential_obstacles.insert((next_i, next_j));
                    }
                    (i, j) = (next_i, next_j);
                }
            }
        }

        potential_obstacles.len().to_string()
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
            "41".to_string()
        )
    }

    #[test]
    fn check_day06_part2_case1() {
        assert_eq!(
            Day06::solve_part_two(
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
            "6".to_string()
        )
    }

    #[test]
    fn check_day06_both_case1() {
        assert_eq!(Day06::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
