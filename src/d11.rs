use crate::utils::*;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: usize, // horizontal ->
    y: usize, // vertical   ↓
}

impl Point {
    fn zero() -> Self {
        Point { x: 0, y: 0 }
    }
}

fn get_distance(
    p1: Point,
    p2: Point,
    x_expansions: &Vec<usize>,
    y_expansions: &Vec<usize>,
    multiplier: usize,
) -> usize {
    let x_dist = (p1.x as i32 - p2.x as i32).abs() as usize;
    let y_dist = (p1.y as i32 - p2.y as i32).abs() as usize;

    let x_expands = x_expansions
        .iter()
        .filter(|&&num| (p1.x.min(p2.x)..p1.x.max(p2.x)).contains(&num))
        .count();
    let y_expands = y_expansions
        .iter()
        .filter(|&&num| (p1.y.min(p2.y)..p2.y.max(p2.y)).contains(&num))
        .count();

    x_dist + y_dist + x_expands * (multiplier - 1) + y_expands * (multiplier - 1)
}

fn generate_unique_pairs(points: &[Point]) -> Vec<(Point, Point)> {
    let mut pairs = Vec::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            pairs.push((points[i].clone(), points[j].clone()));
        }
    }
    pairs
}

pub fn solve_d11() -> (usize, usize) {
    // load the universe
    let lines = read_lines("data/d11.txt");
    let n_rows = lines.len();
    assert!(n_rows > 0);
    let n_cols = lines[0].len();
    assert!(n_cols > 0);
    let mut galaxy_points: Vec<Point> = Vec::new();
    let mut rows_empty = vec![true; n_rows];
    let mut cols_empty = vec![true; n_cols];

    // find position of galaxies
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                cols_empty[x] = false;
                rows_empty[y] = false;
                galaxy_points.push(Point { x, y });
            }
        }
    }

    // decide on empty rows and columns
    let x_expansions: Vec<usize> = cols_empty
        .iter()
        .enumerate()
        .filter(|&(_idx, &value)| value)
        .map(|(idx, _value)| idx)
        .collect();
    let y_expansions: Vec<usize> = rows_empty
        .iter()
        .enumerate()
        .filter(|&(_idx, &value)| value)
        .map(|(idx, _value)| idx)
        .collect();

    // generate pairs of galaxies
    let pairs = generate_unique_pairs(&galaxy_points);

    // calculate their manhattan distances
    let mut res_p1 = 0;
    let mut res_p2 = 0;
    for pair in pairs {
        res_p1 += get_distance(pair.0, pair.1, &x_expansions, &y_expansions, 2);
        res_p2 += get_distance(pair.0, pair.1, &x_expansions, &y_expansions, 1_000_000);
    }

    (res_p1, res_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_distance_test() {
        assert_eq!(
            get_distance(
                Point { x: 0, y: 0 },
                Point { x: 4, y: 0 },
                &vec![],
                &vec![],
                2
            ),
            4
        );
        assert_eq!(
            get_distance(
                Point { x: 5, y: 0 },
                Point { x: 0, y: 0 },
                &vec![],
                &vec![],
                2
            ),
            5
        );
        assert_eq!(
            get_distance(
                Point { x: 1, y: 1 },
                Point { x: 3, y: 3 },
                &vec![],
                &vec![],
                2
            ),
            4
        );
        assert_eq!(
            get_distance(
                Point { x: 1, y: 1 },
                Point { x: 3, y: 3 },
                &vec![2],
                &vec![],
                2
            ),
            5
        );
        assert_eq!(
            get_distance(
                Point { x: 1, y: 1 },
                Point { x: 3, y: 3 },
                &vec![2],
                &vec![2],
                2
            ),
            6
        );

        assert_eq!(
            get_distance(
                Point { x: 3, y: 0 },
                Point { x: 7, y: 8 },
                &vec![2, 5, 8],
                &vec![3, 7],
                2
            ),
            15
        );

        assert_eq!(
            get_distance(
                Point { x: 1, y: 5 },
                Point { x: 4, y: 9 },
                &vec![2, 5, 8],
                &vec![3, 7],
                2
            ),
            9
        );
    }

    #[test]
    fn get_distance_order_test() {
        assert_eq!(
            get_distance(
                Point { x: 4, y: 0 },
                Point { x: 0, y: 0 },
                &vec![],
                &vec![],
                2
            ),
            get_distance(
                Point { x: 0, y: 0 },
                Point { x: 4, y: 0 },
                &vec![],
                &vec![],
                2
            ),
        );
        assert_eq!(
            get_distance(
                Point { x: 1, y: 5 },
                Point { x: 4, y: 9 },
                &vec![2, 5, 8],
                &vec![3, 7],
                2
            ),
            get_distance(
                Point { x: 4, y: 9 },
                Point { x: 1, y: 5 },
                &vec![2, 5, 8],
                &vec![3, 7],
                2
            )
        );
    }
}
