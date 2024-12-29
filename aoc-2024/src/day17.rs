use crate::{utils, Solution};

pub struct Day17;

#[derive(Debug)]
struct Computer {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    instruction_ptr: usize,
    program: Vec<i64>,
    output: Vec<i64>,
}

impl Computer {
    fn new(register_a: i64, register_b: i64, register_c: i64, program: Vec<i64>) -> Self {
        Computer {
            register_a,
            register_b,
            register_c,
            instruction_ptr: 0,
            program,
            output: Vec::new(),
        }
    }

    fn get_combo_value(&self, operand: i64) -> i64 {
        match operand {
            0..=3 => operand,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Invalid combo operand: {}", operand),
        }
    }

    fn run(&mut self) {
        while self.instruction_ptr < self.program.len() {
            let opcode = self.program[self.instruction_ptr];
            let operand = self.program[self.instruction_ptr + 1];

            match opcode {
                0 => { // adv
                    let power = self.get_combo_value(operand);
                    self.register_a /= 1 << power;
                }
                1 => { // bxl
                    self.register_b ^= operand;
                }
                2 => { // bst
                    self.register_b = self.get_combo_value(operand) % 8;
                }
                3 => { // jnz
                    if self.register_a != 0 {
                        self.instruction_ptr = operand as usize;
                        continue;
                    }
                }
                4 => { // bxc
                    self.register_b ^= self.register_c;
                }
                5 => { // out
                    let value = self.get_combo_value(operand) % 8;
                    self.output.push(value);
                }
                6 => { // bdv
                    let power = self.get_combo_value(operand);
                    self.register_b = self.register_a / (1 << power);
                }
                7 => { // cdv
                    let power = self.get_combo_value(operand);
                    self.register_c = self.register_a / (1 << power);
                }
                _ => panic!("Invalid opcode: {}", opcode),
            }
            self.instruction_ptr += 2;
        }
    }
}

impl Solution for Day17 {
    fn part1(&self, input: &str) -> String {
        let lines: Vec<&str> = input.lines().collect();
        let register_a = lines[0].split(": ").nth(1).unwrap().parse().unwrap();
        let register_b = lines[1].split(": ").nth(1).unwrap().parse().unwrap();
        let register_c = lines[2].split(": ").nth(1).unwrap().parse().unwrap();
        
        let program: Vec<i64> = lines[4]
            .split(": ")
            .nth(1)
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        let mut computer = Computer::new(register_a, register_b, register_c, program);
        computer.run();
        
        computer.output.iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn part2(&self, input: &str) -> String {
        unimplemented!("Part 2 not implemented yet")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        let result = Day17.part1(input);
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0".to_string());
    }

    #[test]
    fn test_part2() {
        let input: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        let result = Day17.part2(input);
        assert_eq!(result, "117440");
    }
}
