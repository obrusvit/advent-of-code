use crate::Solution;

pub struct Day13;

type Vector = (f64, f64);

struct Machine {
    x: Vector,
    y: Vector,
    t: Vector,
}

impl Machine {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();
        let parse_coords = |line: &str| {
            let coords = line.split(": ").nth(1).unwrap();
            let (x, y) = coords.split_once(", ").unwrap();
            (
                x.trim_start_matches("X")
                    .trim_start_matches("+")
                    .trim_start_matches("=")
                    .parse::<f64>()
                    .unwrap(),
                y.trim_start_matches("Y")
                    .trim_start_matches("+")
                    .trim_start_matches("=")
                    .parse::<f64>()
                    .unwrap(),
            )
        };

        let x = parse_coords(lines.next().unwrap());
        let y = parse_coords(lines.next().unwrap());
        let t = parse_coords(lines.next().unwrap());

        Self { x, y, t }
    }

    fn with_correction(&self) -> Self {
        const CORRECTION: f64 = 10000000000000.0;
        Self {
            t: (self.t.0 + CORRECTION, self.t.1 + CORRECTION),
            ..(*self)
        }
    }

    fn solve(&self) -> Option<i64> {
        linsolve(self.x, self.y, self.t).map(|(a, b)| 3 * a as i64 + b as i64)
    }
}

fn float_eq(n1: f64, n2: f64) -> bool {
    (n1 - n2).abs() < f64::EPSILON
}

fn linsolve(x: Vector, y: Vector, t: Vector) -> Option<Vector> {
    let det = x.0 * y.1 - x.1 * y.0;
    if float_eq(det, 0.0) {
        return None;
    }

    let a = (t.0 * y.1 - t.1 * y.0) / det;
    let b = (x.0 * t.1 - x.1 * t.0) / det;

    // test if a and b are integers
    if !float_eq(a, a.round()) || !float_eq(b, b.round()) {
        return None;
    }
    Some((a, b))
}

impl Solution for Day13 {
    fn part1(&self, input: &str) -> String {
        input
            .split("\n\n")
            .into_iter()
            .map(|sys| Machine::from(sys).solve().unwrap_or(0))
            .sum::<i64>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        input
            .split("\n\n")
            .into_iter()
            .map(|sys| Machine::from(sys).with_correction().solve().unwrap_or(0))
            .sum::<i64>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let result = Day13.part1(input);
        assert_eq!(result, 480.to_string());
    }
}
