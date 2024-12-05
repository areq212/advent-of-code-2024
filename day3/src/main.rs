use std::fs;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::anychar;
use nom::combinator::map;
use nom::IResult;
use nom::multi::many0;
use nom::sequence::Tuple;

fn main() {
    let input = fs::read_to_string("day3/resources/input.txt")
        .expect("Should have been able to read the file");

    let part_1_answer = part1::solution(&input)
        .expect("Failed to solve part 1.");
    println!("Day 3 Part 1 answer: {}", part_1_answer);

    let part_2_answer = part2::solution(&input)
        .expect("Failed to solve part 2.");
    println!("Day 3 Part 2 answer: {}", part_2_answer);
}

mod part1 {
    use crate::{Operation, parse};

    pub fn solution(input: &str) -> anyhow::Result<String> {
        let (_, operations) = parse(input).map_err(|err| err.to_owned())?;
        let sum: u64 = operations.iter()
            .filter_map(|o| match o {
                Operation::Mul(a, b) => Some(a * b),
                _ => None
            })
            .sum();
        Ok(sum.to_string())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn part1_example_test() {
            // given
            let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

            // when
            let result = solution(input).unwrap();

            // then
            assert_eq!(result, "161");
        }
    }
}

mod part2 {
    use crate::{Operation, parse};

    pub fn solution(input: &str) -> anyhow::Result<String> {
        let (_, operations) = parse(input).map_err(|err| err.to_owned())?;
        let mut sum = 0;
        let mut is_enabled = true;
        for operation in operations {
            match operation {
                Operation::Mul(a, b) =>
                    if is_enabled {
                        sum += a * b
                    },
                Operation::Do => is_enabled = true,
                Operation::Dont => is_enabled = false,
            }
        }
        Ok(sum.to_string())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn part2_example_test() {
            // given
            let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

            // when
            let result = solution(input).unwrap();

            // then
            assert_eq!(result, "48");
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Operation {
    Mul(u64, u64),
    Do,
    Dont,
}

fn parse(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, maybe_operations) = many0(alt((
        parse_mul,
        map(tag("do()"), |_| Some(Operation::Do)),
        map(tag("don't()"), |_| Some(Operation::Dont)),
        map(anychar, |_| None))))(input)?;
    Ok((input, maybe_operations.into_iter().flatten().collect::<Vec<_>>()))
}

fn parse_mul(input: &str) -> IResult<&str, Option<Operation>> {
    let (input, (_, a, _, b, _)) = (tag("mul("), complete::u64, tag(","), complete::u64, tag(")")).parse(input)?;
    Ok((input, Some(Operation::Mul(a, b))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_part1_test() {
        // given
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)";

        // when
        let (_, operation) = parse(input).unwrap();

        // then
        assert_eq!(operation, vec![Operation::Mul(2, 4), Operation::Mul(5, 5)]);
    }


    #[test]
    fn parse_part2_test() {
        // given
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        // when
        let (_, operation) = parse(input).unwrap();

        // then
        assert_eq!(operation, vec![Operation::Mul(2, 4), Operation::Dont, Operation::Mul(5, 5), Operation::Mul(11, 8), Operation::Do, Operation::Mul(8, 5)]);
    }
}
