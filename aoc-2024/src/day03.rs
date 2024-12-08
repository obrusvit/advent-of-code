use crate::Solution;
use regex::Regex;

pub struct Day03;

impl Solution for Day03 {
    fn part1(&self, input: &str) -> String {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let sum = re.captures_iter(input).fold(0, |sum, cap| {
            let m1 = cap[1].parse::<i32>().unwrap();
            let m2 = cap[2].parse::<i32>().unwrap();
            sum + m1 * m2
        });
        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
        let mut allowed = true;
        let sum = re.captures_iter(input).fold(0, |sum, cap| {
            let matched = cap.get(0).unwrap().as_str();
            match matched {
                "do()" => {
                    allowed = true;
                    sum
                }
                "don't()" => {
                    allowed = false;
                    sum
                }
                _ if allowed => {
                    let m1 = cap[1].parse::<i32>().unwrap();
                    let m2 = cap[2].parse::<i32>().unwrap();
                    sum + m1 * m2
                }
                _ => sum,
            }
        });
        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = Day03.part1(input);
        assert_eq!(result, 161.to_string());
    }

    #[test]
    fn test_part2() {
        let input: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = Day03.part2(input);
        assert_eq!(result, 48.to_string());
    }
}
