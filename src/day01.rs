use super::File;
use std::collections::BinaryHeap;

pub fn star1(file: File) -> Result<usize, anyhow::Error> {
    let mut max: usize = 0;
    let mut current: usize = 0;

    for line in file.contents.lines().map(|l| l.trim()) {
        if line.is_empty() {
            if current >= max {
                max = current;
            }
            current = 0;
            continue
        }
        current += line.parse::<usize>()?;
    }

    Ok(max)
}

pub fn star2(file: File) -> Result<usize, anyhow::Error> {
    let mut maxes: BinaryHeap<usize> = BinaryHeap::new();
    let mut current: usize = 0;

    for line in file.contents.lines().map(|l| l.trim()) {
        if line.is_empty() {
            maxes.push(current);
            current = 0;
            continue
        }
        current += line.parse::<usize>()?;
    }

    let top3 = maxes.into_iter().take(3).sum();
    Ok(top3)
}