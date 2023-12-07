use crate::utils::*;

fn calc_times_for_d(d: u128, total_time: u128) -> (f64, f64) {
    let discriminant = total_time.pow(2) - 4 * d;
    let t1 = 0.5 * (total_time as f64 - (discriminant as f64).sqrt());
    let t2 = 0.5 * (total_time as f64 + (discriminant as f64).sqrt());
    (t1, t2)
}

pub fn solve_d6() -> (i64, i64) {
    let lines = read_lines("data/d06.txt");
    let times_str = lines[0].split(':').collect::<Vec<&str>>()[1];
    let dists_str = lines[1].split(':').collect::<Vec<&str>>()[1];

    let times = str_array_to_vec::<u128>(times_str);
    let dists = str_array_to_vec::<u128>(dists_str);

    let res1 = times.iter().zip(dists.iter()).fold(1, |acc, (&t, &d)| {
        let (t1, t2) = calc_times_for_d(d, t);
        let ways_to_win = (t2.floor() - t1.ceil()) as i64 + 1;
        acc * ways_to_win
    });

    let time_p2 = str_to::<u128>(&times_str.replace(" ", ""));
    let dist_p2 = str_to::<u128>(&dists_str.replace(" ", ""));

    let (t1, t2) = calc_times_for_d(dist_p2, time_p2);
    let ways_to_win = (t2.floor() - t1.ceil()) as i64 + 1;

    //2269432, 35865985
    (res1, ways_to_win)
}
