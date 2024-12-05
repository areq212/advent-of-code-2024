use std::fs;

use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::newline;
use nom::multi::{many0, separated_list1};
use nom::sequence::Tuple;

fn main() {
    let input = fs::read_to_string("day5/resources/input.txt")
        .expect("Should have been able to read the file");

    let part_1_answer = part1::solution(&input)
        .expect("Failed to solve part 1.");
    println!("Day 5 Part 1 answer: {}", part_1_answer);

    let part_2_answer = part2::solution(&input)
        .expect("Failed to solve part 2.");
    println!("Day 5 Part 2 answer: {}", part_2_answer);
}

mod part1 {
    use crate::{PageNumber, parse, PrintUpdate, Rule};

    pub fn solution(input: &str) -> anyhow::Result<String> {
        let (_, (rules, print_updates)) = parse(input).map_err(|err| err.to_owned())?;
        let sum: PageNumber = print_updates.into_iter().filter_map(|p| verify_print_update(&rules, p)).sum();
        Ok(sum.to_string())
    }

    fn verify_print_update(rules: &[Rule], print_update: PrintUpdate) -> Option<PageNumber> {
        let pages_combination: Vec<_> = print_update.pages
            .iter().enumerate()
            .flat_map(|(i, &p)| print_update.pages[i + 1..].iter()
                .map(move |&op| (p, op))
            )
            .collect();
        let matches = pages_combination.iter().all(|(left, right)| rules.iter().all(|r| r.apply(left, right).unwrap_or(true)));

        if matches {
            Some(print_update.pages[print_update.pages.len() / 2])
        } else {
            None
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn verify_print_update_test() {
            // given
            let rules = vec![
                Rule(75, 47), Rule(75, 61), Rule(75, 53), Rule(75, 29),
                Rule(47, 61), Rule(47, 53), Rule(47, 29),
                Rule(61, 53), Rule(61, 29),
                Rule(53, 29),
            ];
            let print_uprate = PrintUpdate { pages: vec![75, 47, 61, 53, 29] };

            // when
            let result = verify_print_update(&rules, print_uprate);

            // then
            assert_eq!(result, Some(61));
        }

        #[test]
        fn part1_example_test() {
            // given
            let input = "47|53
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
97,13,75,29,47";
            // when
            let solution = solution(input).unwrap();

            // then
            assert_eq!(solution, "143".to_owned());
        }
    }
}

mod part2 {
    use crate::{PageNumber, parse, PrintUpdate, Rule};

    pub fn solution(input: &str) -> anyhow::Result<String> {
        let (_, (rules, print_updates)) = parse(input).map_err(|err| err.to_owned())?;
        let sum: PageNumber = print_updates.into_iter().filter_map(|p| fix_print_update(&rules, p)).sum();
        Ok(sum.to_string())
    }

    fn fix_print_update(rules: &[Rule], mut print_update: PrintUpdate) -> Option<PageNumber> {
        let mut is_first = true;
        loop {
            let pages_combination: Vec<_> = print_update.pages
                .iter().enumerate()
                .flat_map(|(i, &p)| print_update.pages[i + 1..].iter().enumerate()
                    .map(move |(j, &op)| ((i, p), (i + 1 + j, op)))
                )
                .collect();
            let matches: Vec<_> = pages_combination.iter()
                .filter(|((_, left), (_, right))| !rules.iter().all(|r| r.apply(left, right).unwrap_or(true)))
                .collect();
            if matches.is_empty() {
                return if is_first {
                    None
                } else {
                    Some(print_update.pages[print_update.pages.len() / 2])
                }
            }
            is_first = false;
            let [((i, _), (j, _)), ..] = matches.as_slice() else { unreachable!() };
            print_update.pages.swap(*i, *j);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn fix_print_update_test() {
            // given
            let rules = vec![
                Rule(97, 75),
                Rule(75, 47), Rule(75, 61), Rule(75, 53), Rule(75, 29),
                Rule(47, 61), Rule(47, 53), Rule(47, 29),
                Rule(61, 53), Rule(61, 29),
                Rule(53, 29),
            ];
            let print_uprate = PrintUpdate { pages: vec![75, 97, 47, 61, 53] };

            // when
            let result = fix_print_update(&rules, print_uprate);

            // then
            assert_eq!(result, Some(47));
        }

        #[test]
        fn part2_example_test() {
            // given
            let input = "47|53
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
97,13,75,29,47";
            // when
            let solution = solution(input).unwrap();

            // then
            assert_eq!(solution, "123".to_owned());
        }
    }
}

type PageNumber = u64;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Rule(PageNumber, PageNumber);

impl Rule {
    fn apply(&self, left: &PageNumber, right: &PageNumber) -> Option<bool> {
        return if *left == self.0 && *right == self.1 {
            Some(true)
        } else if *left == self.1 && *right == self.0 {
            Some(false)
        } else {
            None
        };
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct PrintUpdate {
    pages: Vec<PageNumber>,
}

fn parse(input: &str) -> IResult<&str, (Vec<Rule>, Vec<PrintUpdate>)> {
    let (input, rules) = separated_list1(newline, parse_rule)(input)?;
    let (input, _) = many0(newline)(input)?;
    let (input, print_updates) = separated_list1(newline, parse_page_update)(input)?;
    Ok((input, (rules, print_updates)))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, (left, _, right)) = (complete::u64, tag("|"), complete::u64).parse(input)?;
    Ok((input, Rule(left, right)))
}

fn parse_page_update(input: &str) -> IResult<&str, PrintUpdate> {
    let (input, pages) = separated_list1(tag(","), complete::u64)(input)?;
    Ok((input, PrintUpdate { pages }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rule_test() {
        // given
        let input = "47|53";

        // when
        let (input, rule) = parse_rule(input).unwrap();

        // then
        assert!(input.is_empty());
        assert_eq!(rule, Rule(47, 53));
    }

    #[test]
    fn parse_page_update_test() {
        // given
        let input = "75,47,61,53,29";

        // when
        let (input, rule) = parse_page_update(input).unwrap();

        // then
        assert!(input.is_empty());
        assert_eq!(rule, PrintUpdate { pages: vec![75, 47, 61, 53, 29] });
    }

    #[test]
    fn parse_test() {
        // given
        let input = "47|53
97|13
53|13

75,47,61,53,29
61,13,29
97,13,75,29,47";

        // when
        let (input, (rules, print_updates)) = parse(input).unwrap();

        // then
        assert!(input.is_empty());
        assert_eq!(rules, vec![
            Rule(47, 53),
            Rule(97, 13),
            Rule(53, 13)]
        );
        assert_eq!(print_updates, vec![
            PrintUpdate { pages: vec![75, 47, 61, 53, 29] },
            PrintUpdate { pages: vec![61, 13, 29] },
            PrintUpdate { pages: vec![97, 13, 75, 29, 47] }]
        );
    }
}