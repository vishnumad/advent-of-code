use std::collections::HashSet;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::new(input);

    while !map.completed {
        map.tick();
    }

    Some(map.visited_positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // TODO: Is there a more efficient way to do this??

    let mut map = Map::new(input);

    fn has_cycle(map: &mut Map, obstacle_pos: Position) -> bool {
        map.reset();
        map.set_obstacle_pos(obstacle_pos);

        while !map.completed {
            map.tick();
        }

        map.has_cycle
    }

    let mut count: u32 = 0;
    for y in 0..map.char_map.len() {
        for x in 0..map.char_map[y].len() {
            let c = map.char_map[y][x];
            if c != '.' {
                continue;
            }

            if has_cycle(&mut map, (x, y)) {
                count += 1;
            }
        }
    }

    Some(count)
}

type Position = (usize, usize);
type PositionVector = (usize, usize, u32); // x, y, direction

pub struct Map {
    char_map: Vec<Vec<char>>,
    visited_positions: HashSet<Position>,
    visited_position_vectors: HashSet<PositionVector>,
    guard_direction: u32,
    guard_position: Position,
    initial_guard_position: Position,
    obstacle_position: Position,
    has_cycle: bool,
    completed: bool,
}

impl Map {
    fn new(input: &str) -> Self {
        let char_map: Vec<Vec<char>> = input
            .split_whitespace()
            .map(|line| line.chars().collect())
            .collect();

        let mut initial_guard_position = (0, 0);
        for y in 0..char_map.len() {
            for x in 0..char_map[y].len() {
                match char_map[y][x] {
                    '^' => initial_guard_position = (x, y),
                    _ => {}
                }
            }
        }

        let mut map = Map {
            char_map,
            visited_positions: HashSet::new(),
            visited_position_vectors: HashSet::new(),
            guard_direction: 0,
            guard_position: (0, 0),
            obstacle_position: (0, 0),
            initial_guard_position,
            has_cycle: false,
            completed: false,
        };
        map.reset();

        map
    }

    fn tick(&mut self) {
        fn get_next_position(direction: u32, (x, y): Position) -> (usize, usize) {
            match direction {
                0 => (x, y.wrapping_sub(1)),
                90 => (x + 1, y),
                180 => (x, y + 1),
                270 => (x.wrapping_sub(1), y),
                _ => unreachable!("invalid direction"),
            }
        }

        // update guard position
        let (guard_x, guard_y) = self.guard_position;
        self.visited_positions.insert(self.guard_position);
        self.visited_position_vectors
            .insert((guard_x, guard_y, self.guard_direction));

        let (next_x, next_y) = get_next_position(self.guard_direction, self.guard_position);

        // check if next pos in bounds
        if next_x >= self.char_map[0].len() || next_y >= self.char_map.len() {
            // completed
            self.completed = true;
            return;
        }

        let next_val = self.char_map[next_y][next_x];
        if next_val == '#' || (next_x, next_y) == self.obstacle_position {
            // obstacle, rotate the guard
            self.guard_direction = (self.guard_direction + 90) % 360;
            return;
        }

        // check for cycles
        if self
            .visited_position_vectors
            .contains(&(next_x, next_y, self.guard_direction))
        {
            self.has_cycle = true;
            self.completed = true;
            return;
        }

        // empty space, move the guard
        self.guard_position = (next_x, next_y);
    }

    fn set_obstacle_pos(&mut self, position: Position) {
        self.obstacle_position = position;
    }

    fn reset(&mut self) {
        self.guard_position = self.initial_guard_position;
        self.obstacle_position = (0, 0);
        self.guard_direction = 0;
        self.visited_positions.clear();
        self.visited_position_vectors.clear();
        self.has_cycle = false;
        self.completed = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
