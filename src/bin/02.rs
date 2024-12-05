use std::ops::Not;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    let input_lines = input.trim().split('\n');

    let safe_report_count = input_lines.map(parse_report).filter(is_safe).count();
    Some(safe_report_count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input_lines = input.trim().split('\n');

    let safe_report_count = input_lines
        .map(parse_report)
        .filter(|report| {
            if is_safe(report) {
                true
            } else {
                // check if report is safe after removing any one level
                (0..report.len()).any(|skip_idx| {
                    let mut subreport = report.clone();
                    subreport.remove(skip_idx);

                    is_safe(&subreport)
                })
            }
        })
        .count();

    Some(safe_report_count)
}

fn is_safe(levels: &Vec<i32>) -> bool {
    // +1 increasing, -1 decreasing, 0 same
    let report_signum = (levels[1] - levels[0]).signum();
    if report_signum == 0 {
        return false;
    }

    let is_unsafe = levels
        .windows(2)
        .map(|level_window| level_window[1] - level_window[0])
        .any(|delta| delta.signum() != report_signum || delta == 0 || delta.abs() > 3);

    is_unsafe.not()
}

fn parse_report(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
