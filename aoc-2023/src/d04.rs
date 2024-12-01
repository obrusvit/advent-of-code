use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn str_to_int_array(s: &str) -> Vec<i32> {
    s.trim()
        .split_whitespace()
        .map(|d| d.parse::<i32>().ok().unwrap())
        .collect()
}

fn solve_card(winning: &[i32], mine: &[i32]) -> (i32, i32) {
    let mut points = 0;
    let mut matches = 0;
    for n in winning {
        if mine.contains(n) {
            if matches == 0 {
                points = 1;
            } else {
                points *= 2;
            }
            matches += 1;
        }
    }
    (points, matches)
}
pub fn solve_d4() -> (i32, i32) {
    let lines = read_lines("data/d04.txt");
    let mut res = 0;
    let mut multipliers = vec![1; lines.len()];
    for (idx, line) in lines.iter().enumerate() {
        let parts: Vec<&str> = line.split('|').collect();
        let winning_str: Vec<&str> = parts[0].split(':').collect();
        let winning: Vec<i32> = str_to_int_array(winning_str[1]);
        let mine: Vec<i32> = str_to_int_array(parts[1]);
        let (points, matches) = solve_card(&winning, &mine);
        res += points;
        let base_mult = multipliers[idx];
        for n_match in idx + 1..idx + 1 + matches as usize {
            multipliers[n_match] += base_mult;
        }
        // println!("{idx} -> {matches} -> {:?}", multipliers);
    }

    let res2 = multipliers.iter().sum();
    // 25571, 8805731
    (res, res2)
}
