use super::File;
use std::collections::{ HashMap, HashSet, VecDeque };

pub fn star1(file: File) -> Result<usize, anyhow::Error> {
    let map = parse_input(&file.contents);
    let openable_valves = map.values().filter(|v| v.rate > 0).count();

    struct SearchItem<'a> {
        valve: &'a str,
        released: usize,
        time_left: usize,
        open: HashSet<&'a str>,
        last: Option<&'a str>
    }

    // the search queue.
    let mut current = VecDeque::from_iter([
        SearchItem { valve: "AA", released: 0, time_left: 30, open: HashSet::new(), last: None }
    ]);
    // the best result we've found so far.
    let mut best_released = 0;

    while let Some(curr) = current.pop_back() {
        let valve = map.get(curr.valve).unwrap();
        let time_left = curr.time_left - 1;

        // No time or all valves open; nothing more we can do so end.
        if time_left == 0 || curr.open.len() == openable_valves {
            if curr.released > best_released {
                best_released = curr.released;
            }
            continue
        }

        // One move for each connection.
        for conn in &valve.connections {
            if let Some(v) = curr.last {
                if v == *conn {
                    continue
                }
            }
            current.push_back(SearchItem {
                valve: conn,
                released: curr.released,
                time_left: time_left,
                open: curr.open.clone(),
                last: Some(curr.valve)
            })
        }

        // One move to turn the valve on in current location.
        if valve.rate > 0 && !curr.open.contains(curr.valve) {
            let mut open = curr.open;
            open.insert(curr.valve);

            current.push_back(SearchItem {
                valve: curr.valve,
                released: curr.released + (time_left * valve.rate),
                time_left: time_left,
                open: open,
                last: None
            });
        }
    }

    Ok(best_released)
}

pub fn star2(_file: File) -> Result<&'static str, anyhow::Error> {
    Ok("This is about where I can't be bothered any more :)")
}

struct Valve<'a> {
    rate: usize,
    connections: Vec<&'a str>
}

fn parse_input(input: &str) -> HashMap<&str, Valve> {
    let valve_re = regex!("[A-Z][A-Z]");
    let flow_re = regex!("rate=([0-9]+)");
    input.trim().lines().map(move |l| {
        let mut caps = valve_re.captures_iter(l);
        let valve = caps.next().unwrap().get(0).unwrap().as_str();
        let rate: usize = flow_re.captures(l).unwrap().get(1).unwrap().as_str().parse().unwrap();
        let connections: Vec<&str> = caps.map(|c| c.get(0).unwrap().as_str()).collect();
        (valve, Valve { rate, connections })
    }).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "
            Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
            Valve BB has flow rate=13; tunnels lead to valves CC, AA
            Valve CC has flow rate=2; tunnels lead to valves DD, BB
            Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
            Valve EE has flow rate=3; tunnels lead to valves FF, DD
            Valve FF has flow rate=0; tunnels lead to valves EE, GG
            Valve GG has flow rate=0; tunnels lead to valves FF, HH
            Valve HH has flow rate=22; tunnel leads to valve GG
            Valve II has flow rate=0; tunnels lead to valves AA, JJ
            Valve JJ has flow rate=21; tunnel leads to valve II
        ";

        assert_eq!(star1(File { contents: input.to_string() }).unwrap(), 1651);
    }
}