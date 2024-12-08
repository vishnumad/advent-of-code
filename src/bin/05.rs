use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules_input, updates_input) = input.split_once("\n\n").unwrap();
    let rules = parse_rules(rules_input);
    let updates = parse_updates(updates_input);

    let sum = updates
        .iter()
        .filter(|update| is_ordered(&rules, update))
        .map(|update| update[update.len() / 2])
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules_input, updates_input) = input.split_once("\n\n").unwrap();
    let rules = parse_rules(rules_input);
    let updates = parse_updates(updates_input);

    let sum: u32 = updates
        .iter()
        .filter(|update| !is_ordered(&rules, update))
        .map(|update| order(&rules, update))
        .map(|update| update[update.len() / 2])
        .sum();

    Some(sum)
}

fn is_ordered(rules: &HashMap<u32, Vec<u32>>, pages: &Vec<u32>) -> bool {
    let mut seen_pages: Vec<u32> = Vec::new();

    pages.iter().all(|page| {
        seen_pages.push(*page);

        if let Some(page_rule) = rules.get(page) {
            let is_invalid = page_rule.iter().any(|rule| seen_pages.contains(rule));
            !is_invalid
        } else {
            // no rule, so valid
            true
        }
    })
}

fn order(rules: &HashMap<u32, Vec<u32>>, pages: &Vec<u32>) -> Vec<u32> {
    let mut copy: Vec<u32> = pages.clone();

    copy.sort_by(|a, b| {
        if rules.get(a).is_some_and(|rules| rules.contains(b)) {
            // A < B
            return Ordering::Less;
        }

        if rules.get(b).is_some_and(|rules| rules.contains(a)) {
            // A > B
            return Ordering::Greater;
        }

        Ordering::Equal
    });

    copy
}

fn parse_rules(rules_input: &str) -> HashMap<u32, Vec<u32>> {
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    rules_input
        .split_whitespace()
        .map(|line| line.split_once('|').unwrap())
        .for_each(|(key, value)| {
            let key: u32 = key.parse().unwrap();
            let value: u32 = value.parse().unwrap();
            rules.entry(key).or_insert_with(Vec::new).push(value);
        });

    rules
}

fn parse_updates(updates_input: &str) -> Vec<Vec<u32>> {
    updates_input
        .split_whitespace()
        .map(|line| {
            line.split(',')
                .filter_map(|i| i.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
