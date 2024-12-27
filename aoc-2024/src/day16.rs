use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::utils::{Coord, Direction};
use crate::Solution;

pub struct Day16;

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    pos: Coord,
    dir: Direction,
    cost: i32,
    path: Vec<Coord>,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Grid {
    cells: Vec<Vec<char>>,
}

impl Grid {
    fn from(input: &str) -> Self {
        Self {
            cells: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    fn get(&self, pos: &Coord) -> char {
        self.cells[pos.y as usize][pos.x as usize]
    }

    fn is_wall(&self, pos: &Coord) -> bool {
        self.get(pos) == '#'
    }
    fn find_char(&self, target: char) -> Option<Coord> {
        for y in 0..self.cells.len() {
            for x in 0..self.cells[0].len() {
                if self.cells[y][x] == target {
                    return Some(Coord {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }
        None
    }

    fn is_valid(&self, pos: &Coord) -> bool {
        pos.y >= 0
            && pos.x >= 0
            && (pos.y as usize) < self.cells.len()
            && (pos.x as usize) < self.cells[0].len()
    }

    fn find_optimal_path_nodes(&self) -> (usize, usize) {
        let start = self.find_char('S').expect("No start position found");
        let end = self.find_char('E').expect("No end position found");

        let mut heap = BinaryHeap::new();
        let mut costs = HashMap::new();
        let mut optimal_paths = Vec::new();
        let mut min_cost = i32::MAX;

        let initial = State {
            pos: start,
            dir: Direction::Right,
            cost: 0,
            path: vec![start],
        };

        heap.push(initial);
        costs.insert((start, Direction::Right), 0);

        while let Some(State {
            pos,
            dir,
            cost,
            path,
        }) = heap.pop()
        {
            if cost > min_cost {
                break; // All optimal paths found
            }

            if pos == end {
                if cost < min_cost {
                    min_cost = cost;
                    optimal_paths.clear();
                }
                if cost == min_cost {
                    optimal_paths.push(path);
                }
                continue;
            }

            for new_dir in &[
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ] {
                let new_pos = pos + new_dir.to_coord();

                if !self.is_valid(&new_pos) || self.is_wall(&new_pos) {
                    continue;
                }

                let turn_cost = if *new_dir == dir { 0 } else { 1000 };
                let move_cost = 1;
                let new_cost = cost + turn_cost + move_cost;

                if new_cost > min_cost {
                    continue;
                }

                let better = costs
                    .get(&(new_pos, *new_dir))
                    .map_or(true, |&c| new_cost <= c);

                if better {
                    costs.insert((new_pos, *new_dir), new_cost);
                    let mut new_path = path.clone();
                    new_path.push(new_pos);
                    heap.push(State {
                        pos: new_pos,
                        dir: *new_dir,
                        cost: new_cost,
                        path: new_path,
                    });
                }
            }
        }

        let unique_nodes: HashSet<_> = optimal_paths
            .into_iter()
            .flat_map(|path| path.into_iter())
            .filter(|pos| self.get(pos) == '.')
            .collect();

        (min_cost as usize, unique_nodes.len() + 2)
    }
}

impl Solution for Day16 {
    fn part1(&self, input: &str) -> String {
        Grid::from(input).find_optimal_path_nodes().0.to_string()
    }

    fn part2(&self, input: &str) -> String {
        Grid::from(input).find_optimal_path_nodes().1.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_a() {
        let input: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        let result = Day16.part1(input);
        assert_eq!(result, 7036.to_string());
    }

    #[test]
    fn test_part1_b() {
        let input: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        let result = Day16.part1(input);
        assert_eq!(result, 11048.to_string());
    }

    #[test]
    fn test_part2_a() {
        let input: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        let result = Day16.part2(input);
        assert_eq!(result, 45.to_string());
    }

    #[test]
    fn test_part2_b() {
        let input: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        let result = Day16.part2(input);
        assert_eq!(result, 64.to_string());
    }
}
