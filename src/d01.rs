use std::fs::read_to_string;

fn parse_number(s: &str) -> Option<char> {
    let max_idx = s.len();
    let num = match &s[..3.min(max_idx)] {
        "one" => Some('1'),
        "two" => Some('2'),
        "six" => Some('6'),
        _ => match &s[..4.min(max_idx)] {
            "four" => Some('4'),
            "five" => Some('5'),
            "nine" => Some('9'),
            _ => match &s[..5.min(max_idx)] {
                "three" => Some('3'),
                "seven" => Some('7'),
                "eight" => Some('8'),
                _ => None,
            },
        },
    };
    num
}

fn process_line_p2(line: &str) -> i32 {
    let mut n1 = None;
    let mut n2 = None;
    for (i, c) in line.chars().into_iter().enumerate() {
        if c.is_numeric() {
            if n1 == None {
                n1 = Some(c);
            }
            n2 = Some(c);
        } else if let Some(d) = parse_number(&line[i..]) {
            if n1 == None {
                n1 = Some(d);
            }
            n2 = Some(d);
        } else {
            continue;
        }
    }
    parse_int(n1.unwrap(), n2.unwrap())
}

fn process_line(line: &str) -> i32 {
    let n1 = line.chars().into_iter().find(|c| c.is_numeric()).unwrap();
    let n2 = line
        .chars()
        .into_iter()
        .rev()
        .find(|c| c.is_numeric())
        .unwrap();
    parse_int(n1, n2)
}

fn parse_int(c1: char, c2: char) -> i32 {
    format!("{c1}{c2}").parse::<i32>().unwrap()
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

pub fn solve_d1() -> (i32, i32) {
    let vec = read_lines("data/d01.txt");
    let mut res_p1: i32 = 0;
    let mut res_p2: i32 = 0;
    for el in vec {
        res_p1 += process_line(&el);
        res_p2 += process_line_p2(&el);
    }
    (res_p1, res_p2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number(&"one44444444"), Some('1'));
        assert_eq!(parse_number(&"nine"), Some('9'));
    }
    #[test]
    fn test_process_line() {
        assert_eq!(process_line(&"1abc2"), 12);
        assert_eq!(process_line(&"pqr3stu8vwx"), 38);
        assert_eq!(process_line(&"a1b2c3d4e5f"), 15);
        assert_eq!(process_line(&"treb7uchet"), 77);
    }
    #[test]
    fn test_process_line_p2() {
        assert_eq!(process_line_p2(&"two1one1"), 21);
        assert_eq!(process_line_p2(&"two1one"), 21);
        assert_eq!(process_line_p2(&"nine"), 99);
        assert_eq!(process_line_p2(&"two1nine"), 29);
        assert_eq!(process_line_p2(&"eightwothree"), 83);
        assert_eq!(process_line_p2(&"abcone2threexyz"), 13);
        assert_eq!(process_line_p2(&"xtwone3four"), 24);
        assert_eq!(process_line_p2(&"4nineeightseven2"), 42);
        assert_eq!(process_line_p2(&"zoneight234"), 14);
        assert_eq!(process_line_p2(&"7pqrstsixteen"), 76);
    }
}
