use std::fs::read_to_string;


pub fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

pub fn read_input(day: usize) -> Vec<String> {
    // std::fs::read_to_string(format!("./data/day{:0>2}.txt", day)).unwrap()
    read_lines(format!("./data/d{:0>2}.txt", day).as_ref())
}

pub fn read_test_input(day: usize, n: usize) -> Vec<String> {
    // std::fs::read_to_string(format!("./data/day{:0>2}.txt", day)).unwrap()
    read_lines(format!("./data/d{:0>2}_test{}.txt", day, n).as_ref())
}

pub fn str_to<T: std::str::FromStr>(s: &str) -> T {
    s.parse::<T>().ok().unwrap()
}

pub fn str_array_to_vec<T: std::str::FromStr>(s: &str) -> Vec<T> {
    s.trim()
        .split(|c| c == ',' || c == ' ')
        .map(|d| str_to::<T>(d))
        .collect()
}
