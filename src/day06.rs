use super::File;
use std::collections::HashSet;

pub fn star1(file: File) -> Result<usize, anyhow::Error> {
    distinct_chars_at(&file.contents.as_bytes(), 4)
        .ok_or_else(|| anyhow::anyhow!("Didn't find 4 different chars"))
}

pub fn star2(file: File) -> Result<usize, anyhow::Error> {
    distinct_chars_at(&file.contents.as_bytes(), 14)
        .ok_or_else(|| anyhow::anyhow!("Didn't find 14 different chars"))
}

fn distinct_chars_at(input: &[u8], len: usize) -> Option<usize> {
    for (chars,idx) in input.windows(len).zip(len..) {
        if HashSet::<&u8>::from_iter(chars).len() == len {
            return Some(idx)
        }
    }
    None
}