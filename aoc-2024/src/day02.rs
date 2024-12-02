use crate::{utils, Solution};

pub struct Day02;

fn eval_safety(levels: &[i32]) -> bool {
    let mut rising: Option<bool> = None;
    for window in levels.windows(2) {
        let (prev, curr) = (window[0], window[1]);
        let grad = (prev - curr).abs();
        if grad > 3 || grad == 0 {
            return false;
        }
        let is_rising = curr > prev;
        if let Some(was_rising) = rising {
            if was_rising != is_rising {
                return false;
            }
        } else {
            rising = Some(is_rising);
        }
    }
    true
}

fn eval_safety_p2(levels: &[i32]) -> bool {
    if eval_safety(levels) {
        return true;
    }
    // Try removing one element and check if it's safe
    for i in 0..levels.len() {
        let mut levels_copy = levels.to_vec();
        levels_copy.remove(i);
        if eval_safety(&levels_copy) {
            return true;
        }
    }

    false
}

impl Solution for Day02 {
    fn part1(&self, input: &str) -> String {
        input
            .lines()
            .map(utils::parse_numbers_from_string)
            .filter(|numbers| eval_safety(numbers))
            .count()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        input
            .lines()
            .map(utils::parse_numbers_from_string)
            .filter(|numbers| eval_safety_p2(numbers))
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const LEVELS_1: &str = "7 6 4 2 1";
    const LEVELS_2: &str = "1 2 7 8 9";
    const LEVELS_3: &str = "9 7 6 2 1";
    const LEVELS_4: &str = "1 3 2 4 5";
    const LEVELS_5: &str = "8 6 4 4 1";
    const LEVELS_6: &str = "1 3 6 7 9";

    #[test]
    fn test_eval_safety_p1() {
        assert_eq!(
            eval_safety(&utils::parse_numbers_from_string(LEVELS_1)),
            true
        );
        assert_eq!(
            eval_safety(&utils::parse_numbers_from_string(LEVELS_2)),
            false
        );
        assert_eq!(
            eval_safety(&utils::parse_numbers_from_string(LEVELS_3)),
            false
        );
        assert_eq!(
            eval_safety(&utils::parse_numbers_from_string(LEVELS_4)),
            false
        );
        assert_eq!(
            eval_safety(&utils::parse_numbers_from_string(LEVELS_5)),
            false
        );
        assert_eq!(
            eval_safety(&utils::parse_numbers_from_string(LEVELS_6)),
            true
        );
    }

    #[test]
    fn test_eval_safety_p2() {
        assert_eq!(
            eval_safety_p2(&utils::parse_numbers_from_string(LEVELS_1)),
            true
        );
        assert_eq!(
            eval_safety_p2(&utils::parse_numbers_from_string(LEVELS_2)),
            false
        );
        assert_eq!(
            eval_safety_p2(&utils::parse_numbers_from_string(LEVELS_3)),
            false
        );
        assert_eq!(
            eval_safety_p2(&utils::parse_numbers_from_string(LEVELS_4)),
            true
        );
        assert_eq!(
            eval_safety_p2(&utils::parse_numbers_from_string(LEVELS_5)),
            true
        );
        assert_eq!(
            eval_safety_p2(&utils::parse_numbers_from_string(LEVELS_6)),
            true
        );
    }

    #[test]
    fn test_part1() {
        let input: &str = "7 6 4 2 1
                           1 2 7 8 9
                           9 7 6 2 1
                           1 3 2 4 5
                           8 6 4 4 1
                           1 3 6 7 9";
        let result = Day02.part1(input);
        assert_eq!(result, 2.to_string());
    }

    #[test]
    fn test_part2() {
        let input: &str = "7 6 4 2 1
                           1 2 7 8 9
                           9 7 6 2 1
                           1 3 2 4 5
                           8 6 4 4 1
                           1 3 6 7 9";
        let result = Day02.part2(input);
        assert_eq!(result, 4.to_string());
    }
}
