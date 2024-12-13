use std::collections::HashSet;

use crate::Solution;

pub struct Day10;

type Height = usize;

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    heights: Vec<Vec<Height>>,
}
impl Map {
    fn from(input: &str) -> Self {
        let mut heights: Vec<Vec<Height>> = Vec::new();
        let mut width = 0;
        let mut height = 0;
        for line in input.lines() {
            width = line.len();
            height += 1;
            let mut row = Vec::new();
            for c in line.chars() {
                let height = c.to_digit(10).unwrap() as Height;
                row.push(height);
            }
            heights.push(row);
        }

        Self {
            width,
            height,
            heights,
        }
    }

    fn for_low_points<F>(&self, f: F) -> usize
    where
        F: Fn(&Self, usize, usize) -> usize,
    {
        let mut res = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.heights[y][x] == 0 {
                    res += f(self, x, y);
                }
            }
        }
        res
    }

    fn dfs_part1(&self, x: usize, y: usize, reached: &mut HashSet<(usize, usize)>) {
        let curr = self.heights[y][x];
        if curr == 9 {
            reached.insert((x, y));
            return;
        }
        self.get_adjacent_with_next(x, y)
            .iter()
            .for_each(|c| self.dfs_part1(c.0, c.1, reached));
    }

    fn dfs_part2(&self, x: usize, y: usize) -> usize {
        let curr = self.heights[y][x];
        if curr == 9 {
            return 1;
        }
        self.get_adjacent_with_next(x, y)
            .iter()
            .map(|c| self.dfs_part2(c.0, c.1))
            .sum()
    }

    fn get_adjacent_with_next(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let curr_height = self.heights[y][x];
        let mut result = Vec::new();
        if x > 0 && self.heights[y][x - 1] == curr_height + 1 {
            result.push((x - 1, y));
        }
        if x < self.width - 1 && self.heights[y][x + 1] == curr_height + 1 {
            result.push((x + 1, y));
        }
        if y > 0 && self.heights[y - 1][x] == curr_height + 1 {
            result.push((x, y - 1));
        }
        if y < self.height - 1 && self.heights[y + 1][x] == curr_height + 1 {
            result.push((x, y + 1));
        }
        result
    }
}

impl Solution for Day10 {
    fn part1(&self, input: &str) -> String {
        Map::from(input)
            .for_low_points(|map, x, y| {
                let mut reached = HashSet::new();
                map.dfs_part1(x, y, &mut reached);
                reached.len()
            })
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        Map::from(input)
            .for_low_points(|map, x, y| map.dfs_part2(x, y))
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trail() {
        let input: &str = "4440444
4441444
4442444
6543456
7444447
8444448
9444449";
        let result = Day10.part1(input);
        assert_eq!(result, 2.to_string());
    }

    #[test]
    fn test_part1() {
        let input: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let result = Day10.part1(input);
        assert_eq!(result, 36.to_string());
    }

    #[test]
    fn test_part2() {
        let input: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let result = Day10.part2(input);
        assert_eq!(result, 81.to_string());
    }
}
