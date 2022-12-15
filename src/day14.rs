use super::File;
use std::collections::HashSet;

pub fn star1(file: File) -> Result<usize, anyhow::Error> {
    let walls = parse_walls(&file.contents);

    let mut sim = Simulation::new(walls);
    while sim.step() {}
    Ok(sim.settled_sand.len())
}

pub fn star2(file: File) -> Result<usize, anyhow::Error> {
    let mut walls = parse_walls(&file.contents);

    // Add a floor that we hope is big enough to handle anything.
    let floor_y = lowest_point(&walls) + 2;
    for xy in iter_line((-500, floor_y), (1500, floor_y)) {
        walls.insert(xy);
    }

    let mut sim = Simulation::new(walls);
    while sim.step() {
        if sim.settled_sand.contains(&(500, 0)) {
            break
        }
    }
    Ok(sim.settled_sand.len())
}

struct Simulation {
    walls: Set,
    moving_sand: (i32,i32),
    settled_sand: Set,
    lowest_wall_y: i32,
}

impl Simulation {
    fn new(walls: Set) -> Self {
        // set the low point below anything else that could happen (including
        // the floor introduced in step 2):
        let lowest_wall_y = lowest_point(&walls) + 10;
        Simulation {
            walls,
            moving_sand: (500, 0),
            settled_sand: Set::new(),
            lowest_wall_y
        }
    }
    fn step(&mut self) -> bool {
        let (x,y) = self.moving_sand;

        if y > self.lowest_wall_y {
            return false;
        } else if !self.blocked((x, y+1)) {
            self.moving_sand = (x, y+1)
        } else if !self.blocked((x-1, y+1)) {
            self.moving_sand = (x-1, y+1);
        } else if !self.blocked((x+1, y+1)) {
            self.moving_sand = (x+1, y+1)
        } else {
            self.settled_sand.insert((x,y));
            self.moving_sand = (500, 0);
        }
        true
    }
    fn blocked(&self, xy: (i32,i32)) -> bool {
        self.walls.get(&xy).is_some() || self.settled_sand.get(&xy).is_some()
    }
}

fn lowest_point(map: &Set) -> i32 {
    map.iter().max_by_key(|(_,y)| y).unwrap().1
}

type Set = HashSet<(i32,i32)>;

fn parse_walls(input: &str) -> Set {
    let mut map = HashSet::new();
    let coords = regex!("([0-9]+),([0-9]+)");
    for line in input.trim().lines() {
        let mut caps = coords.captures_iter(line);
        if let Some(cap) = caps.next() {
            let mut xy = (
                cap.get(1).unwrap().as_str().parse().unwrap(),
                cap.get(2).unwrap().as_str().parse().unwrap(),
            );
            for cap in caps {
                let next_x = cap.get(1).unwrap().as_str().parse().unwrap();
                let next_y = cap.get(2).unwrap().as_str().parse().unwrap();
                for coords in iter_line(xy, (next_x, next_y)) {
                    map.insert(coords);
                }
                xy = (next_x, next_y);
            }
        }

    }
    map
}

fn iter_line(mut curr: (i32,i32), end: (i32,i32)) -> impl Iterator<Item=(i32,i32)> {
    let mut finished = false;
    std::iter::from_fn(move || {
        if finished {
            return None;
        }

        let out = curr;

        if curr == end {
            finished = true;
        } else if curr.0 == end.0 {
            // change y
            if curr.1 < end.1 {
                curr = (curr.0, curr.1 + 1)
            } else {
                curr = (curr.0, curr.1 - 1)
            }
        } else {
            // change x
            if curr.0 < end.0 {
                curr = (curr.0 + 1, curr.1)
            } else {
                curr = (curr.0 - 1, curr.1)
            }
        }

        Some(out)
    })
}