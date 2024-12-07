use std::fs;

use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::newline;
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::Tuple;

fn main() {
    let input = fs::read_to_string("day7/resources/input.txt")
        .expect("Should have been able to read the file");

    let part_1_answer = part1::solution(&input)
        .expect("Failed to solve part 1.");
    println!("Day 7 Part 1 answer: {}", part_1_answer);

    let part_2_answer = part2::solution(&input)
        .expect("Failed to solve part 2.");
    println!("Day 7 Part 2 answer: {}", part_2_answer);
}

mod part1 {
    use crate::{Operation, parse};

    pub fn solution(input: &str) -> anyhow::Result<String> {
        let (_, equations) = parse(input).map_err(|err| err.to_owned())?;
        let sum: u64 = equations.iter().filter(|e| calculate(&e.operands).contains(&e.expected_result)).map(|e| e.expected_result).sum();
        Ok(sum.to_string())
    }
    fn calculate(operands: &[u64]) -> Vec<u64> {
        match operands {
            [] => vec![],
            [first] => vec![*first],
            [first, rest@ .. ] => vec![calculate_rec(*first, rest, Operation::Add), calculate_rec(*first, rest, Operation::Mul)].into_iter().flatten().collect()
        }
    }

    fn calculate_rec(prev_result: u64, operands: &[u64], operation: Operation) -> Vec<u64> {
        match operands {
            [] => vec![prev_result],
            [first] =>
            match operation {
                Operation::Add => {vec![prev_result + first]}
                Operation::Mul => {vec![prev_result * first]}
                _ => unreachable!()
            }
            [first, rest@ .. ] => match operation {
                Operation::Add => vec![
                    calculate_rec(prev_result + first, rest, Operation::Add),
                    calculate_rec(prev_result + first, rest, Operation::Mul),
                ].into_iter().flatten().collect(),
                Operation::Mul => vec![
                    calculate_rec(prev_result * first, rest, Operation::Add),
                    calculate_rec(prev_result * first, rest, Operation::Mul),
                ].into_iter().flatten().collect(),
                _ => unreachable!()
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn calculate_test() {
            // given
            let operands = vec![10, 19];

            // when

            let vec = calculate(&operands);

            // then
            dbg!(vec);
        }

        #[test]
        fn part1_example_test() {
            // given
            let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

            // when
            let result = solution(input).unwrap();

            // then
            assert_eq!(result, "3749");
        }
    }
}

mod part2 {
    use crate::{Operation, parse};

    pub fn solution(input: &str) -> anyhow::Result<String> {
        let (_, equations) = parse(input).map_err(|err| err.to_owned())?;
        let sum: u64 = equations.iter().filter(|e| calculate(&e.operands).contains(&e.expected_result)).map(|e| e.expected_result).sum();
        Ok(sum.to_string())
    }
    fn calculate(operands: &[u64]) -> Vec<u64> {
        match operands {
            [] => vec![],
            [first] => vec![*first],
            [first, rest@ .. ] => vec![
                calculate_rec(*first, rest, Operation::Add),
                calculate_rec(*first, rest, Operation::Mul),
                calculate_rec(*first, rest, Operation::Concat),
            ].into_iter().flatten().collect()
        }
    }

    fn calculate_rec(prev_result: u64, operands: &[u64], operation: Operation) -> Vec<u64> {
        match operands {
            [] => vec![prev_result],
            [first] =>
                match operation {
                    Operation::Add => {vec![prev_result + first]}
                    Operation::Mul => {vec![prev_result * first]}
                    Operation::Concat => {
                        let string = format!("{}{}", prev_result.to_string(), first.to_string());
                        vec![string.parse::<u64>().expect("failed to parse {string} to u64") ]}
                }
            [first, rest@ .. ] => match operation {
                Operation::Add => vec![
                    calculate_rec(prev_result + first, rest, Operation::Add),
                    calculate_rec(prev_result + first, rest, Operation::Mul),
                    calculate_rec(prev_result + first, rest, Operation::Concat),
                ].into_iter().flatten().collect(),
                Operation::Mul => vec![
                    calculate_rec(prev_result * first, rest, Operation::Add),
                    calculate_rec(prev_result * first, rest, Operation::Mul),
                    calculate_rec(prev_result * first, rest, Operation::Concat),
                ].into_iter().flatten().collect(),
                Operation::Concat => {
                    let string = format!("{}{}", prev_result.to_string(), first.to_string());
                    let concat = string.parse::<u64>().expect("failed to parse {string} to u64");
                    vec![
                        calculate_rec(concat, rest, Operation::Add),
                        calculate_rec(concat, rest, Operation::Mul),
                        calculate_rec(concat, rest, Operation::Concat),
                    ].into_iter().flatten().collect()
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn part2_example_test() {
            // given
            let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

            // when
            let result = solution(input).unwrap();

            // then
            assert_eq!(result, "11387");
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add, Mul, Concat
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Equation {
    expected_result: u64,
    operands: Vec<u64>,
}

fn parse(input: &str) -> IResult<&str, Vec<Equation>> {
    separated_list1(newline, parse_equation)(input)
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    let (input, (expected_result, _, operands)) = (complete::u64, tag(":"), many1(parse_operand)).parse(input)?;
    return Ok((input, Equation { expected_result, operands }));
}

fn parse_operand(input: &str) -> IResult<&str, u64> {
    let (input, (_, a)) = (tag(" "), complete::u64).parse(input)?;
    Ok((input, a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        // given
        let input = "190: 10 19
3267: 81 40 27";

        // when
        let (_, equations) = parse(input).unwrap();

        // then
        assert_eq!(equations, vec![
            Equation {expected_result: 190, operands: vec![10, 19]},
            Equation {expected_result: 3267, operands: vec![81, 40, 27]},
        ]);
    }

}
