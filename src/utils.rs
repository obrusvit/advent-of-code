use std::fs::read_to_string;

pub fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

pub fn str_to<T: std::str::FromStr>(s: &str) -> T {
    s.parse::<T>().ok().unwrap()
}

pub fn str_array_to_vec<T: std::str::FromStr>(s: &str) -> Vec<T> {
    s.trim()
        .split_whitespace()
        .map(|d| str_to::<T>(d))
        .collect()
}
