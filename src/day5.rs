use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use nom::{character::complete, multi::separated_list1, sequence::separated_pair};

use crate::{parse_input, parse_number};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Rule(usize, usize);

pub fn part_1(input: &str) -> usize {
    let mut lines = input.lines();
    let rules = parse_rules(&mut lines);
    let pages = parse_pages(lines);
    let rule_set: HashSet<&Rule> = rules.iter().collect();
    pages
        .iter()
        .filter(|page| page_is_ordered(&rule_set, page))
        .map(|page| page[page.len() / 2])
        .sum()
}

/// To correct the unordered pages, sort the page numbers by the number of rules
/// that apply to the number. The intuition being that the first number must not
/// appear on the left side of any rules that apply to the page, else it wouldn't
/// be first because some other number must then come before. The second number must only
/// have one rule apply because only one number is in front of it and so on.
pub fn part_2(input: &str) -> usize {
    let mut lines = input.lines();
    let rules = parse_rules(&mut lines);
    let pages = parse_pages(lines);
    let rule_set: HashSet<&Rule> = rules.iter().collect();

    pages
        .into_iter()
        .filter(|page| !page_is_ordered(&rule_set, page))
        .map(|mut page| {
            let filtered_rules = rules
                .iter()
                .filter(|rule| page.contains(&rule.0) && page.contains(&rule.1));
            let compressed_rules = compress_rules(filtered_rules);
            let rule_counts = rules_applied_per_number(&page, &compressed_rules);
            page.sort_by_key(|&page| rule_counts[&page]);
            page[page.len() / 2]
        })
        .sum()
}

fn page_is_ordered(rule_set: &HashSet<&Rule>, page: &[usize]) -> bool {
    page.windows(2)
        .all(|window| rule_set.contains(&Rule(window[0], window[1])))
}

/// For each number, get the set of numbers that must come after it
fn compress_rules<'a>(
    rules: impl Iterator<Item = &'a Rule>,
) -> HashMap<&'a usize, HashSet<&'a usize>> {
    rules.fold(HashMap::new(), |mut map, Rule(left, right)| {
        let entry = map.entry(left).or_default();
        entry.insert(right);
        map
    })
}

/// For each number in the `page`, check how many rules apply to it.
/// This could also be thought of as how many numbers must come before it.
fn rules_applied_per_number(
    page: &[usize],
    rules: &HashMap<&usize, HashSet<&usize>>,
) -> HashMap<usize, usize> {
    let rule_sets = rules.values().collect::<Vec<_>>();
    page.iter()
        .fold(HashMap::<usize, usize>::new(), |mut map, &number| {
            let applicable_rule_count = rule_sets
                .iter()
                .filter(|rule| rule.contains(&number))
                .count();
            let entry = map.entry(number).or_insert(0);
            *entry += applicable_rule_count;
            map
        })
}

fn parse_rules<'a>(input: &mut impl Iterator<Item = &'a str>) -> Vec<Rule> {
    input
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (left, right) = parse_input(
                line,
                separated_pair(parse_number, complete::char('|'), parse_number),
            );
            Rule(left, right)
        })
        .collect()
}

fn parse_pages<'a>(input: impl Iterator<Item = &'a str>) -> Vec<Vec<usize>> {
    input
        .map(|line| parse_input(line, separated_list1(complete::char(','), parse_number)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        let input = read_input(5);
        assert_eq!(part_1(&input), 6041);
    }

    #[test]
    fn test_part_2() {
        let input = read_input(5);
        assert_eq!(part_2(&input), 4884);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = read_input(5);
        b.iter(|| part_1(&input));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = read_input(5);
        b.iter(|| part_2(&input));
    }

    #[test]
    fn test_parse_rules() {
        let input = "1|2\n1|3\n2|3";
        let rules = parse_rules(&mut input.lines());
        assert_eq!(rules, vec![Rule(1, 2), Rule(1, 3), Rule(2, 3)]);
    }

    #[test]
    fn test_parse_pages() {
        let input = "1,2,3\n4,5,6";
        let pages = parse_pages(input.lines());
        assert_eq!(pages, vec![vec![1, 2, 3], vec![4, 5, 6]]);
    }

    #[test]
    fn test_compress_rules() {
        let rules = [
            Rule(1, 2),
            Rule(1, 3),
            Rule(1, 4),
            Rule(2, 3),
            Rule(2, 4),
            Rule(3, 4),
        ];
        let compressed = compress_rules(rules.iter());
        assert_eq!(
            compressed,
            HashMap::from([
                (&1, HashSet::from([&2, &3, &4])),
                (&2, HashSet::from([&3, &4])),
                (&3, HashSet::from([&4]))
            ])
        );
    }

    #[test]
    fn test_rules_applied_per_number() {
        let rules = HashMap::from([
            (&1, HashSet::from([&2, &3, &4])),
            (&2, HashSet::from([&3, &4])),
            (&3, HashSet::from([&4])),
        ]);
        let rule_counts = rules_applied_per_number(&[1, 2, 3, 4], &rules);
        assert_eq!(rule_counts, HashMap::from([(1, 0), (2, 1), (3, 2), (4, 3)]));
    }
}
