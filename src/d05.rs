use std::collections::HashMap;
use std::fs::read_to_string;
#[derive(Debug, Eq, Hash, PartialEq)]
struct Range {
    start: i64,
    end: i64,
}
type Map = HashMap<Range, Range>;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn str_to_int_array(s: &str) -> Vec<i64> {
    s.trim()
        .split_whitespace()
        .map(|d| d.parse::<i64>().ok().unwrap())
        .collect()
}

fn process_line_to_map(map: &mut Map, s: &str) {
    // println!("processing {s} to {:?}", map);
    // the destination range start, the source range start, and the range length.
    let vec = str_to_int_array(s);
    let destination_start = vec[0];
    let source_start = vec[1];
    let range_len = vec[2];
    map.insert(
        Range {
            start: source_start,
            end: source_start + range_len,
        },
        Range {
            start: destination_start,
            end: destination_start + range_len,
        },
    );
}

fn get_from_map(map: &Map, source_value: i64) -> i64 {
    if let Some(pair_found) = map
        .iter()
        .find(|(k, _v)| k.start <= source_value && source_value <= k.end)
    {
        pair_found.1.start + (source_value - pair_found.0.start)
    } else {
        source_value
    }
}

fn get_from_map_rev(map: &Map, destination_value: i64) -> i64 {
    if let Some(pair_found) = map
        .iter()
        .find(|(_k, v)| v.start <= destination_value && destination_value <= v.end)
    {
        pair_found.0.start + (destination_value - pair_found.1.start)
    } else {
        destination_value
    }
}
fn is_seed_in_range(seeds: &Vec<i64>, seed: i64) -> bool {
    for i in 0..seeds.len() - 1 {
        if i % 2 != 0 {
            continue;
        }

        let start = seeds[i];
        let end = seeds[i] + seeds[i + 1];
        if start <= seed && seed <= end {
            return true;
        }
    }
    false
}

pub fn solve_d5() -> (i64, i64) {
    let lines = read_lines("data/d05.txt");

    // parse the maps
    let mut seeds = Vec::new();
    let mut seed_2_soil = Map::new();
    let mut soil_2_fertilizer = Map::new();
    let mut fertilizer_2_water = Map::new();
    let mut water_2_light = Map::new();
    let mut light_2_temperature = Map::new();
    let mut temperature_2_humidity = Map::new();
    let mut humidity_2_location = Map::new();
    let mut current_map = &mut seed_2_soil;
    for (idx, line) in lines.iter().enumerate() {
        if idx == 0 {
            let line_1_split: Vec<&str> = lines[0].split(':').collect();
            for i in str_to_int_array(line_1_split[1]) {
                seeds.push(i);
            }
            continue;
        }
        let lstr = line.as_str();
        match lstr {
            "seed-to-soil map:" => current_map = &mut seed_2_soil,
            "soil-to-fertilizer map:" => current_map = &mut soil_2_fertilizer,
            "fertilizer-to-water map:" => current_map = &mut fertilizer_2_water,
            "water-to-light map:" => current_map = &mut water_2_light,
            "light-to-temperature map:" => current_map = &mut light_2_temperature,
            "temperature-to-humidity map:" => current_map = &mut temperature_2_humidity,
            "humidity-to-location map:" => current_map = &mut humidity_2_location,
            "" => continue,
            _ => {
                process_line_to_map(&mut current_map, lstr);
            }
        }
    }
    // println!("{:?}", seeds);

    // find the location for each seed
    let mut res1 = i64::MAX;
    for &seed in seeds.iter() {
        let soil = get_from_map(&seed_2_soil, seed);
        let fertilizer = get_from_map(&soil_2_fertilizer, soil);
        let water = get_from_map(&fertilizer_2_water, fertilizer);
        let light = get_from_map(&water_2_light, water);
        let temperature = get_from_map(&light_2_temperature, light);
        let humidity = get_from_map(&temperature_2_humidity, temperature);
        let location = get_from_map(&humidity_2_location, humidity);
        if location < res1 {
            res1 = location;
        }
    }

    let mut loc_cand = 0;
    loop {
        let humidity = get_from_map_rev(&humidity_2_location, loc_cand);
        let temperature = get_from_map_rev(&temperature_2_humidity, humidity);
        let light = get_from_map_rev(&light_2_temperature, temperature);
        let water = get_from_map_rev(&water_2_light, light);
        let fertilizer = get_from_map_rev(&fertilizer_2_water, water);
        let soil = get_from_map_rev(&soil_2_fertilizer, fertilizer);
        let seed = get_from_map_rev(&seed_2_soil, soil);
        let seed_found = is_seed_in_range(&seeds, seed);
        if seed_found {
            // println!("found(b): {seed} -> {loc_cand}");
            break;
        }
        loc_cand += 1;
    }
    (res1, loc_cand)
}
