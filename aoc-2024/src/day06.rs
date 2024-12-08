use std::{collections::HashSet, ops::Add};

use crate::Solution;

pub struct Day06;

#[derive(Copy, Clone, Debug)]
enum Terrain {
    Empty,
    Obstacle,
}

type Map = Vec<Vec<Terrain>>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}
impl Add for Coord {
    type Output = Coord;
    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn to_coord(&self) -> Coord {
        match self {
            Direction::North => Coord { x: 0, y: -1 },
            Direction::South => Coord { x: 0, y: 1 },
            Direction::East => Coord { x: 1, y: 0 },
            Direction::West => Coord { x: -1, y: 0 },
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

fn do_walk(map: &Map, start: Coord, dir: Direction) -> HashSet<Coord> {
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut pos = start;
    let mut dir = dir;
    visited.insert(pos);
    loop {
        // look ahead
        let ahead = pos + dir.to_coord();
        match get_element(&map, ahead) {
            Some(Terrain::Empty) => {
                // move forward
                pos = ahead;
                visited.insert(pos);
            }
            Some(Terrain::Obstacle) => {
                dir = dir.turn_right();
            }
            None => {
                break;
            }
        }
    }
    visited
}

fn is_loop(map: &Map, start: Coord, dir: Direction, extra_obstacle: Coord) -> bool {
    let mut visited_states: HashSet<(Coord, Direction)> = HashSet::new();
    let mut pos = start;
    let mut dir = dir;
    loop {
        if !visited_states.insert((pos, dir)) {
            return true;
        }

        let ahead = pos + dir.to_coord();
        if ahead == extra_obstacle {
            dir = dir.turn_right();
            continue;
        }

        if get_element(&map, ahead).is_none() {
            return false;
        }

        match get_element(&map, ahead) {
            Some(Terrain::Empty) => {
                pos = ahead;
            }
            Some(Terrain::Obstacle) => {
                dir = dir.turn_right();
            }
            None => return false,
        }
    }
}

fn get_element(map: &Map, coord: Coord) -> Option<Terrain> {
    let x = coord.x;
    let y = coord.y;
    if x < 0 || x >= map[0].len() as i32 || y < 0 || y >= map.len() as i32 {
        return None;
    }
    Some(map[y as usize][x as usize])
}

fn read_input(input: &str) -> (Map, Coord) {
    let mut guard_pos = Coord { x: 0, y: 0 };
    let mut map = Map::new();
    for (y, row) in input.lines().enumerate() {
        let mut map_row = Vec::new();
        for (x, c) in row.chars().enumerate() {
            let terrain = match c {
                '.' => Terrain::Empty,
                '^' => {
                    guard_pos.x = x as i32;
                    guard_pos.y = y as i32;
                    Terrain::Empty
                }
                _ => Terrain::Obstacle,
            };
            map_row.push(terrain);
        }
        map.push(map_row);
    }
    (map, guard_pos)
}

impl Solution for Day06 {
    fn part1(&self, input: &str) -> String {
        let (map, guard_pos) = read_input(input);
        let dir = Direction::North;
        let walk = do_walk(&map, guard_pos, dir);
        walk.len().to_string() // 5153
    }

    fn part2(&self, input: &str) -> String {
        let (map, guard_pos) = read_input(input);
        let dir = Direction::North;
        let walk = do_walk(&map, guard_pos, dir);

        let mut res = 0;
        for pos in walk {
            if pos == guard_pos || matches!(get_element(&map, pos), Some(Terrain::Obstacle)) {
                continue;
            }
            // Check if placing an obstacle here creates a loop
            if is_loop(&map, guard_pos, Direction::North, pos) {
                res += 1;
            }
        }

        res.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let result = Day06.part1(input);
        assert_eq!(result, 41.to_string());
    }

    #[test]
    fn test_part2() {
        let input: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let result = Day06.part2(input);
        assert_eq!(result, 6.to_string());
    }
}
