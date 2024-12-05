use std::fs;

fn main() {
    let input = fs::read_to_string("day4/resources/input.txt")
        .expect("Should have been able to read the file");

    let part_1_answer = part1::solution(&input)
        .expect("Failed to solve part 1.");
    println!("Day 4 Part 1 answer: {}", part_1_answer);

    let part_2_answer = part2::solution(&input)
        .expect("Failed to solve part 2.");
    println!("Day 4 Part 2 answer: {}", part_2_answer);
}

mod part1 {
    use crate::{match_pattern, parse};

    //   ______> x
    //   |
    //   |
    //  \/
    //   y

    pub fn solution(input: &str) -> anyhow::Result<String> {
        let grid = parse(input);
        let grid_slice: Vec<_> = grid.iter().map(|r| r.as_slice()).collect();
        let patterns = [
            [('X', 0, 0), ('M', 1, 0), ('A', 2, 0), ('S', 3, 0)], // ->
            [('X', 0, 0), ('M', -1, 0), ('A', -2, 0), ('S', -3, 0)], // <-
            [('X', 0, 0), ('M', 0, 1), ('A', 0, 2), ('S', 0, 3)], // \/
            [('X', 0, 0), ('M', 0, -1), ('A', 0, -2), ('S', 0, -3)], // /\
            [('X', 0, 0), ('M', 1, 1), ('A', 2, 2), ('S', 3, 3)], // \/ >
            [('X', 0, 0), ('M', -1, 1), ('A', -2, 2), ('S', -3, 3)], // \/ <
            [('X', 0, 0), ('M', 1, -1), ('A', 2, -2), ('S', 3, -3)], // \/ >
            [('X', 0, 0), ('M', -1, -1), ('A', -2, -2), ('S', -3, -3)], // /\ <
        ];
        let mut occurrences = 0;
        for (y, inner) in grid.iter().enumerate() {
            for (x, _) in inner.iter().enumerate() {
                if match_any_pattern(&grid_slice, patterns, y, x) {
                    occurrences += 1;
                }
            }
        }
        Ok(occurrences.to_string())
    }

    fn match_any_pattern(grid_slice: &Vec<&[char]>, patterns: [[(char, isize, isize); 4]; 8], y: usize, x: usize) -> bool {
        for pattern in patterns {
            if match_pattern(y, x, &grid_slice, &pattern) {
                return true;
            }
        }
        return false;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn part1_example_test() {
            // given
            let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
            // when
            let solution = solution(input).unwrap();

            // then
            assert_eq!(solution, "18".to_owned());
        }
    }
}

mod part2 {
    use crate::{match_pattern, parse};

    pub fn solution(input: &str) -> anyhow::Result<String> {
        let grid = parse(input);
        let grid_slice: Vec<_> = grid.iter().map(|r| r.as_slice()).collect();
        let patterns = [
            [[('M', -1, -1), ('A', 0, 0), ('S', 1, 1)], [('M', -1, 1), ('A', 0, 0), ('S', 1, -1)]],
            [[('M', -1, -1), ('A', 0, 0), ('S', 1, 1)], [('M', 1, -1), ('A', 0, 0), ('S', -1, 1)]],
            [[('M', 1, -1), ('A', 0, 0), ('S', -1, 1)], [('M', 1, 1), ('A', 0, 0), ('S', -1, -1)]],
            [[('M', -1, 1), ('A', 0, 0), ('S', 1, -1)], [('M', 1, 1), ('A', 0, 0), ('S', -1, -1)],
            ]
        ];
        let mut occurrences = 0;
        for (y, inner) in grid.iter().enumerate() {
            for (x, _) in inner.iter().enumerate() {
                for sub_patters in patterns {
                    if match_all_pattern(x, y, &sub_patters, &grid_slice) {
                        occurrences += 1;
                    }
                }
            }
        }
        Ok(occurrences.to_string())
    }

    fn match_all_pattern(x: usize, y: usize, patterns: &[[(char, isize, isize); 3]; 2], grid_slice: &[&[char]]) -> bool {
        for pattern in patterns {
            if !match_pattern(y, x, grid_slice, pattern) {
                return false;
            }
        }
        return true;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn part2_example_test() {
            // given
            let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
            // when
            let solution = solution(input).unwrap();

            // then
            assert_eq!(solution, "9".to_owned());
        }
    }
}

fn match_pattern(y: usize, x: usize, grid: &[&[char]], pattern: &[(char, isize, isize)]) -> bool {
    for (c, x_offset, y_offset) in pattern {
        let yo = y as isize + y_offset;
        if yo < 0 || yo >= grid.len() as isize { return false; };
        let xo = x as isize + x_offset;
        if xo < 0 || xo >= grid[0].len() as isize { return false; };
        let v = grid[yo as usize][xo as usize];
        if v != *c {
            return false;
        }
    }
    return true;
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars().collect())
        .collect()
}
