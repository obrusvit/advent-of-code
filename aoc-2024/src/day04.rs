use crate::Solution;

pub struct Day04;

const DIRECTIONS: &[(i32, i32)] = &[
    (0, -1),
    (0, 1),
    (-1, 0),
    (1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

fn get_element(map: &Vec<Vec<char>>, x: i32, y: i32) -> char {
    if x < 0 || x >= map[0].len() as i32 || y < 0 || y >= map.len() as i32 {
        return '.';
    }
    map[y as usize][x as usize]
}

impl Solution for Day04 {
    fn part1(&self, input: &str) -> String {
        const PATTERN: &str = "XMAS";
        let mut res = 0;
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        for (y, row) in map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == PATTERN[0..1].chars().next().unwrap() {
                    for dir in DIRECTIONS {
                        for i in 1..PATTERN.len() {
                            let new_x = x as i32 + dir.0 * i as i32;
                            let new_y = y as i32 + dir.1 * i as i32;
                            let el = get_element(&map, new_x, new_y);
                            if el != PATTERN[i..i + 1].chars().next().unwrap() {
                                break;
                            }
                            if i == PATTERN.len() - 1 {
                                res += 1;
                            }
                        }
                    }
                }
            }
        }
        res.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut res = 0;
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        for (y, row) in map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == 'A' {
                    let c1_d1 = get_element(&map, x as i32 - 1, y as i32 - 1);
                    let c2_d1 = get_element(&map, x as i32 + 1, y as i32 + 1);
                    let c1_d2 = get_element(&map, x as i32 - 1, y as i32 + 1);
                    let c2_d2 = get_element(&map, x as i32 + 1, y as i32 - 1);
                    let diag1 = &[c1_d1, c2_d1];
                    let diag2 = &[c1_d2, c2_d2];
                    if diag1.contains(&'M')
                        && diag1.contains(&'S')
                        && diag2.contains(&'M')
                        && diag2.contains(&'S')
                    {
                        res += 1;
                    }
                }
            }
        }
        res.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let result = Day04.part1(input);
        assert_eq!(result, 18.to_string());
    }

    #[test]
    fn test_part2() {
        let input: &str = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        let result = Day04.part2(input);
        assert_eq!(result, 9.to_string());
    }
}
