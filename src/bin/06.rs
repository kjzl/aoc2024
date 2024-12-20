advent_of_code::solution!(6);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn in_map_bounds(&self, matrix: &[Vec<MapItem>]) -> bool {
        self.y < matrix.len() && self.x < matrix[0].len()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    #[default]
    Up,
    Down,
    Right,
    Left,
}

impl Dir {
    fn turn_right(&mut self) {
        *self = match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Right => Dir::Down,
            Dir::Left => Dir::Up,
        };
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GuardMoveError {
    EndOfMap,
    Loop,
}

#[derive(Debug, Clone)]
pub struct Guard {
    pos: Pos,
    dir: Dir,
    last_turns: Vec<(Pos, Dir)>,
}

impl Guard {
    fn new(pos: Pos) -> Self {
        Guard {
            pos,
            dir: Dir::Up,
            last_turns: Default::default(),
        }
    }

    fn next_pos(&self) -> Option<Pos> {
        Some(match self.dir {
            Dir::Up => Pos {
                y: self.pos.y.checked_sub(1)?,
                ..self.pos
            },
            Dir::Down => Pos {
                y: self.pos.y.checked_add(1)?,
                ..self.pos
            },
            Dir::Right => Pos {
                x: self.pos.x.checked_add(1)?,
                ..self.pos
            },
            Dir::Left => Pos {
                x: self.pos.x.checked_sub(1)?,
                ..self.pos
            },
        })
    }

    // returns false if we detect a loop
    // a loop is present, if the position of the current turning point
    // equals the position of the turning point 3 times ago
    fn turn_right(&mut self) -> bool {
        if self.last_turns.contains(&(self.pos, self.dir)) {
            return false;
        }
        self.last_turns.push((self.pos, self.dir));
        self.dir.turn_right();
        true
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    matrix: Vec<Vec<MapItem>>,
    guard: Guard,
}

impl Map {
    fn item_at_mut(&mut self, pos: Pos) -> Option<&mut MapItem> {
        match pos.in_map_bounds(&self.matrix) {
            true => Some(&mut self.matrix[pos.y][pos.x]),
            false => None,
        }
    }

    // false if we cannot move further because of end of map or loop
    fn move_guard(&mut self) -> Option<GuardMoveError> {
        let Some(next_pos) = self.guard.next_pos() else {
            return Some(GuardMoveError::EndOfMap);
        };
        let Some(next_item) = self.item_at_mut(next_pos) else {
            return Some(GuardMoveError::EndOfMap);
        };
        if *next_item == MapItem::Obstacle {
            if !self.guard.turn_right() {
                return Some(GuardMoveError::Loop);
            }
            return self.move_guard();
        }
        *next_item = MapItem::GuardPath;
        self.guard.pos = next_pos;
        None
    }

    fn count_guard_path(&self) -> usize {
        self.matrix
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|item| **item == MapItem::GuardPath)
                    .count()
            })
            .sum()
    }

    fn iter(&self) -> impl Iterator<Item = (Pos, MapItem)> + '_ {
        self.matrix.iter().enumerate().flat_map(|(y, col)| {
            col.iter()
                .enumerate()
                .map(move |(x, item)| (Pos { x, y }, *item))
        })
    }

    fn parse(inp: &str) -> Self {
        let matrix: Vec<Vec<MapItem>> = inp
            .lines()
            .map(|line| line.chars().map(MapItem::from).collect())
            .collect();
        let guard_pos = matrix
            .iter()
            .enumerate()
            .find_map(|(y, col)| {
                col.iter()
                    .position(|item| *item == MapItem::GuardPath)
                    .map(|x| Pos { x, y })
            })
            .expect("Map must contain a Guard Entity");
        Self {
            matrix,
            guard: Guard::new(guard_pos),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapItem {
    GuardPath,
    Obstacle,
    Empty,
}

impl From<char> for MapItem {
    fn from(value: char) -> Self {
        match value {
            '#' => MapItem::Obstacle,
            '.' => MapItem::Empty,
            '^' => MapItem::GuardPath,
            val => panic!("Invalid char in parsed Map: '{val}'"),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut map = Map::parse(input);
    while map.move_guard().is_none() {}
    Some(map.count_guard_path() as u64)
}

// we want to put a new obstacle in every possible empty slot in the map
pub fn part_two(input: &str) -> Option<u64> {
    let map = Map::parse(input);
    let mut walked_map = map.clone();
    while walked_map.move_guard().is_none() {}
    let possible_obstacle_positions_for_loop = walked_map
        .iter()
        .filter_map(|(pos, item)| {
            if item == MapItem::GuardPath && pos != map.guard.pos {
                Some(pos)
            } else {
                None
            }
        })
        .filter_map(|pos| {
            let mut extra_obstacle_map = map.clone();
            *extra_obstacle_map.item_at_mut(pos).unwrap() = MapItem::Obstacle;
            loop {
                match extra_obstacle_map.move_guard() {
                    Some(GuardMoveError::Loop) => return Some(()),
                    Some(GuardMoveError::EndOfMap) => return None,
                    None => (),
                }
            }
        })
        .count();
    Some(possible_obstacle_positions_for_loop as u64)
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
