use std::collections::HashMap;

use crate::Solution;

pub struct Day11;

struct Pebbles {
    pebbles: HashMap<usize, usize>,
}
impl Pebbles {
    fn from(input: &str) -> Self {
        let pebbles = input
            .split_whitespace()
            .map(|s| (s.parse().unwrap(), 1))
            .collect();
        Self { pebbles }
    }

    fn make_steps(&mut self, steps: usize) -> &mut Self {
        for _ in 0..steps {
            self.step();
        }
        self
    }

    fn step(&mut self) {
        let entries: Vec<(usize, usize)> = self.pebbles.drain().collect();

        for (value, count) in entries {
            match value {
                0 => {
                    *self.pebbles.entry(1).or_insert(0) += count;
                }
                n if has_even_digits(n) => {
                    let n_str = n.to_string();
                    let (left, right) = n_str.split_at(n_str.len() / 2);
                    let left_num: usize = left.parse().unwrap();
                    let right_num: usize = right.parse().unwrap();
                    *self.pebbles.entry(left_num).or_insert(0) += count;
                    *self.pebbles.entry(right_num).or_insert(0) += count;
                }
                n => {
                    *self.pebbles.entry(n * 2024).or_insert(0) += count;
                }
            }
        }
    }

    fn num_pebbles(&self) -> usize {
        self.pebbles.values().sum()
    }
}

fn has_even_digits(mut num: usize) -> bool {
    let mut count = 0;
    while num > 0 {
        count += 1;
        num /= 10;
    }
    count % 2 == 0
}

impl Solution for Day11 {
    fn part1(&self, input: &str) -> String {
        Pebbles::from(input)
            .make_steps(25)
            .num_pebbles()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        Pebbles::from(input)
            .make_steps(75)
            .num_pebbles()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: &str = "125 17";
        let result = Day11.part1(input);
        assert_eq!(result, 55312.to_string());
    }
}
