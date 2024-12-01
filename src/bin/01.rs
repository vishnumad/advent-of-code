use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let input_lines = input.trim().split('\n').into_iter();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    input_lines.for_each(|line| {
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();

        left.push(parts[0]);
        right.push(parts[1]);
    });

    left.sort_unstable();
    right.sort_unstable();

    let sum = left
        .into_iter()
        .zip(right)
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let input_lines = input.trim().split('\n').into_iter();

    let mut left: Vec<i32> = Vec::new();
    let mut right_count: HashMap<i32, i32> = HashMap::new();

    input_lines.for_each(|line| {
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();

        left.push(parts[0]);
        right_count
            .entry(parts[1])
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });

    let sum = left
        .into_iter()
        .map(|left_value| {
            let count = right_count.get(&left_value).unwrap_or(&0);
            left_value * count
        })
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
