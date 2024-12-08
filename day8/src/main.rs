use std::fs;
use std::ops::{Add, Sub};

fn main() {
    let input = fs::read_to_string("day8/resources/input.txt")
        .expect("Should have been able to read the file");

    let part_1_answer = part1::solution(&input)
        .expect("Failed to solve part 1.");
    println!("Day 8 Part 1 answer: {}", part_1_answer);

    let part_2_answer = part2::solution(&input)
        .expect("Failed to solve part 2.");
    println!("Day 8 Part 2 answer: {}", part_2_answer);
}

mod part1 {
    use std::collections::HashMap;
    use itertools::Itertools;
    use crate::{Frequency, IVec, parse};

    pub fn solution(input: &str) -> anyhow::Result<String> {
        let nodes = parse(input);
        let max_x = input.lines().map(str::len).max().unwrap_or(0) as isize;
        let max_y = input.lines().collect::<Vec<_>>().len() as isize;

        let fer_to_pos: HashMap<Frequency, Vec<IVec>> = nodes.iter()
            .fold(HashMap::new(), |mut map, node| {
                map.entry(node.freq).or_default().push(node.pos);
                map
            });
        let antinodes: Vec<_> = fer_to_pos.iter()
            .flat_map(|(_, v)| calculate_antinodes(v))
            .collect();
        let count_of_antinodes = antinodes.iter()
            .unique()
            .filter(|an| an.is_within_boundary(0, 0, max_x, max_y))
            .count();
        Ok(count_of_antinodes.to_string())
    }

    fn calculate_antinodes(nodes: &[IVec]) -> Vec<IVec> {
        let mut result = vec![];
        for &l in nodes.iter() {
            for &r in nodes.iter() {
                if l == r {
                    continue;
                }
                result.push(l - r + l);
            }
        }
        return result;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn part1_example_test() {
            // given
            let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

            // when
            let result = solution(input).unwrap();

            // then
            assert_eq!(result, "14");
        }
    }
}

mod part2 {
    use std::collections::HashMap;
    use itertools::Itertools;
    use crate::{Frequency, IVec, parse};

    pub fn solution(input: &str) -> anyhow::Result<String> {
        let nodes = parse(input);
        let max_x = input.lines().map(str::len).max().unwrap_or(0) as isize;
        let max_y = input.lines().collect::<Vec<_>>().len() as isize;

        let fer_to_pos: HashMap<Frequency, Vec<IVec>> = nodes.iter()
            .fold(HashMap::new(), |mut map, node| {
                map.entry(node.freq).or_default().push(node.pos);
                map
            });
        let antinodes: Vec<_> = fer_to_pos.iter()
            .flat_map(|(_, v)| calculate_antinodes(v, max_x, max_y))
            .collect();
        let count_of_antinodes = antinodes.iter()
            .unique()
            // .filter(|an| an.is_within_boundary(0, 0, max_x, max_y))
            .count();
        Ok(count_of_antinodes.to_string())
    }

    fn calculate_antinodes(nodes: &[IVec], max_x: isize, max_y: isize) -> Vec<IVec> {
        let mut result = vec![];
        for &l in nodes.iter() {
            'x_loop: for &r in nodes.iter() {
                if l == r {
                    result.push(l);
                    continue 'x_loop;
                }
                let diff = l - r;
                let mut an = diff + l;
                'inner: loop {
                    if !an.is_within_boundary(0, 0, max_x, max_y) {
                        break 'inner;
                    };
                    result.push(an);
                    an = diff + an;
                }
            }
        }
        return result;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn part2_example_test() {
            // given
            let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

            // when
            let result = solution(input).unwrap();

            // then
            assert_eq!(result, "34");
        }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash)]
struct IVec {
    x: isize,
    y: isize,
}

impl IVec {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn is_within_boundary(&self, min_x: isize, min_y: isize, max_x: isize, max_y: isize) -> bool {
        self.x >= min_x && self.x < max_x &&
            self.y >= min_y && self.y < max_y
    }
}

impl Add<IVec> for IVec {
    type Output = IVec;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<IVec> for IVec {
    type Output = IVec;

    fn sub(self, rhs: IVec) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

type Frequency = char;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
struct Node {
    pos: IVec,
    freq: Frequency,
}

fn parse(input: &str) -> Vec<Node> {
    return input.lines().enumerate()
        .flat_map(|(y, l)| l.chars().enumerate()
            .filter_map(move |(x, c)| match c {
                '.' => None,
                _ => Some(Node { pos: IVec::new(x as isize, y as isize), freq: c })
            })
            .collect::<Vec<_>>()).collect();
}

