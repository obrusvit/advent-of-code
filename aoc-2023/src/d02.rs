use regex::Regex;
use std::fs::read_to_string;

pub fn solve_d2() -> (i32, i32) {
    let mut res = 0;
    let mut power_cubes = 0;
    let lines = read_lines("data/d02.txt");
    for game_line in lines {
        let mut game_possible = true;
        let mut minimal = CubeSet::new(0, 0, 0);
        let parts: Vec<&str> = game_line.split(':').collect();
        for s_withdrawal in parts[1].split(';') {
            let one_withdrawal = parse_one_withdrawal(s_withdrawal);
            minimal.update_minimal(&one_withdrawal);
            game_possible = game_possible && one_withdrawal.is_possible();
        }
        if game_possible {
            let id = parse_game_id(parts[0]);
            res += id;
        }
        power_cubes += minimal.get_power();
    }
    // 2101, 58269
    (res, power_cubes)
}

fn parse_game_id(s: &str) -> i32 {
    let re = Regex::new(r"Game (?<game_id>\d+)").unwrap();
    re.captures(s)
        .and_then(|cap| cap["game_id"].to_string().parse::<i32>().ok())
        .expect("Game ID not found")
}

fn parse_one_withdrawal(s: &str) -> CubeSet {
    fn parse_color(regex: &Regex, s: &str) -> i32 {
        regex
            .captures(s)
            .and_then(|cap| cap[1].parse::<i32>().ok())
            .unwrap_or(0)
    }
    let re_r = Regex::new(r"(?<n_red>\d+) red").unwrap();
    let re_g = Regex::new(r"(?<n_green>\d+) green").unwrap();
    let re_b = Regex::new(r"(?<n_blue>\d+) blue").unwrap();

    let r = parse_color(&re_r, s);
    let g = parse_color(&re_g, s);
    let b = parse_color(&re_b, s);

    CubeSet::new(r, g, b)
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

#[derive(Debug, PartialEq)]
struct CubeSet {
    r: i32,
    g: i32,
    b: i32,
}

impl CubeSet {
    const MAX_R: i32 = 12;
    const MAX_G: i32 = 13;
    const MAX_B: i32 = 14;

    fn new(r: i32, g: i32, b: i32) -> Self {
        Self { r, g, b }
    }

    fn is_possible(&self) -> bool {
        return self.r <= Self::MAX_R && self.g <= Self::MAX_G && self.b <= Self::MAX_B;
    }

    fn update_minimal(&mut self, other: &CubeSet) {
        self.r = self.r.max(other.r);
        self.g = self.g.max(other.g);
        self.b = self.b.max(other.b);
    }

    fn get_power(&self) -> i32 {
        return self.r * self.g * self.b;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parge_game_id_test() {
        assert_eq!(parse_game_id(&"Game 2"), 2);
        assert_eq!(parse_game_id(&"Game 21"), 21);
        assert_eq!(parse_game_id(&"Game 999"), 999);
    }

    #[test]
    fn parse_one_withdrawal_test() {
        assert_eq!(
            parse_one_withdrawal(&"3 blue, 4 red"),
            CubeSet::new(4, 0, 3)
        );
        assert_eq!(
            parse_one_withdrawal(&"4 red, 1 green, 5 blue"),
            CubeSet::new(4, 1, 5)
        );
    }
}
