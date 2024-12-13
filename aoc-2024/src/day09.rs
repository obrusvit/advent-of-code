use crate::Solution;

pub struct Day09;

#[derive(Debug, Clone)]
struct Block {
    file_id: Option<usize>,
}

struct Disk {
    blocks: Vec<Block>,
}

impl Disk {
    fn from(s: &str) -> Self {
        let mut blocks = Vec::new();
        let mut is_file = true;
        let mut file_id = 0;
        for c in s.chars() {
            let Some(digit) = c.to_digit(10) else { break };
            if is_file {
                for _ in 0..digit {
                    blocks.push(Block {
                        file_id: Some(file_id),
                    });
                }
                file_id += 1;
            } else {
                for _ in 0..digit {
                    blocks.push(Block { file_id: None });
                }
            }
            is_file = !is_file;
        }
        Self { blocks }
    }

    fn find_first_empty(&self) -> usize {
        self.blocks
            .iter()
            .position(|b| b.file_id.is_none())
            .unwrap()
    }

    fn defragment(&mut self) {
        let mut i = self.blocks.len();
        while i > 0 {
            i -= 1;
            if self.blocks[i].file_id.is_some() {
                let j = self.find_first_empty();
                if j > i {
                    break;
                }
                self.blocks.swap(i, j);
            }
        }
    }

    fn defragment2(&mut self) {
        let mut pos = self.blocks.len() - 1;
        let mut file_id = self.blocks[pos].file_id.unwrap_or(0);

        while pos > 0 {
            // Find start of current file
            while pos > 0 && self.blocks[pos].file_id != Some(file_id) {
                pos -= 1;
            }
            if self.blocks[pos].file_id != Some(file_id) {
                file_id -= 1;
                pos = self.blocks.len() - 1;
                continue;
            }

            // Calculate file size
            let file_end = pos;
            while pos > 0 && self.blocks[pos - 1].file_id == Some(file_id) {
                pos -= 1;
            }
            let file_start = pos;
            let file_size = file_end - file_start + 1;

            // Find gap from the beginning up to file_start
            let mut gap_len = 0;
            let mut gap_pos = 0;
            let mut best_gap = None;

            while gap_pos < file_start {
                if self.blocks[gap_pos].file_id.is_none() {
                    gap_len += 1;
                    if gap_len >= file_size {
                        best_gap = Some(gap_pos - gap_len + 1);
                    }
                } else {
                    gap_len = 0;
                }
                gap_pos += 1;
            }

            // Move file if gap found
            if let Some(gap_start) = best_gap {
                for i in 0..file_size {
                    self.blocks[gap_start + i] = self.blocks[file_start + i].clone();
                    self.blocks[file_start + i].file_id = None;
                }
            }

            file_id -= 1;
            pos = self.blocks.len() - 1;
        }
    }

    fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .map(|(i, block)| i * block.file_id.unwrap_or(0))
            .sum()
    }
}

impl Solution for Day09 {
    fn part1(&self, input: &str) -> String {
        let mut disk = Disk::from(input);
        disk.defragment();
        disk.checksum().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut disk = Disk::from(input);
        disk.defragment2();
        disk.checksum().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: &str = "2333133121414131402";
        let result = Day09.part1(input);
        assert_eq!(result, 1928.to_string());
    }

    #[test]
    fn test_part2() {
        let input: &str = "2333133121414131402";
        let result = Day09.part2(input);
        assert_eq!(result, 2858.to_string());
    }
}
