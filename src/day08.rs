use super::File;
use std::collections::HashMap;

type Grid = HashMap<(i32,i32), u32>;

fn parse_input(input: &str) -> Grid {
    input.trim().lines().enumerate().flat_map(|(y,l)| {
        l.chars().enumerate().map(move |(x,h)| {
            ((x as i32,y as i32), h as u32 - 48)
        })
    }).collect()
}

pub fn star1(file: File) -> Result<usize, anyhow::Error> {
    let grid = parse_input(&file.contents);
    let visible_trees = grid
        .iter()
        .filter(|((x,y), height)| is_visible((*x,*y), **height, &grid))
        .count();

    Ok(visible_trees)
}

pub fn star2(file: File) -> Result<u32, anyhow::Error> {
    let grid = parse_input(&file.contents);
    let most_scenic = grid
        .iter()
        .map(|((x,y), height)| scenic_score((*x,*y), *height, &grid))
        .max()
        .unwrap_or(0);

    Ok(most_scenic)
}

fn is_visible(xy: (i32,i32), height: u32, grid: &Grid) -> bool {
    let left = is_visible_direction(xy, height, grid, |(x,y)| (x - 1, y));
    let right = is_visible_direction(xy, height, grid, |(x,y)| (x + 1, y));
    let up = is_visible_direction(xy, height, grid, |(x,y)| (x, y - 1));
    let down = is_visible_direction(xy, height, grid, |(x,y)| (x, y + 1));

    left || right || up || down
}

fn is_visible_direction<F>(mut xy: (i32,i32), height: u32, grid: &Grid, next: F) -> bool
where F: Fn((i32, i32)) -> (i32,i32)
{
    loop {
        xy = next(xy);
        let Some(&h) = grid.get(&xy) else {
            return true;
        };
        if h >= height {
            return false
        }
    }
}

fn scenic_score(xy: (i32,i32), height: u32, grid: &Grid) -> u32 {
    let left = visible_from_direction(xy, height, grid, |(x,y)| (x - 1, y));
    let right = visible_from_direction(xy, height, grid, |(x,y)| (x + 1, y));
    let up = visible_from_direction(xy, height, grid, |(x,y)| (x, y - 1));
    let down = visible_from_direction(xy, height, grid, |(x,y)| (x, y + 1));

    left * right * up * down
}

fn visible_from_direction<F>(mut xy: (i32,i32), height: u32, grid: &Grid, next: F) -> u32
where F: Fn((i32, i32)) -> (i32,i32)
{
    let mut count = 0;
    loop {
        xy = next(xy);
        let Some(&h) = grid.get(&xy) else {
            return count;
        };
        if h <= height {
            count += 1;
        }
        if h >= height {
            return count;
        }
    }
}