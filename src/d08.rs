use crate::utils::*;
use std::collections::HashMap;
use num::integer::lcm;

type Map = HashMap<String, (String, String)>;

fn traverse_graph(
    directions: &String,
    map: &Map,
    start_node: &String,
    possible_end_nodes: &Vec<&String>,
) -> u128 {
    let mut res = 0;
    let mut curr = start_node;
    for dir in directions.chars().cycle() {
        let curr_dirs = map.get(curr).unwrap();
        match dir {
            'L' => {
                curr = &curr_dirs.0;
            }
            'R' => {
                curr = &curr_dirs.1;
            }
            _ => unreachable!(),
        }
        res += 1;
        if possible_end_nodes.contains(&curr) {
            break;
        }
    }
    res
}

fn solve_p2(directions: &String, map: &Map) -> u128 {
    let start_nodes: Vec<&String> = map.keys().filter(|k| k.ends_with('A')).collect();
    let end_nodes: Vec<&String> = map.keys().filter(|k| k.ends_with('Z')).collect();
    let mut res = 1;
    for start in start_nodes {
        let r = traverse_graph(directions, map, start, &end_nodes);
        res = lcm(res, r);
    }
    res
}

pub fn solve_d8() -> (u128, u128) {
    let lines = read_lines("data/d08.txt");
    let directions = &lines[0];
    let mut map = Map::new();
    for line in lines.iter().skip(2) {
        let mut sp = line.split('=');
        let src = sp.next().unwrap().trim();
        let mut dirs = sp
            .next()
            .unwrap()
            .trim()
            .trim_matches(|c| c == '(' || c == ')')
            .split(',');
        let dir_left = dirs.next().unwrap();
        let dir_right = dirs.next().unwrap().trim_start();
        map.insert(src.to_owned(), (dir_left.to_owned(), dir_right.to_owned()));
    }

    let res_p1 = traverse_graph(
        &directions,
        &map,
        &"AAA".to_owned(),
        &vec![&"ZZZ".to_owned()],
    );
    let res_p2 = solve_p2(&directions, &map);

    (res_p1, res_p2)
}
