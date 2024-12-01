use crate::utils::*;

#[derive(Debug, Hash, Clone)]
struct Box<'a> {
    lenses: Vec<Lens<'a>>,
}

impl<'a> Box<'a> {
    fn new() -> Self {
        Self { lenses: Vec::new() }
    }
}
#[derive(Debug, Hash, Clone)]
struct Lens<'a> {
    label: &'a str,
    len: u8,
}

impl<'a> Lens<'a> {
    fn new(label: &'a str, len: u8) -> Self {
        Self { label, len }
    }
}

fn hash(s: &str) -> u8 {
    let mut res: u16 = 0;
    for c in s.chars() {
        let ascii = c as u8;
        res += ascii as u16;
        res *= 17;
        res %= 256;
    }
    res as u8
}

pub fn solve_d15() -> (usize, usize) {
    let lines = read_input(15);
    // let lines = read_test_input(15, 1);
    assert_eq!(lines.len(), 1);
    let input = &lines[0];
    let mut res_p1 = 0;
    let mut boxes = vec![Box::new(); 256];
    for command in input.split(',') {
        res_p1 += hash(command) as usize;
        let cmd_parts = command.split(|c| c == '=' || c == '-').collect::<Vec<_>>();
        let lens_label = cmd_parts[0];
        let box_id = hash(lens_label);

        let cmd_part2 = cmd_parts[1];
        let box_in_question = &mut boxes[box_id as usize];
        if cmd_part2.is_empty() {
            // erase operation
            box_in_question.lenses.retain(|b| b.label != lens_label);
        } else {
            // add operation
            let lens_len = str_to::<u8>(cmd_part2);
            let new_lens = Lens::new(lens_label, lens_len);
            if let Some(existing_lens) = box_in_question
                .lenses
                .iter_mut()
                .find(|lens| lens.label == lens_label)
            {
                existing_lens.len = lens_len;
            } else {
                box_in_question.lenses.push(new_lens);
            }
        }
    }
    let mut res_p2 = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (ii, lens) in b.lenses.iter().enumerate() {
            res_p2 += (i + 1) * (ii + 1) * lens.len as usize;
        }
    }
    (res_p1, res_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_test() {
        assert_eq!(hash(&"HASH"), 52);
        assert_eq!(hash(&"rn=1"), 30);
        assert_eq!(hash(&"cm-"), 253);
        assert_eq!(hash(&"qp=3"), 97);
        assert_eq!(hash(&"cm=2"), 47);

        assert_eq!(hash(&"rn"), 0);
        assert_eq!(hash(&"cm"), 0);
        assert_eq!(hash(&"qp"), 1);
    }
}
