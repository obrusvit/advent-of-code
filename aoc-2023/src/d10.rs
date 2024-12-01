use crate::utils::*;

type Map = Vec<Vec<Option<Pipe>>>;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: usize, // horizontal ->
    y: usize, // vertical   ↓
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn get_opposite(dir: &Direction) -> Direction {
    use Direction::*;
    match dir {
        North => South,
        South => North,
        East => West,
        West => East,
    }
}

fn go_from(point: &Point, dir: &Direction) -> Point {
    use Direction::*;
    match dir {
        North => Point {
            x: point.x,
            y: point.y - 1,
        },
        South => Point {
            x: point.x,
            y: point.y + 1,
        },
        East => Point {
            x: point.x + 1,
            y: point.y,
        },
        West => Point {
            x: point.x - 1,
            y: point.y,
        },
    }
}
#[derive(Debug, Clone)]
struct Pipe {
    connections: [Direction; 2],
}

impl Pipe {
    fn from_char(c: char) -> Option<Self> {
        use Direction::{East, North, South, West};
        match c {
            '|' => Some(Pipe {
                connections: [North, South],
            }),
            '-' => Some(Pipe {
                connections: [East, West],
            }),
            'L' => Some(Pipe {
                connections: [North, East],
            }),
            'J' => Some(Pipe {
                connections: [North, West],
            }),
            '7' => Some(Pipe {
                connections: [South, West],
            }),
            'F' => Some(Pipe {
                connections: [South, East],
            }),
            _ => None,
        }
    }

    fn get_the_other_dir(&self, came_from: Direction) -> Direction {
        if self.connections[0] == came_from {
            self.connections[1]
        } else {
            self.connections[0]
        }
    }
}

pub fn solve_d10() -> (u32, u32) {
    let lines = read_lines("data/d10.txt");
    assert!(lines.len() > 0);
    assert!(lines[0].len() > 0);
    let n_rows = lines.len();
    let n_cols = lines[0].len();
    let mut map: Map = vec![vec![None; n_cols]; n_rows];
    let mut start_pos = Point { x: 0, y: 0 };
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start_pos = Point { x, y };
            }
            map[y][x] = Pipe::from_char(c);
        }
    }
    let mut loop_counter = 0;
    let mut curr_pos = start_pos.clone();
    let mut curr_dir = Direction::East;
    loop {
        let next_pos = go_from(&curr_pos, &curr_dir);
        if next_pos == start_pos {
            break;
        }
        let next_tube = map[next_pos.y][next_pos.x].as_ref().unwrap();
        loop_counter += 1;

        let came_from = get_opposite(&curr_dir);
        curr_dir = next_tube.get_the_other_dir(came_from);
        curr_pos = next_pos;
    }

    let res_p1 = (loop_counter + 1) / 2;
    // 7102
    (res_p1, 0)
}
