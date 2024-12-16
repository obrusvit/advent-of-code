use crate::{utils, Solution};

pub struct Day14;

#[cfg(test)]
const WIDTH: i32 = 11;
#[cfg(not(test))]
const WIDTH: i32 = 101;

#[cfg(test)]
const HEIGHT: i32 = 7;
#[cfg(not(test))]
const HEIGHT: i32 = 103;

const N_STEPS: usize = 100;

#[derive(Clone, Debug)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn from(input: &str) -> Self {
        let mut parts = input.split_whitespace();
        let p = parts.next().unwrap();
        let v = parts.next().unwrap();
        let mut p_parts = p.split(",");
        let x = p_parts
            .next()
            .unwrap()
            .trim_start_matches("p=")
            .parse()
            .unwrap();
        let y = p_parts.next().unwrap().parse().unwrap();
        let mut v_parts = v.split(",");
        let vx = v_parts
            .next()
            .unwrap()
            .trim_start_matches("v=")
            .parse()
            .unwrap();
        let vy = v_parts.next().unwrap().parse().unwrap();
        Self { x, y, vx, vy }
    }
}

struct Simulation {
    robots: Vec<Robot>,
}

impl Simulation {
    fn new(robots: Vec<Robot>) -> Self {
        Self { robots }
    }
    fn simulate(&mut self, n_steps: usize) -> &mut Self {
        for _ in 0..n_steps {
            self.step();
        }
        self
    }

    fn step(&mut self) {
        self.robots.iter_mut().for_each(|robot| {
            robot.x = ((robot.x + robot.vx) % WIDTH + WIDTH) % WIDTH;
            robot.y = ((robot.y + robot.vy) % HEIGHT + HEIGHT) % HEIGHT;
        })
    }

    fn eval_quadrants(&self) -> usize {
        let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
        let (borderline_x, borderline_y) = (WIDTH / 2, HEIGHT / 2);
        for robot in &self.robots {
            // assign to quadrant but exclude the middle row and middle column
            if robot.x < borderline_x && robot.y < borderline_y {
                q1 += 1;
            } else if robot.x > borderline_x && robot.y < borderline_y {
                q2 += 1;
            } else if robot.x < borderline_x && robot.y > borderline_y {
                q3 += 1;
            } else if robot.x > borderline_x && robot.y > borderline_y {
                q4 += 1;
            }
        }
        q1 * q2 * q3 * q4
    }

    fn part2(&mut self) -> usize {
        // p1 result: 225943500
        // Use eval safety as "entropy" source.
        let mut n_steps = 0;
        loop {
            n_steps += 1;
            self.step();
            let quad = self.eval_quadrants();
            if quad < 225943500 / 4 || n_steps > 10000 {
                // 6377 <-- christmas tree, yaay
                return n_steps;
            }
        }
    }

    fn print_snapshot(&self) {
        let mut snapshot = vec![vec!['.'; WIDTH as usize]; HEIGHT as usize];
        for robot in &self.robots {
            snapshot[robot.y as usize][robot.x as usize] = '#';
        }
        println!(
            "{}",
            snapshot
                .iter()
                .map(|row| row.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
}

impl Solution for Day14 {
    fn part1(&self, input: &str) -> String {
        let robots = input
            .lines()
            .map(|line| Robot::from(line))
            .collect::<Vec<Robot>>();
        Simulation::new(robots)
            .simulate(N_STEPS)
            .eval_quadrants()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let robots = input
            .lines()
            .map(|line| Robot::from(line))
            .collect::<Vec<Robot>>();
        let n_steps = Simulation::new(robots.clone()).part2();
        Simulation::new(robots).simulate(n_steps).print_snapshot();
        "0".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // test input is 11 tiles wide and 7 tiles tall
        let input: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        let result = Day14.part1(input);
        assert_eq!(result, 12.to_string());
    }
}
