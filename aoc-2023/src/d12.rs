use crate::utils::*;

fn create_string_from_vec_char(chars: &mut Vec<char>) -> String {
    let mut res = String::with_capacity(chars.len());
    for c in chars {
        res.push(c.to_owned());
    }
    res
}

fn count_spring_groups(s: &str) -> Vec<usize> {
    s.split('.')
        .filter(|el| !el.is_empty())
        .map(|el| el.len())
        .collect()
}

fn count_spring_groups_chars(groups: &mut Vec<usize>, chars: &Vec<char>){
    groups.clear();
    let mut count = 0;

    for &ch in chars {
        if ch == '.' {
            if count > 0 {
                groups.push(count);
                count = 0;
            }
        } else {
            count += 1;
        }
    }

    if count > 0 {
        groups.push(count);
    }

}

fn solve_spring_map(s: &str) -> usize {
    // println!("{s}");
    let parts: Vec<&str> = s.split(' ').collect();
    assert_eq!(parts.len(), 2);
    let mut map: Vec<char> = parts[0].chars().collect();
    let mut groups: Vec<usize> = Vec::with_capacity(10);
    let target = str_array_to_vec::<usize>(parts[1]);

    fn backtrace(m: &mut Vec<char>, g: &mut Vec<usize>, t: &Vec<usize>, curr_idx: usize, counter: &mut usize) {
        // println!("backtrace, idx: {:>2}, evaluating: {:?}", curr_idx, m);
        if curr_idx == m.len() {
            // println!("backtrace, end,      evaluating {:?}", m);
            // let s_final = create_string_from_vec_char(m);
            // if &count_spring_groups(&s_final) == t {
            count_spring_groups_chars(g, &m);
            if g == t {
                *counter += 1;
                // println!("counting UP");
            }
            return;
        }
        if m[curr_idx] == '?' {
            m[curr_idx] = '.';
            backtrace(m, g, t, curr_idx + 1, counter);
            // println!("backtrace, if branch, idx: {curr_idx}, evaluating: {:?}", m);
            m[curr_idx] = '#';
            // println!("backtrace, if branch, idx: {curr_idx}, evaluating: {:?}", m);
            backtrace(m, g, t, curr_idx + 1, counter);
            m[curr_idx] = '?';
            // backtrace(m, t, curr_idx + 1, counter);
        } else {
            backtrace(m, g, t, curr_idx + 1, counter);
        }
    }
    let mut res = 0;
    backtrace(&mut map, &mut groups, &target, 0, &mut res);

    res
}

pub fn solve_d12() -> (usize, usize) {
    let lines = read_input(12);
    // let lines = read_test_input(12,1);
    let mut res_p1 = 0;
    let mut res_p2 = 0;
    for line in lines {
        println!("{line}");
        res_p1 += solve_spring_map(&line);
        // res_p2 += solve_part2(&line);
    }

    (res_p1, res_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_spring_groups_test() {
        assert_eq!(count_spring_groups("#.#.###"), vec![1, 1, 3]);
        assert_eq!(count_spring_groups(".#.###.#.######"), vec![1, 3, 1, 6]);
        assert_eq!(count_spring_groups("####.#...#..."), vec![4, 1, 1]);
        assert_eq!(count_spring_groups("#....######..#####."), vec![1, 6, 5]);
        assert_eq!(count_spring_groups(".###.##....#"), vec![3, 2, 1]);
    }

    #[test]
    fn solve_spring_map_test() {
        assert_eq!(solve_spring_map("??? 1,1"), 1);

        assert_eq!(solve_spring_map("???.### 1,1,3"), 1);
        assert_eq!(solve_spring_map(".??..??...?##. 1,1,3"), 4);
        assert_eq!(solve_spring_map("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(solve_spring_map("????.#...#... 4,1,1"), 1);
        assert_eq!(solve_spring_map("????.######..#####. 1,6,5"), 4);
        assert_eq!(solve_spring_map("?###???????? 3,2,1"), 10);
    }
}

