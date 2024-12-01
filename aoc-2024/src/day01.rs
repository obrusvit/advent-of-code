use crate::Solution;

pub struct Day01;

fn parse_numbers_from_string(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

impl Solution for Day01 {
    fn part1(&self, input: &str) -> String {
        let (mut v1, mut v2): (Vec<i32>, Vec<i32>) = input
            .lines()
            .map(parse_numbers_from_string)
            .map(|nums| (nums[0], nums[1]))
            .unzip();
        v1.sort();
        v2.sort();

        v1.iter()
            .zip(v2.iter())
            .map(|(l, r)| (r - l).abs())
            .sum::<i32>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (v1, v2): (Vec<i32>, Vec<i32>) = input
            .lines()
            .map(parse_numbers_from_string)
            .map(|nums| (nums[0], nums[1]))
            .unzip();
        v1.iter()
            .map(|x| {
                let count = v2.iter().filter(|&&y| &y == x).count();
                x * count as i32
            })
            .sum::<i32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: &str = "3   4
            4   3
            2   5
            1   3
            3   9
            3   3";
        let result = Day01.part1(input);
        assert_eq!(result, 11.to_string());
    }

    #[test]
    fn test_part2() {
        let input: &str = "3   4
            4   3
            2   5
            1   3
            3   9
            3   3";
        let result = Day01.part2(input);
        assert_eq!(result, "31".to_string());
    }
}
