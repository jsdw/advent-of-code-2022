use super::File;
use itertools::Itertools;
use std::collections::HashSet;

pub fn star1(file: File) -> Result<u32, anyhow::Error> {
    let score = file.contents.lines().filter_map(|l| {
        let mid = l.len() / 2;
        let fst = &l[0..mid];
        let snd = &l[mid..];

        let fst_set: HashSet<char> = fst.chars().collect();
        let snd_set: HashSet<char> = snd.chars().collect();

        let dupe_n = *fst_set.intersection(&snd_set).next()? as u32;

        if dupe_n >= 97 {
            Some(dupe_n - 97 + 1)
        } else if dupe_n >= 65 {
            Some(dupe_n - 65 + 27)
        } else {
            None
        }
    }).sum();

    Ok(score)
}

pub fn star2(file: File) -> Result<u32, anyhow::Error> {
    let score = file.contents.lines().chunks(3).into_iter().filter_map(|mut c| {
        let a: HashSet<char> = c.next()?.chars().collect();
        let b: HashSet<char> = c.next()?.chars().collect();
        let c: HashSet<char> = c.next()?.chars().collect();

        // Yes, more efficient ways to do all of this set stuff, but for the sake of getting it done...
        let dupe_n = *a.intersection(&b).cloned().collect::<HashSet<char>>().intersection(&c).next()? as u32;

        if dupe_n >= 97 {
            Some(dupe_n - 97 + 1)
        } else if dupe_n >= 65 {
            Some(dupe_n - 65 + 27)
        } else {
            None
        }
    }).sum();

    Ok(score)
}