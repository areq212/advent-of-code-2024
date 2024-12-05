use std::fs;

use nom::character::complete::{char, digit1, newline};
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::separated_list1;

type Level = i64;
type Report = Vec<Level>;

fn main() {
    let input = fs::read_to_string("day2/resources/input.txt")
        .expect("Should have been able to read the file");

    let part_1_answer = part1::solution(&input)
        .expect("Failed to solve part 1.");
    println!("Day 2 Part 1 answer: {}", part_1_answer);

    let part_2_answer = part2::solution(&input)
        .expect("Failed to solve part 2.");
    println!("Day 2 Part 2 answer: {}", part_2_answer);
}

mod part1 {
    use crate::{Change, Level, parse};

    pub fn solution(input: &str) -> anyhow::Result<String> {
        let (_, reports) = parse(input).map_err(|err| err.to_owned())?;
        let count_valid_reports = reports.into_iter().filter(|r| is_report_valid(r)).count();

        Ok(count_valid_reports.to_string())
    }

    fn is_report_valid(report: impl AsRef<[Level]>) -> bool {
        let mut level_change_type: Option<Change> = None;
        let mut last_level: Option<Level> = None;

        for level in report.as_ref() {
            if let Some(last_level) = last_level {
                let change: Change = (last_level - level).into();
                if !change.has_valid_rate() {
                    return false;
                }
                match level_change_type {
                    None => level_change_type = Some(change),
                    Some(ref last_change) => {
                        if !change.has_same_direction(last_change) {
                            return false;
                        }
                    }
                }
            }
            last_level = Some(*level);
        }
        return true;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn part1_example_test() {
            // given
            let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
            // when
            let result = solution(input).unwrap();

            // then
            assert_eq!("2", result);
        }

        #[test]
        fn is_report_valid_tet() {
            // given
            let report = vec![7, 6, 4, 2, 1];
            // when
            let valid = is_report_valid(report);

            // then
            assert!(valid)
        }
    }
}

mod part2 {
    use crate::{Change, Level, parse};

    pub fn solution(input: &str) -> anyhow::Result<String> {
        let (_, reports) = parse(input).map_err(|err| err.to_owned())?;
        let count_valid_reports = reports.into_iter()
            .filter(|r| is_report_valid_with_dumper(r)).count();

        Ok(count_valid_reports.to_string())
    }

    fn is_report_valid_with_dumper(report: impl AsRef<[Level]>) -> bool {
        for (idx, _) in report.as_ref().iter().enumerate() {
            let report = report.as_ref();
            if is_report_valid(&[&report[..idx], &report[idx + 1..]].concat()) {
                return true;
            }
        }
        return false;
    }

    fn is_report_valid(report: impl AsRef<[Level]>) -> bool {
        let mut level_change_type: Option<Change> = None;
        let mut last_level: Option<Level> = None;

        for level in report.as_ref() {
            if let Some(last_level) = last_level {
                let change: Change = (last_level - level).into();
                if !change.has_valid_rate() {
                    return false;
                }
                match level_change_type {
                    None => level_change_type = Some(change),
                    Some(ref last_change) => {
                        if !change.has_same_direction(last_change) {
                            return false;
                        }
                    }
                }
            }
            last_level = Some(*level);
        }
        return true;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn part2_example_test() {
            // given
            let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
            // when
            let result = solution(input).unwrap();

            // then
            assert_eq!("4", result);
        }

        #[test]
        fn is_report_valid_tet() {
            // given
            let report = vec![7, 6, 4, 2, 1];
            // when
            let valid = is_report_valid(&report);

            // then
            assert!(valid)
        }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Report>> {
    separated_list1(newline, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Report> {
    separated_list1(char(' '), map_res(digit1, str::parse))(input)
}


#[derive(Debug, Clone)]
enum Change {
    Increasing { rate: usize },
    Decreasing { rate: usize },
    Stagnant,
}

impl Change {
    fn has_valid_rate(&self) -> bool {
        match self {
            Change::Increasing { rate } |
            Change::Decreasing { rate } => rate > &0 && rate <= &3,
            Change::Stagnant => false,
        }
    }

    fn has_same_direction(&self, other: &Self) -> bool {
        match self {
            Change::Increasing { .. } => match other {
                Change::Increasing { .. } => true,
                _ => false
            },
            Change::Decreasing { .. } => match other {
                Change::Decreasing { .. } => true,
                _ => false
            }
            Change::Stagnant => match other {
                Change::Stagnant => true,
                _ => false
            }
        }
    }
}

impl From<Level> for Change {
    fn from(value: Level) -> Self {
        if value == 0 {
            Change::Stagnant
        } else if value > 0 {
            Change::Increasing { rate: value.abs() as usize }
        } else {
            Change::Decreasing { rate: value.abs() as usize }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_test() {
        // given
        let input = "7 6 4 2 1";

        // when
        let (_, vec) = parse_line(input).unwrap();

        // then
        assert_eq!(vec, vec![7, 6, 4, 2, 1])
    }

    #[test]
    fn parse_test() {
        // given
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1";

        // when
        let (_, vec) = parse(input).unwrap();

        // then
        assert_eq!(vec, vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
        ])
    }
}