use crate::{utils::Coord, Solution};
use std::str::FromStr;

pub struct Day18;

#[cfg(test)]
const N_SIZE: usize = 7;
#[cfg(not(test))]
const N_SIZE: usize = 71;

#[cfg(test)]
const N_STEPS: usize = 12;
#[cfg(not(test))]
const N_STEPS: usize = 1024;

struct Grid {
    cells: Vec<Vec<char>>,
}

impl Grid {
    fn new(obstacles: &[Coord]) -> Self {
        let mut cells = vec![vec!['.'; N_SIZE]; N_SIZE];
        for &Coord { x, y } in obstacles {
            cells[y as usize][x as usize] = '#';
        }
        Self { cells }
    }

    fn find_path(&self) -> Vec<Coord> {
        let start = Coord { x: 0, y: 0 };
        let end = Coord {
            x: N_SIZE as i32 - 1,
            y: N_SIZE as i32 - 1,
        };

        let mut queue = vec![start];
        let mut visited = vec![vec![false; N_SIZE]; N_SIZE];
        let mut parent = vec![vec![Coord { x: -1, y: -1 }; N_SIZE]; N_SIZE];
        visited[start.y as usize][start.x as usize] = true;

        let mut found_end = false;
        while !queue.is_empty() && !found_end {
            let current = queue.remove(0);

            for dy in -1..=1 {
                for dx in -1..=1 {
                    // skip diagonal movement
                    if (dx != 0 && dy != 0) || (dx == 0 && dy == 0) {
                        continue;
                    }

                    let next = Coord {
                        x: current.x + dx,
                        y: current.y + dy,
                    };

                    if next.x < 0
                        || next.x >= N_SIZE as i32
                        || next.y < 0
                        || next.y >= N_SIZE as i32
                    {
                        continue;
                    }
                    if self.cells[next.y as usize][next.x as usize] == '#' {
                        continue;
                    }
                    if visited[next.y as usize][next.x as usize] {
                        continue;
                    }

                    visited[next.y as usize][next.x as usize] = true;
                    parent[next.y as usize][next.x as usize] = current;
                    queue.push(next);

                    if next == end {
                        found_end = true;
                        break;
                    }
                }
                if found_end {
                    break;
                }
            }
        }

        if !found_end {
            return Vec::new();
        }

        // Reconstruct path from end to start
        let mut path = Vec::new();
        let mut current = end;

        while current != start {
            path.push(current);
            current = parent[current.y as usize][current.x as usize];
            if current.x == -1 && current.y == -1 {
                return Vec::new();
            }
        }

        path.push(start);
        path.reverse();
        path
    }
}

impl FromStr for Coord {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or("ParseCoordError".to_owned())?;
        Ok(Coord {
            x: x.parse().map_err(|_| "ParseCoordError".to_owned())?,
            y: y.parse().map_err(|_| "ParseCoordError".to_owned())?,
        })
    }
}

impl Solution for Day18 {
    fn part1(&self, input: &str) -> String {
        let coords: Vec<Coord> = input
            .lines()
            .take(N_STEPS)
            .map(|line| line.parse().expect("error parsing"))
            .collect();
        let grid = Grid::new(&coords);
        (grid.find_path().len() - 1).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let coords: Vec<Coord> = input
            .lines()
            .map(|line| line.parse().expect("error parsing"))
            .collect();
        let res = "".to_owned();
        for i in N_STEPS..coords.len() {
            let coords = &coords[..i as usize];
            let grid = Grid::new(&coords);
            if grid.find_path().len() == 0 {
                let last = coords.last().unwrap();
                return format!("{},{}", last.x, last.y);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        let result = Day18.part1(input);
        assert_eq!(result, 22.to_string());
    }

    #[test]
    fn test_part2() {
        let input: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        let result = Day18.part2(input);
        assert_eq!(result, "6,1".to_string());
    }
}
