use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let matcher = Regex::new(r"mul\(([0-9]*),([0-9]*)\)").unwrap();

    let sum: u32 = matcher
        .captures_iter(input)
        .map(|capture| {
            let (_, [x, y]) = capture.extract();
            let x: u32 = x.parse().unwrap();
            let y: u32 = y.parse().unwrap();

            x * y
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let matcher = Regex::new(r"do\(\)|don't\(\)|mul\(([0-9]*),([0-9]*)\)").unwrap();

    let mut sum: u32 = 0;
    let mut enabled = true;

    matcher.captures_iter(input).for_each(|capture| {
        let matched = capture.get(0).unwrap().as_str();
        match matched {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                // mul(X,Y)
                if enabled {
                    let x = capture.get(1).unwrap().as_str();
                    let y = capture.get(2).unwrap().as_str();

                    let x: u32 = x.parse().unwrap();
                    let y: u32 = y.parse().unwrap();

                    sum += x * y;
                }
            }
        }
    });

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
