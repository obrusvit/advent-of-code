use std::collections::{HashMap, HashSet};

use crate::Solution;

pub struct Day12;

type Crop = char;

#[derive(Debug)]
struct Garden {
    #[allow(dead_code)]
    id: usize,
    #[allow(dead_code)]
    crop: Crop,
    coordinates: HashSet<(usize, usize)>,
    neighbors: HashMap<char, usize>,
}

impl Garden {
    fn new(id: usize, crop: Crop) -> Self {
        Self {
            id,
            crop,
            coordinates: HashSet::new(),
            neighbors: HashMap::new(),
        }
    }

    fn area(&self) -> usize {
        self.coordinates.len()
    }

    fn perimeter(&self) -> usize {
        self.neighbors.values().sum()
    }

    fn sides(&self) -> usize {
        self.coordinates
            .iter()
            .map(|(x, y)| {
                // calculate if neigbors are within the Garden
                let top = self.coordinates.contains(&(*x, y.wrapping_sub(1)));
                let bot = self.coordinates.contains(&(*x, y + 1));
                let left = self.coordinates.contains(&(x.wrapping_sub(1), *y));
                let right = self.coordinates.contains(&(x + 1, *y));
                let top_left = self
                    .coordinates
                    .contains(&(x.wrapping_sub(1), y.wrapping_sub(1)));
                let top_right = self.coordinates.contains(&(x + 1, y.wrapping_sub(1)));
                let bot_left = self.coordinates.contains(&(x.wrapping_sub(1), y + 1));
                let bot_right = self.coordinates.contains(&(x + 1, y + 1));

                // determine the type of the corner
                let left_top_convex_corner = { !left && !top };
                let right_top_convex_corner = { !top && !right };
                let left_bot_convex_corner = { !left && !bot };
                let right_bot_convex_corner = { !right && !bot };
                let left_top_concave_corner = { left && top && !top_left };
                let right_top_concave_corner = { top && right && !top_right };
                let left_bot_concave_corner = { left && bot && !bot_left };
                let right_bot_concave_corner = { right && bot && !bot_right };

                // calculate how many of these `_corner` variables are true
                [
                    left_top_convex_corner,
                    right_top_convex_corner,
                    left_bot_convex_corner,
                    right_bot_convex_corner,
                    left_top_concave_corner,
                    right_top_concave_corner,
                    left_bot_concave_corner,
                    right_bot_concave_corner,
                ]
                .iter()
                .filter(|&&x| x)
                .count()
            })
            .sum()
    }
}
struct GardensMap {
    width: usize,
    height: usize,
    grid: Vec<Vec<Crop>>,
    gardens: Vec<Garden>,
}

impl GardensMap {
    fn from(input: &str) -> Self {
        let grid: Vec<Vec<Crop>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = grid.len();
        let width = grid[0].len();

        let mut res = GardensMap {
            width,
            height,
            grid,
            gardens: Vec::new(),
        };

        res.process_map();
        res
    }

    fn process_map(&mut self) {
        let mut visited = HashSet::new();

        // Find all gardens using flood fill algo
        let mut garden_id = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if !visited.contains(&(x, y)) {
                    let crop_type = self.grid[y][x];
                    self.flood_fill(x, y, garden_id, crop_type, &mut visited);
                    garden_id += 1;
                }
            }
        }
    }

    fn flood_fill(
        &mut self,
        x: usize,
        y: usize,
        id: usize,
        crop: Crop,
        visited: &mut HashSet<(usize, usize)>,
    ) {
        let mut garden = Garden::new(id, crop);
        garden.coordinates.insert((x, y));

        let mut stack = vec![(x, y)];
        visited.insert((x, y));

        while let Some((curr_x, curr_y)) = stack.pop() {
            // Count edges of the map as fences
            if curr_x == 0 {
                *garden.neighbors.entry('#').or_insert(0) += 1;
            }
            if curr_x == self.width - 1 {
                *garden.neighbors.entry('#').or_insert(0) += 1;
            }
            if curr_y == 0 {
                *garden.neighbors.entry('#').or_insert(0) += 1;
            }
            if curr_y == self.height - 1 {
                *garden.neighbors.entry('#').or_insert(0) += 1;
            }

            let adjacent = [
                (curr_x.wrapping_sub(1), curr_y), // left
                (curr_x + 1, curr_y),             // right
                (curr_x, curr_y.wrapping_sub(1)), // up
                (curr_x, curr_y + 1),             // down
            ];

            for (next_x, next_y) in adjacent {
                if next_x >= self.width || next_y >= self.height {
                    continue;
                }

                let next_crop = self.grid[next_y][next_x];
                if next_crop == crop {
                    if !visited.contains(&(next_x, next_y)) {
                        // Same crop type - expand garden
                        visited.insert((next_x, next_y));
                        garden.coordinates.insert((next_x, next_y));
                        stack.push((next_x, next_y));
                    }
                } else {
                    // Different crop type - count as neighbor
                    *garden.neighbors.entry(next_crop).or_insert(0) += 1;
                }
            }
        }

        self.gardens.push(garden);
    }

    fn get_fence_price_part1(&self) -> usize {
        self.gardens
            .iter()
            .map(|garden| garden.area() * garden.perimeter())
            .sum()
    }

    fn get_fence_price_part2(&self) -> usize {
        self.gardens
            .iter()
            .map(|garden| garden.area() * garden.sides())
            .sum()
    }
}

impl Solution for Day12 {
    fn part1(&self, input: &str) -> String {
        GardensMap::from(input).get_fence_price_part1().to_string()
    }

    fn part2(&self, input: &str) -> String {
        GardensMap::from(input).get_fence_price_part2().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_small() {
        let input: &str = "AAAA
BBCD
BBCC
EEEC";
        let result = Day12.part1(input);
        assert_eq!(result, 140.to_string());
    }

    #[test]
    fn test_part1_bigger() {
        let input: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        let result = Day12.part1(input);
        assert_eq!(result, 772.to_string());
    }

    #[test]
    fn test_part1() {
        let input: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let result = Day12.part1(input);
        assert_eq!(result, 1930.to_string());
    }

    #[test]
    fn test_part2_small() {
        let input: &str = "AAAA
BBCD
BBCC
EEEC";
        let result = Day12.part2(input);
        assert_eq!(result, 80.to_string());
    }

    #[test]
    fn test_part2_bigger() {
        let input: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        let result = Day12.part2(input);
        assert_eq!(result, 436.to_string());
    }

    #[test]
    fn test_part2_eshape() {
        let input: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        let result = Day12.part2(input);
        assert_eq!(result, 236.to_string());
    }

    #[test]
    fn test_part2_ab() {
        let input: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        let result = Day12.part2(input);
        assert_eq!(result, 368.to_string());
    }

    #[test]
    fn test_part2() {
        let input: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let result = Day12.part2(input);
        assert_eq!(result, 1206.to_string());
    }
}
