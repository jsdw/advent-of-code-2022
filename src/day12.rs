use super::File;
use std::collections::{ HashMap, VecDeque };

struct Map {
    start: (i32,i32),
    end: (i32,i32),
    heights: HashMap<(i32,i32), u32>
}

fn parse_input(input: &str) -> anyhow::Result<Map> {
    let mut start = None;
    let mut end = None;
    let mut heights = HashMap::new();

    for (y, l) in input.trim().lines().enumerate() {
        for (x, mut c) in l.trim().chars().enumerate() {
            let coords = (x as i32, y as i32);
            if c == 'S' {
                start = Some(coords);
                c = 'a';
            } else if c == 'E' {
                end = Some(coords);
                c = 'z';
            }
            heights.insert(coords, c as u32);
        }
    }

    Ok(Map {
        start: start.ok_or_else(|| anyhow::anyhow!("No 'S' start token found"))?,
        end: end.ok_or_else(|| anyhow::anyhow!("no 'E' end token found"))?,
        heights
    })
}

fn steps_map(start: (i32,i32), heights: &HashMap<(i32,i32), u32>) -> HashMap<(i32,i32), usize> {
    let mut steps = HashMap::<(i32,i32), usize>::from_iter([(start, 0)]);
    let mut next = VecDeque::from_iter([start]);

    while let Some((x,y)) = next.pop_front() {
        let curr_steps = *steps.get(&(x,y)).unwrap();
        let curr_height = *heights.get(&(x,y)).unwrap();

        let possible_next = [(x-1, y), (x+1, y), (x, y-1), (x, y+1)]
            .into_iter()
            .filter(|xy| {
                match heights.get(xy) {
                    Some(h) if *h <= curr_height + 1 => true,
                    _ => false,
                }
            });

        for xy in possible_next {
            let e = steps.entry(xy).or_insert(usize::MAX);
            if *e > curr_steps + 1 {
                *e = curr_steps + 1;
                next.push_back(xy);
            }
        }
    }

    steps
}

pub fn star1(file: File) -> Result<usize, anyhow::Error> {
    let Map { start, end, heights } = parse_input(&file.contents)?;
    let steps = steps_map(start, &heights);
    steps.get(&end).copied().ok_or_else(|| anyhow::anyhow!("We didn't make it to the end!"))
}

pub fn star2(file: File) -> Result<usize, anyhow::Error> {
    let Map { start, end, heights } = parse_input(&file.contents)?;
    let min_height = heights.get(&start).unwrap();

    let min_steps = heights
        .iter()
        .filter(|(_, h)| *h == min_height)
        .filter_map(|(xy,_)| {
            let steps = steps_map(*xy, &heights);
            steps.get(&end).copied()
        })
        .min()
        .unwrap();

    Ok(min_steps)
}