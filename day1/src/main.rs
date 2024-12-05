use std::fs;
use nom::character::complete::{char, digit1, newline};
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::{many1, separated_list0};
use nom::sequence::separated_pair;

fn main() {
    let input = fs::read_to_string("day1/resources/input.txt")
        .expect("Should have been able to read the file");

    let part_1_answer = part1::solution(&input)
        .expect("Failed to solve part 1.");
    println!("Day 1 Part 1 answer: {}", part_1_answer);

    let part_2_answer = part2::solution(&input)
        .expect("Failed to solve part 2.");
    println!("Day 1 Part 2 answer: {}", part_2_answer);
}

mod part1 {
    use itertools::Itertools;
    use super::parse;

    pub fn solution(input: &str) -> anyhow::Result<String> {
        let (_, pairs) = parse(input).map_err(|err| err.to_owned())?;
        let (lefts, rights): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
        let sum: u64 = lefts.iter().sorted().zip(rights.iter().sorted())
            .map(|(l, r)| l.abs_diff(*r))
            .sum();
        Ok(sum.to_string())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn part1_example_test() {
            // given
            let input = "3   4
4   3
2   5
1   3
3   9
3   3";
            // when
            let solution = solution(input).unwrap();

            // then
            assert_eq!(solution, "11".to_owned());
        }
    }
}

mod part2 {
    use std::collections::HashMap;
    use std::hash::Hash;
    use super::parse;

    pub fn solution(input: &str) -> anyhow::Result<String> {
        let (_, pairs) = parse(input).map_err(|err| err.to_owned())?;
        let (lefts, rights): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
        let frequency = group_with_count(rights);
        let sum: u64 = lefts.into_iter()
            .map(|i| i * (*frequency.get(&i).unwrap_or(&0) as u64))
            .sum();
        Ok(sum.to_string())
    }

    fn group_with_count<I: Eq + Hash>(iterator: impl IntoIterator<Item=I>) -> HashMap<I, usize> {
        let mut collector: HashMap<I, usize> = HashMap::new();
        for item in iterator {
            let freq: &mut usize = collector.entry(item).or_insert(0);
            *freq += 1;
        }
        collector
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn part2_example_test() {
            // given
            let input = "3   4
4   3
2   5
1   3
3   9
3   3";
            // when
            let solution = solution(input).unwrap();

            // then
            assert_eq!(solution, "31");
        }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    separated_list0(newline, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(map_res(digit1, str::parse), many1(char(' ')), map_res(digit1, str::parse))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_test() {
        // given
        let line = "1   4\n";

        // when
        let (_, result) = parse_line(line).unwrap();

        // then
        assert_eq!(result, (1, 4));
    }

    #[test]
    fn parse_test() {
        // given
        let line = "3   4
4   3
2   5";

        // when
        let (_, result) = parse(line).unwrap();

        // then
        assert_eq!(result, vec![(3, 4), (4, 3), (2, 5)]);
    }
}