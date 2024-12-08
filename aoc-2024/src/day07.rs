use crate::{utils, Solution};

pub struct Day07;

#[derive(Debug)]
struct Equation {
    test_value: i64,
    numbers: Vec<i64>,
}

fn concat(a: i64, b: i64) -> i64 {
    let b_str = b.to_string();
    format!("{}{}", a, b_str).parse().unwrap()
}

fn evaluate(numbers: &[i64], operators: &[char]) -> i64 {
    let mut result = numbers[0];
    for i in 0..operators.len() {
        match operators[i] {
            '+' => result += numbers[i + 1],
            '*' => result *= numbers[i + 1],
            '|' => result = concat(result, numbers[i + 1]),
            _ => panic!("Invalid operator"),
        }
    }
    result
}

fn try_all_operators(equation: &Equation) -> bool {
    let op_count = equation.numbers.len() - 1;
    let combinations = 2_i32.pow(op_count as u32); // 2^n combinations for n operator positions

    for i in 0..combinations {
        let mut operators = Vec::new();
        for j in 0..op_count {
            // Use bits of i to determine operator: 0 for '+', 1 for '*'
            operators.push(if (i & (1 << j)) == 0 { '+' } else { '*' });
        }

        if evaluate(&equation.numbers, &operators) == equation.test_value {
            return true;
        }
    }
    false
}

fn try_all_operators2(equation: &Equation) -> bool {
    let op_count = equation.numbers.len() - 1;
    let combinations = 3_i32.pow(op_count as u32); // 3^n combinations for n operator positions

    for i in 0..combinations {
        let mut operators = Vec::new();
        for j in 0..op_count {
            // Use base-3 digits to determine operator: 0 for '+', 1 for '*', 2 for '||'
            let op = match (i / 3_i32.pow(j as u32)) % 3 {
                0 => '+',
                1 => '*',
                2 => '|',
                _ => unreachable!(),
            };
            operators.push(op);
        }

        if evaluate(&equation.numbers, &operators) == equation.test_value {
            return true;
        }
    }
    false
}

fn parse_equation(line: &str) -> Equation {
    let parts: Vec<&str> = line.split(": ").collect();
    let test_value = parts[0].parse().unwrap();
    let numbers: Vec<i64> = parts[1]
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    Equation {
        test_value,
        numbers,
    }
}

impl Solution for Day07 {
    fn part1(&self, input: &str) -> String {
        let equations: Vec<Equation> = input.lines().map(parse_equation).collect();

        equations
            .iter()
            .filter(|eq| try_all_operators(eq))
            .map(|eq| eq.test_value)
            .sum::<i64>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let equations: Vec<Equation> = input.lines().map(parse_equation).collect();

        equations
            .iter()
            .filter(|eq| try_all_operators2(eq))
            .map(|eq| eq.test_value)
            .sum::<i64>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let result = Day07.part1(input);
        assert_eq!(result, 3749.to_string());
    }

    #[test]
    fn test_part2() {
        let input: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let result = Day07.part2(input);
        assert_eq!(result, 11387.to_string());
    }
}
