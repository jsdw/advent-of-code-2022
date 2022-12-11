use super::File;
use yap::{ IntoTokens, Tokens };
use std::collections::HashSet;
use std::cell::Cell;
use itertools::Itertools;

pub fn star1(file: File) -> Result<usize, anyhow::Error> {
    let mut seen_tail_pos = HashSet::<(i32,i32)>::from_iter([(0,0)]);
    let mut head_pos = (0,0);
    let mut tail_pos = (0,0);
    for (dir, count) in parse_input(&file.contents) {
        for _ in 0..count {
            head_pos = dir.nudge(head_pos);
            tail_pos = move_tail(tail_pos, head_pos);
            seen_tail_pos.insert(tail_pos);
        }
    }
    Ok(seen_tail_pos.len())
}

pub fn star2(file: File) -> Result<usize, anyhow::Error> {
    let mut seen_tail_pos = HashSet::<(i32,i32)>::from_iter([(0,0)]);
    let mut head_pos = (0,0);
    let tails = vec![Cell::new((0,0)); 9];
    for (dir, count) in parse_input(&file.contents) {
        for _ in 0..count {
            head_pos = dir.nudge(head_pos);
            tails[0].set(move_tail(tails[0].get(), head_pos));

            for (h,t) in tails.iter().tuple_windows() {
                t.set(move_tail(t.get(), h.get()));
            }

            seen_tail_pos.insert(tails.last().unwrap().get());
        }
    }
    Ok(seen_tail_pos.len())
}

// This could be much less verbose, but oh well.
fn move_tail((tx,ty): (i32,i32), (hx,hy): (i32,i32)) -> (i32,i32) {
    // same; leave
    if hx == tx && hy == ty { (tx,ty) }

    // leave diagonals alone too
    else if hx-1 == tx && hy-1 == ty { (tx,ty) }
    else if hx+1 == tx && hy-1 == ty { (tx,ty) }
    else if hx-1 == tx && hy+1 == ty { (tx,ty) }
    else if hx+1 == tx && hy+1 == ty { (tx,ty) }

    // move for horizontal and vertical
    else if tx == hx { (tx, pull_1d(ty, hy)) }
    else if ty == hy { (pull_1d(tx, hx), ty) }

    // handle "2-away" diagonals with a diagonal move. This
    // only happens when lots of tails, because while the head
    // can never move diagonally, any tail can.
    else if hx-2 == tx && hy-2 == ty { (hx-1,hy-1) }
    else if hx-2 == tx && hy+2 == ty { (hx-1,hy+1) }
    else if hx+2 == tx && hy+2 == ty { (hx+1,hy+1) }
    else if hx+2 == tx && hy-2 == ty { (hx+1,hy-1) }

    // move to the side of the head for further diagonals
    else if hx-1 == tx { (hx, pull_1d(ty, hy)) }
    else if hx+1 == tx { (hx, pull_1d(ty, hy)) }
    else if hy-1 == ty { (pull_1d(tx, hx), hy) }
    else if hy+1 == ty { (pull_1d(tx, hx), hy) }

    // That should cover all valid moves
    else { panic!("invalid move from ({tx},{ty}) towards ({hx},{hy})") }
}

fn pull_1d(t: i32, h: i32) -> i32 {
    if t < h { h-1 }
    else if t > h { h+1 }
    else { h }
}

fn parse_input(input: &str) -> impl Iterator<Item=(Direction, i32)> + '_ {
    input.trim().lines().filter_map(move |l| {
        let mut toks = l.into_tokens();
        let dir = match toks.next()? {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            _ => Direction::Right
        };

        toks.token(' ');
        let count: i32 = toks
            .tokens_while(|c| c.is_numeric())
            .collect::<String>()
            .parse()
            .ok()?;

        Some((dir, count))
    })
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn nudge(&self, (x,y): (i32,i32)) -> (i32,i32) {
        match self {
            Direction::Down => (x,y+1),
            Direction::Up => (x,y-1),
            Direction::Left => (x-1,y),
            Direction::Right => (x+1,y),
        }
    }
}