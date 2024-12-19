use crate::{utils::Coord, utils::Direction, Solution};
use std::collections::HashSet;

pub struct Day15;

struct Grid {
    cells: Vec<Vec<char>>,
}

impl Grid {
    fn from(input: &str) -> Self {
        Self {
            cells: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    fn from_scaled_up(input: &str) -> Self {
        Self {
            cells: input
                .lines()
                .map(|line| {
                    line.chars()
                        .flat_map(|c| match c {
                            '#' => ['#', '#'],
                            'O' => ['[', ']'],
                            '.' => ['.', '.'],
                            '@' => ['@', '.'],
                            _ => panic!("Invalid character"),
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn get(&self, pos: &Coord) -> char {
        self.cells[pos.y as usize][pos.x as usize]
    }

    fn set(&mut self, pos: &Coord, value: char) {
        self.cells[pos.y as usize][pos.x as usize] = value;
    }

    fn find_robot(&self) -> Coord {
        self.cells
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().position(|&c| c == '@').map(|x| Coord {
                    x: x as i32,
                    y: y as i32,
                })
            })
            .expect("Robot not found")
    }

    fn is_wall(&self, pos: &Coord) -> bool {
        self.get(pos) == '#'
    }

    fn calc_gps_boxes_sum(&self) -> i32 {
        self.cells
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, &c)| {
                    if c == 'O' || c == '[' {
                        Some(Coord {
                            x: x as i32,
                            y: y as i32,
                        })
                    } else {
                        None
                    }
                })
            })
            .map(|coord| calc_gps(&coord))
            .sum()
    }
}

fn read_directions(input: &str) -> Vec<Direction> {
    input
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '>' => Some(Direction::Right),
            '<' => Some(Direction::Left),
            _ => None,
        })
        .collect()
}

fn eval_step(map: &mut Grid, robot_pos: &Coord, dir: &Direction) -> Coord {
    let next_pos = *robot_pos + dir.to_coord();

    if map.is_wall(&next_pos) {
        return *robot_pos;
    }

    match map.get(&next_pos) {
        '.' => {
            move_robot(map, robot_pos, &next_pos);
            next_pos
        }
        'O' => handle_box_push(map, robot_pos, &next_pos, dir, 1),
        '[' | ']' => handle_box_push(map, robot_pos, &next_pos, dir, 2),
        _ => *robot_pos,
    }
}

fn move_robot(grid: &mut Grid, from: &Coord, to: &Coord) {
    grid.set(from, '.');
    grid.set(to, '@');
}

fn handle_box_push(
    map: &mut Grid,
    robot_pos: &Coord,
    next_pos: &Coord,
    dir: &Direction,
    part: i32,
) -> Coord {
    let box_positions = if part == 1 {
        collect_consecutive_boxes(map, next_pos, dir)
    } else {
        collect_consecutive_boxes_part2(map, next_pos, dir)
    };

    let can_move = box_positions
        .iter()
        .all(|&b| !map.is_wall(&(b + dir.to_coord())));

    if can_move {
        // Store box symbols before modifying the map
        let box_moves: Vec<(Coord, Coord, char)> = box_positions
            .iter()
            .map(|&pos| {
                let new_pos = pos + dir.to_coord();
                let symbol = map.get(&pos);
                (pos, new_pos, symbol)
            })
            .collect();

        // Clear old positions
        for (old_pos, _, _) in &box_moves {
            map.set(&old_pos, '.');
        }

        // Execute the moves
        for (_, new_pos, symbol) in box_moves {
            map.set(&new_pos, symbol);
        }

        move_robot(map, robot_pos, next_pos);
        *next_pos
    } else {
        *robot_pos
    }
}

fn collect_consecutive_boxes(map: &Grid, start: &Coord, dir: &Direction) -> Vec<Coord> {
    std::iter::successors(Some(*start), |pos| {
        let next_pos = *pos + dir.to_coord();
        match map.get(&next_pos) {
            'O' | '[' | ']' if !map.is_wall(&next_pos) => Some(next_pos),
            _ => None,
        }
    })
    .collect()
}

fn collect_consecutive_boxes_part2(map: &Grid, start: &Coord, dir: &Direction) -> Vec<Coord> {
    fn collect_vertical(map: &Grid, start: &Coord, dir: &Direction) -> Vec<Coord> {
        let mut boxes = Vec::new();
        let mut to_check = vec![*start];
        let mut checked = HashSet::new();

        while let Some(pos) = to_check.pop() {
            if !checked.insert(pos) {
                continue;
            }

            match map.get(&pos) {
                '[' => {
                    boxes.push(pos);
                    to_check.extend([
                        pos + dir.to_coord(),              // vertical neighbor
                        pos + Direction::Right.to_coord(), // right part of box
                    ]);
                }
                ']' => {
                    boxes.push(pos);
                    to_check.extend([
                        pos + dir.to_coord(),             // vertical neighbor
                        pos + Direction::Left.to_coord(), // left part of box
                    ]);
                }
                _ => continue,
            }
        }

        boxes
    }

    match dir {
        Direction::Left | Direction::Right => collect_consecutive_boxes(map, start, dir),
        Direction::Up | Direction::Down => collect_vertical(map, start, dir),
    }
}

fn calc_gps(coord: &Coord) -> i32 {
    100 * coord.y + coord.x
}

impl Solution for Day15 {
    fn part1(&self, input: &str) -> String {
        let mut parts = input.split("\n\n");
        let mut grid = Grid::from(parts.next().unwrap());
        let directions = read_directions(parts.next().unwrap());
        let mut robot_pos = grid.find_robot();
        for dir in directions {
            robot_pos = eval_step(&mut grid, &robot_pos, &dir);
        }
        grid.calc_gps_boxes_sum().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut parts = input.split("\n\n");
        let mut grid = Grid::from_scaled_up(parts.next().unwrap());
        let directions = read_directions(parts.next().unwrap());
        let mut robot_pos = grid.find_robot();
        for dir in directions {
            robot_pos = eval_step(&mut grid, &robot_pos, &dir);
        }
        // 1475512
        grid.calc_gps_boxes_sum().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_small() {
        let input: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        let result = Day15.part1(input);
        assert_eq!(result, 2028.to_string());
    }

    #[test]
    fn test_part1_large() {
        let input: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";
        let result = Day15.part1(input);
        assert_eq!(result, 10092.to_string());
    }

    #[test]
    fn test_part2() {
        let input: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";
        let result = Day15.part2(input);
        assert_eq!(result, 9021.to_string());
    }
}
