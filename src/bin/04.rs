advent_of_code::solution!(4);

type Point = (usize, usize);

pub struct Puzzle {
    puzzle: Vec<Vec<char>>,
}

impl Puzzle {
    fn new(input: &str) -> Self {
        let puzzle: Vec<Vec<char>> = input
            .split_whitespace()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        Puzzle { puzzle }
    }

    fn rows(&self) -> usize {
        self.puzzle.len()
    }

    fn cols(&self) -> usize {
        self.puzzle[0].len()
    }

    fn get(&self, (x, y): Point) -> Option<char> {
        if x >= self.cols() || y >= self.rows() {
            None
        } else {
            Some(self.puzzle[y][x])
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let word: Vec<char> = "XMAS".chars().collect();
    let puzzle = Puzzle::new(input);

    let mut count: u32 = 0;

    for y in 0..puzzle.rows() {
        for x in 0..puzzle.cols() {
            // east
            if has_word(&word, &puzzle, |i| (x + i, y)) {
                count += 1;
            }

            // southeast
            if has_word(&word, &puzzle, |i| (x + i, y + i)) {
                count += 1;
            }

            // south
            if has_word(&word, &puzzle, |i| (x, y + i)) {
                count += 1;
            }

            // southwest
            if has_word(&word, &puzzle, |i| (x.wrapping_sub(i), y + i)) {
                count += 1;
            }

            // west
            if has_word(&word, &puzzle, |i| (x.wrapping_sub(i), y)) {
                count += 1;
            }

            // northwest
            if has_word(&word, &puzzle, |i| (x.wrapping_sub(i), y.wrapping_sub(i))) {
                count += 1;
            }

            // north
            if has_word(&word, &puzzle, |i| (x, y.wrapping_sub(i))) {
                count += 1;
            }

            // northeast
            if has_word(&word, &puzzle, |i| (x + i, y.wrapping_sub(i))) {
                count += 1;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let word: Vec<char> = "MAS".chars().collect();
    let puzzle = Puzzle::new(input);

    let mut count: u32 = 0;

    for y in 0..puzzle.rows() {
        for x in 0..puzzle.cols() {
            // NW -> SE
            let (nw_x, nw_y) = (x.wrapping_sub(1), y.wrapping_sub(1));
            let nw_se_valid = has_word(&word, &puzzle, |i| (nw_x + i, nw_y + i));

            // SE -> NW
            let (se_x, se_y) = (x + 1, y + 1);
            let se_nw_valid = has_word(&word, &puzzle, |i| {
                (se_x.wrapping_sub(i), se_y.wrapping_sub(i))
            });

            // NE -> SW
            let (ne_x, ne_y) = (x + 1, y.wrapping_sub(1));
            let ne_sw_valid = has_word(&word, &puzzle, |i| (ne_x.wrapping_sub(i), ne_y + i));

            // SW -> NE
            let (sw_x, sw_y) = (x.wrapping_sub(1), y + 1);
            let sw_ne_valid = has_word(&word, &puzzle, |i| (sw_x + i, sw_y.wrapping_sub(i)));

            if (nw_se_valid || se_nw_valid) && (ne_sw_valid || sw_ne_valid) {
                count += 1;
            }
        }
    }

    Some(count)
}

fn has_word<Func>(word: &Vec<char>, puzzle: &Puzzle, get_point_at: Func) -> bool
where
    Func: Fn(usize) -> Point,
{
    (0..word.len()).all(|i| {
        let value = puzzle.get(get_point_at(i));
        value.is_some_and(|c| c == word[i])
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
