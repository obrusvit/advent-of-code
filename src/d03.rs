fn read_lines(filename: &str) -> Vec<String> {
    use std::fs::read_to_string;
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
struct Point {
    x: usize, // horizontal ->
    y: usize, // vertical   ↓
}
const X_MAX: usize = 139;
const Y_MAX: usize = 139;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Gear {
    ratio: i32,
    n_numbers: i32,
}

fn get_envelope(left: &Point, right: &Point) -> Vec<Point> {
    assert!(left.y == right.y);
    use itertools::Itertools;
    let top_left = Point {
        x: left.x.saturating_sub(1),
        y: left.y.saturating_sub(1),
    };
    let bot_right = Point {
        x: right.x.saturating_add(1).min(X_MAX),
        y: right.y.saturating_add(1).min(Y_MAX),
    };
    let res = (top_left.x..=bot_right.x)
        .cartesian_product(top_left.y..=bot_right.y)
        .filter(|(x, y)| !(y == &left.y && &left.x <= x && x <= &right.x))
        .map(|(x, y)| Point { x, y })
        .collect::<Vec<Point>>();
    res
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_numeric()
}
fn is_gear(c: char) -> bool {
    c == '*'
}

pub fn solve_d3() -> (i32, i32) {
    use regex::Regex;
    use std::collections::HashMap;
    let mut res = 0;
    let mut gears_seen = HashMap::<Point, Gear>::new();
    let lines = read_lines("data/d03.txt");
    for (y, line) in lines.iter().enumerate() {
        // println!("{y} -> {line}");
        let re_num = Regex::new(r"(\d+)").unwrap();
        for number_match in re_num.find_iter(&line) {
            let number = &line[number_match.start()..number_match.end()];
            let location = (number_match.start(), number_match.end() - 1);
            let envelope = get_envelope(&Point { x: location.0, y }, &Point { x: location.1, y });
            let mut summed = false;
            for point in envelope {
                let c = lines[point.y].chars().nth(point.x).unwrap();
                let n = number.parse::<i32>().ok().unwrap_or(0);
                // println!("({n}) {:?} -> {}", point, c);
                if !summed && is_symbol(c) {
                    res += n;
                    summed = true;
                    // println!("summing: {number}, ({res})");
                }
                if is_gear(c) {
                    gears_seen
                        .entry(point)
                        .and_modify(|g| {
                            g.ratio *= n;
                            g.n_numbers += 1;
                        })
                        .or_insert_with(|| Gear {
                            ratio: n,
                            n_numbers: 1,
                        });
                }
            }
        }
    }
    let res2 = gears_seen
        .iter()
        .filter(|(_k, v)| v.n_numbers == 2)
        .fold(0, |acc, (_k, v)| acc + v.ratio);
    // 553825, 93994191
    (res, res2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_envelope_test() {
        assert_eq!(
            get_envelope(&Point { x: 0, y: 0 }, &Point { x: 2, y: 0 }).sort(),
            vec![
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 1 },
                Point { x: 3, y: 1 },
                Point { x: 3, y: 0 }
            ]
            .sort()
        );
        assert_eq!(
            get_envelope(&Point { x: 1, y: 1 }, &Point { x: 4, y: 1 }).sort(),
            vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 3, y: 0 },
                Point { x: 4, y: 0 },
                Point { x: 5, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 5 },
                Point { x: 0, y: 2 },
                Point { x: 1, y: 2 },
                Point { x: 2, y: 2 },
                Point { x: 3, y: 2 },
                Point { x: 4, y: 2 },
                Point { x: 5, y: 2 }
            ]
            .sort()
        );
    }

    #[test]
    fn is_symbol_test() {
        assert_eq!(is_symbol('@'), true);
        assert_eq!(is_symbol('%'), true);
        assert_eq!(is_symbol('!'), true);
        assert_eq!(is_symbol('?'), true);

        assert_eq!(is_symbol('.'), false);
        assert_eq!(is_symbol('4'), false);
        assert_eq!(is_symbol('9'), false);
    }
}

