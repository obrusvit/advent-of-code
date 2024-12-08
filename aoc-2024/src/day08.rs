use std::collections::HashSet;

use crate::Solution;

pub struct Day08;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Antenna {
    x: i32,
    y: i32,
    freq: char,
}

#[derive(Debug)]
struct City {
    width: i32,
    height: i32,
    antennas: Vec<Antenna>,
    antinodes: HashSet<(i32, i32)>,
}

impl City {
    fn push_antinode(&mut self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return false;
        }
        self.antinodes.insert((x, y));
        true
    }
}

fn read_input(input: &str) -> City {
    let mut antennas = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in input.lines().enumerate() {
        height += 1;
        width = line.len() as i32;
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => continue,
                c => antennas.push(Antenna {
                    x: x as i32,
                    y: y as i32,
                    freq: c,
                }),
            }
        }
    }
    City {
        width,
        height,
        antennas,
        antinodes: HashSet::new(),
    }
}

impl Solution for Day08 {
    fn part1(&self, input: &str) -> String {
        let mut city = read_input(input);
        let antennas = city.antennas.clone();
        let mut pairs_evaluated: HashSet<(Antenna, Antenna)> = HashSet::new();
        for a1 in &antennas {
            for a2 in &antennas {
                if a1 == a2 || a1.freq != a2.freq || !pairs_evaluated.insert((*a1, *a2)) {
                    continue;
                }
                let dx = a1.x - a2.x;
                let dy = a1.y - a2.y;
                city.push_antinode(a1.x + dx, a1.y + dy);
                city.push_antinode(a2.x - dx, a2.y - dy);
            }
        }
        city.antinodes.len().to_string() // 398
    }

    fn part2(&self, input: &str) -> String {
        let mut city = read_input(input);
        let antennas = city.antennas.clone();
        let mut pairs_evaluated: HashSet<(Antenna, Antenna)> = HashSet::new();
        for a1 in &antennas {
            for a2 in &antennas {
                if a1 == a2 || a1.freq != a2.freq || !pairs_evaluated.insert((*a1, *a2)) {
                    continue;
                }
                let dx = a1.x - a2.x;
                let dy = a1.y - a2.y;
                let mut factor = 0;
                while city.push_antinode(a1.x + factor * dx, a1.y + factor * dy)
                    || city.push_antinode(a2.x - factor * dx, a2.y - factor * dy)
                {
                    factor += 1;
                }
            }
        }
        city.antinodes.len().to_string() // 1333
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let result = Day08.part1(input);
        assert_eq!(result, 14.to_string());
    }

    #[test]
    fn test_part2() {
        let input: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let result = Day08.part2(input);
        assert_eq!(result, 34.to_string());
    }
}
