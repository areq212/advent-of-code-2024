use std::fs;
use std::ops::Add;

fn main() {
    let input = fs::read_to_string("day6/resources/input.txt")
        .expect("Should have been able to read the file");

    let part_1_answer = part1::solution(&input)
        .expect("Failed to solve part 1.");
    println!("Day 6 Part 1 answer: {}", part_1_answer);

    let part_2_answer = part2::solution(&input)
        .expect("Failed to solve part 2.");
    println!("Day 6 Part 2 answer: {}", part_2_answer);
}

mod part1 {
    use std::collections::HashSet;

    use anyhow::Context;

    use crate::{find_guard_initial_pos_and_dir, parse};

    pub fn solution(input: &str) -> anyhow::Result<String> {
        let grid = parse(input);

        let grid_x_max = grid.iter().map(Vec::len).min().context("grid has now rows data")?;
        let grid_y_max = grid.len();
        let (mut guard_pos, mut guard_dir) = find_guard_initial_pos_and_dir(&grid).context("failed to find guard on the grid")?;
        let mut visited_pos = HashSet::new();

        'outer: loop {
            visited_pos.insert(guard_pos);

            'inner: loop {
                let next_pos = guard_pos + guard_dir;
                if !next_pos.is_within_boundary(grid_x_max, grid_y_max) {
                    break 'outer;
                }
                if grid[next_pos.1][next_pos.0] == '#' {
                    guard_dir = guard_dir.turn_right();
                } else {
                    guard_pos = next_pos;
                    break 'inner;
                }
            }
        }
        Ok(visited_pos.len().to_string())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn part1_example_test() {
            // given
            let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
            // when
            let solution = solution(input).unwrap();

            // then
            assert_eq!(solution, "41".to_owned());
        }
    }
}

mod part2 {
    use std::collections::HashSet;

    use anyhow::Context;

    use crate::{find_guard_initial_pos_and_dir, parse};

    pub fn solution(input: &str) -> anyhow::Result<String> {
        let grid = parse(input);

        let grid_x_max = grid.iter().map(Vec::len).min().context("grid has now rows data")?;
        let grid_y_max = grid.len();
        let (mut guard_pos, mut guard_dir) = find_guard_initial_pos_and_dir(&grid).context("failed to find guard on the grid")?;
        let mut visited_pos = HashSet::new();
        let mut placed_hashes = HashSet::new();
        'outer: loop {
            visited_pos.insert((guard_pos, guard_dir));

            'inner: loop {
                let next_pos = guard_pos + guard_dir;
                if !next_pos.is_within_boundary(grid_x_max, grid_y_max) {
                    break 'outer;
                }
                if grid[next_pos.1][next_pos.0] == '#' {
                    guard_dir = guard_dir.turn_right();
                    continue 'inner;
                }
                let mut cloned_grid: Vec<Vec<char>> = grid.iter().cloned().collect();
                cloned_grid[next_pos.1][next_pos.0] = 'X';
                let mut cloned_guard_pos = guard_pos.clone();
                let mut cloned_guard_dir = guard_dir.clone();
                let mut cloned_visited_pos = HashSet::new();
                'checking: loop {
                    let cloned_next_pos = cloned_guard_pos + cloned_guard_dir;
                    if !cloned_next_pos.is_within_boundary(grid_x_max, grid_y_max) {
                        break 'checking;
                    }
                    let char = cloned_grid[cloned_next_pos.1][cloned_next_pos.0];
                    if char == '#' || char == 'X' {
                        cloned_guard_dir = cloned_guard_dir.turn_right();
                        continue 'checking;
                    }
                    if cloned_visited_pos.contains(&(cloned_next_pos, cloned_guard_dir)) {
                        placed_hashes.insert(next_pos);
                        break 'checking;
                    }
                    cloned_visited_pos.insert((cloned_next_pos, cloned_guard_dir));
                    cloned_guard_pos = cloned_next_pos;

                }
                guard_pos = next_pos;
                break 'inner;
            }
        }

        Ok(placed_hashes.len().to_string())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn part2_example_test() {
            // given
            let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
            // when
            let solution = solution(input).unwrap();

            // then
            assert_eq!(solution, "6".to_owned());
        }
    }
}

fn find_guard_initial_pos_and_dir(grid: &Grid) -> Option<(Position, Direction)> {
    grid.iter().enumerate()
        .find_map(|(y, row)|
        row.iter().enumerate()
            .find_map(
                move |(x, c)|
                match c {
                    '>' => Some((Position(x, y), Direction(1, 0))),
                    '^' => Some((Position(x, y), Direction(0, -1))),
                    'v' => Some((Position(x, y), Direction(0, 1))),
                    '<' => Some((Position(x, y), Direction(-1, 0))),
                    _ => None
                }
            ))
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position(usize, usize);

impl Position {
    fn is_within_boundary(&self, max_x: usize, max_y: usize) -> bool {
        self.0 < max_x && self.1 < max_y
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, rhs: Direction) -> Self::Output {
        Position((self.0 as isize + rhs.0 as isize) as usize, (self.1 as isize + rhs.1 as isize) as usize)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Direction(i8, i8);

type Grid = Vec<Vec<char>>;

impl Direction {
    fn turn_right(&self) -> Direction {
        let x = self.1 * -1;
        let y = self.0 * 1;
        Self(x, y)
    }
}

fn parse(input: &str) -> Grid {
    input.lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_right_test() {
        // given
        let v = Direction(1, 0); // >

        // when
        let v = v.turn_right();

        // then
        assert_eq!(v, Direction(0, 1)); // \/

        // when
        let v = v.turn_right();

        // then
        assert_eq!(v, Direction(-1, 0)); // <

        // when
        let v = v.turn_right();

        // then
        assert_eq!(v, Direction(0, -1)); // /\

        // when
        let v = v.turn_right();

        // then
        assert_eq!(v, Direction(1, 0)); // >
    }

    #[test]
    fn position_is_within_boundary_test() {
        // give
        let pos = Position(8, 10);

        // when
        let res = pos.is_within_boundary(10, 10);

        // then
        assert!(!res);
    }
}