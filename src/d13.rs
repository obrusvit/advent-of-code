use crate::utils::*;

type Map = Vec<Vec<char>>;

fn get_other_terrain(t: char) -> char {
    match t {
        '.' => '#',
        '#' => '.',
        _ => t,
    }
}

/// Returns true if mirror is found at indices ((pos-1), pos)
fn is_mirror_at_pos<T: PartialEq>(line: &Vec<T>, pos: usize) -> bool {
    if pos == 0 || pos >= line.len() {
        return false;
    }
    (pos..line.len())
        .zip((0..pos).rev())
        .all(|(b, f)| line[f] == line[b])
}

fn scan_mirrors(map: &Map, skip: Option<usize>) -> Option<usize> {
    assert!(map.len() > 0);
    assert!(map[0].len() > 0);
    // scan vertical direction, i.e. by rows
    for idx in 0..map.len() {
        if is_mirror_at_pos(&map, idx) {
            let res = 100 * idx;
            match skip {
                None => return Some(res),
                Some(s) => {
                    if s != res {
                        return Some(res);
                    }
                }
            }
        }
    }

    // scan horizontal direction, i.e. by columns
    'outer: for idx in 0..map[0].len() {
        if is_mirror_at_pos(&map[0], idx) {
            for j in 1..map.len() {
                if !is_mirror_at_pos(&map[j], idx) {
                    continue 'outer;
                }
            }
            match skip {
                None => return Some(idx),
                Some(s) => {
                    if s != idx {
                        return Some(idx);
                    }
                }
            }
        }
    }
    None
}

fn print_map(map: &Map) {
    println!("print_map");
    for l in map {
        println!("{:?}", l);
    }
    println!("");
}

fn scan_mirrors_p2(map: &Map, skip: Option<usize>) -> Option<usize> {
    assert!(map.len() > 0);
    assert!(map[0].len() > 0);
    print_map(map);
    let mut map_clone = map.clone();
    let res_p1 = scan_mirrors(&map_clone, None).unwrap();
    for i in 0..map.len() {
        let curr_line = map[i].clone();
        for j in 0..map[0].len() {
            let mut curr_line_cp = curr_line.clone();
            curr_line_cp[j] = get_other_terrain(curr_line_cp[j]);
            map_clone[i] = curr_line_cp;
            if let Some(d) = scan_mirrors(&map_clone, skip) {
                return Some(d);
            }
        }
        map_clone[i] = curr_line;
    }
    None
}

pub fn solve_d13() -> (usize, usize) {
    let lines = read_input(13);
    // let lines = read_test_input(13, 1);
    let mut curr_map = Map::new();
    let mut res_p1 = 0;
    let mut res_p2 = 0;
    for (idx, line) in lines.iter().enumerate() {
        if line.is_empty() || idx == lines.len() - 1 {
            let p1 = scan_mirrors(&curr_map, None);
            let p2 = scan_mirrors_p2(&curr_map, p1);
            res_p1 += p1.unwrap();
            res_p2 += p2.unwrap();
            curr_map.clear();
            continue;
        }
        curr_map.push(line.chars().collect());
    }

    // 34911, 33183
    (res_p1, res_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec(s: &str) -> Vec<char> {
        s.chars().collect::<Vec<_>>()
    }

    #[test]
    fn is_mirror_at_pos_test() {
        assert!(is_mirror_at_pos(&to_vec("##"), 1));
        assert!(!is_mirror_at_pos(&to_vec("##"), 0));
        assert!(!is_mirror_at_pos(&to_vec("##"), 2));

        assert!(is_mirror_at_pos(&to_vec("#..#"), 2));
        assert!(!is_mirror_at_pos(&to_vec("#..#"), 1));
        assert!(!is_mirror_at_pos(&to_vec("#..#"), 0));

        assert!(is_mirror_at_pos(&to_vec("#.##..##."), 5));
        assert!(!is_mirror_at_pos(&to_vec("#.##..##."), 4));
    }

    #[test]
    fn is_vert_mirror_at_pos_test() {
        assert!(is_mirror_at_pos(
            &vec![&to_vec("###..."), &to_vec("###...")],
            1
        ));
        assert!(!is_mirror_at_pos(
            &vec!["#.#...".to_owned(), "###...".to_owned()],
            1
        ));
    }
}
