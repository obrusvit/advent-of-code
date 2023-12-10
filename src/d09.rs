use crate::utils::*;

fn get_difference_array(seq: &Vec<i64>) -> Vec<i64> {
    let res_len = seq.len() - 1;
    (0..res_len).fold(Vec::with_capacity(res_len), |mut acc, idx| {
        acc.push(seq[idx + 1] - seq[idx]);
        acc
    })
}

fn extrapolate_sequence(seq: &Vec<i64>) -> (i64, i64) {
    if seq.iter().all(|&el| el == 0) {
        return (0, 0);
    }
    let diff_array = get_difference_array(seq);
    let (ex_front, ex_back) = extrapolate_sequence(&diff_array);

    let extrapolate_front = seq.iter().next().unwrap() - ex_front;
    let extrapolate_back = seq.iter().last().unwrap() + ex_back;
    (extrapolate_front, extrapolate_back)
}
pub fn solve_d9() -> (i64, i64) {
    let lines = read_lines("data/d09.txt");
    let mut res_p1 = 0;
    let mut res_p2 = 0;
    for line in lines.iter() {
        let seq = str_array_to_vec::<i64>(line);
        let (res_front, res_back) = extrapolate_sequence(&seq);
        res_p1 += res_back;
        res_p2 += res_front;
    }
    //2075724761, 1072
    (res_p1, res_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_difference_array_test() {
        assert_eq!(
            get_difference_array(&vec![0, 3, 6, 9, 12, 15]),
            vec![3, 3, 3, 3, 3]
        );
    }
    #[test]
    fn extrapolate_sequence_test() {
        assert_eq!(extrapolate_sequence(&vec![0, 3, 6, 9, 12, 15]), (-3, 18));
        assert_eq!(extrapolate_sequence(&vec![1, 3, 6, 10, 15, 21]), (0, 28));
        assert_eq!(extrapolate_sequence(&vec![10, 13, 16, 21, 30, 45]), (5, 68));
    }
}
