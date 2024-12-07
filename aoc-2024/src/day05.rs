use crate::{utils, Solution};

pub struct Day05;

type Rules = Vec<(i32, i32)>;
type Update = Vec<i32>;

fn is_sorted(rules: &Rules, update: &Update) -> bool {
    for (idx, n) in update.iter().enumerate() {
        for (a, b) in rules.iter() {
            if *a == *n {
                let preceding = &update[..idx];
                if preceding.contains(b) {
                    return false;
                }
            }
        }
    }
    true
}

fn sort(rules: &Rules, update: &Update) -> Update {
    // Brute force sorting is fine. Remember, we're doing this for fun.
    let mut res = update.clone();
    while !is_sorted(rules, &res) {
        let mut swapped = false;
        let len = res.len();
        for idx in 0..len {
            let n = res[idx];
            for (a, b) in rules.iter() {
                if *a == n {
                    let preceding = &res[..idx];
                    if preceding.contains(b) {
                        let b_idx = preceding.iter().position(|x| x == b).unwrap();
                        res.swap(idx, b_idx);
                        swapped = true;
                    }
                }
            }
        }
        if !swapped {
            break;
        }
    }
    res
}

fn parse_rules(input: &str) -> Rules {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split('|');
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

impl Solution for Day05 {
    fn part1(&self, input: &str) -> String {
        let mut parts = input.split("\n\n");
        let rules = parse_rules(parts.next().unwrap());
        let updates = parts.next().unwrap().lines();
        let mut res = 0;
        for update in updates {
            let update = utils::parse_numbers_from_string_comma(update);
            if is_sorted(&rules, &update) {
                res += update[update.len() / 2];
            }
        }
        res.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut parts = input.split("\n\n");
        let rules = parse_rules(parts.next().unwrap());
        let updates = parts.next().unwrap().lines();
        let mut res = 0;
        for update in updates {
            let update = utils::parse_numbers_from_string_comma(update);
            if !is_sorted(&rules, &update) {
                let sorted = sort(&rules, &update);
                res += sorted[sorted.len() / 2];
            }
        }
        res.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_rules() -> Rules {
        parse_rules(
            "97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13",
        )
    }

    #[test]
    fn test_is_sorted() {
        let test_rules = get_test_rules();
        assert_eq!(
            is_sorted(
                &test_rules,
                &utils::parse_numbers_from_string_comma("75,47,61,53,29")
            ),
            true
        );
        assert_eq!(
            is_sorted(
                &test_rules,
                &utils::parse_numbers_from_string_comma("97,61,53,29,13")
            ),
            true
        );
        assert_eq!(
            is_sorted(
                &test_rules,
                &utils::parse_numbers_from_string_comma("75,29,13")
            ),
            true
        );
        assert_eq!(
            is_sorted(
                &test_rules,
                &utils::parse_numbers_from_string_comma("75,97,47,61,53")
            ),
            false
        );
        assert_eq!(
            is_sorted(
                &test_rules,
                &utils::parse_numbers_from_string_comma("61,13,29")
            ),
            false
        );
        assert_eq!(
            is_sorted(
                &test_rules,
                &utils::parse_numbers_from_string_comma("97,13,75,29,47")
            ),
            false
        );
    }

    #[test]
    fn test_sort() {
        let test_rules = get_test_rules();
        assert_eq!(
            sort(
                &test_rules,
                &utils::parse_numbers_from_string_comma("75,97,47,61,53")
            ),
            vec![97, 75, 47, 61, 53]
        );
        assert_eq!(
            sort(
                &test_rules,
                &utils::parse_numbers_from_string_comma("61,13,29")
            ),
            vec![61, 29, 13]
        );
        assert_eq!(
            sort(
                &test_rules,
                &utils::parse_numbers_from_string_comma("97,13,75,29,47")
            ),
            vec![97, 75, 47, 29, 13]
        );
    }

    #[test]
    fn test_part1() {
        let input: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let result = Day05.part1(input);
        assert_eq!(result, 143.to_string());
    }

    #[test]
    fn test_part2() {
        let input: &str = "";
        let result = Day05.part2(input);
        assert_eq!(result, 0.to_string());
    }
}
