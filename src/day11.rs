use std::collections::VecDeque;

struct Monkey {
    items: VecDeque<u128>,
    operation: Box<dyn Fn(u128) -> u128>,
    modulo: u128,
    then: usize,
    otherwise: usize,
}

fn monkeys() -> [Monkey; 8] {
    [
        Monkey {
            items: VecDeque::from_iter([99, 67, 92, 61, 83, 64, 98]),
            operation: Box::new(|n| n * 17),
            modulo: 3,
            then: 4,
            otherwise: 2,
        },
        Monkey {
            items: VecDeque::from_iter([78, 74, 88, 89, 50]),
            operation: Box::new(|n| n * 11),
            modulo: 5,
            then: 3,
            otherwise: 5,
        },
        Monkey {
            items: VecDeque::from_iter([98, 91]),
            operation: Box::new(|n| n + 4),
            modulo: 2,
            then: 6,
            otherwise: 4,
        },
        Monkey {
            items: VecDeque::from_iter([59, 72, 94, 91, 79, 88, 94, 51]),
            operation: Box::new(|n| n * n),
            modulo: 13,
            then: 0,
            otherwise: 5,
        },
        Monkey {
            items: VecDeque::from_iter([95, 72, 78]),
            operation: Box::new(|n| n + 7),
            modulo: 11,
            then: 7,
            otherwise: 6,
        },
        Monkey {
            items: VecDeque::from_iter([76]),
            operation: Box::new(|n| n + 8),
            modulo: 17,
            then: 0,
            otherwise: 2,
        },
        Monkey {
            items: VecDeque::from_iter([69, 60, 53, 89, 71, 88]),
            operation: Box::new(|n| n + 5),
            modulo: 19,
            then: 7,
            otherwise: 1,
        },
        Monkey {
            items: VecDeque::from_iter([72, 54, 63, 80]),
            operation: Box::new(|n| n + 3),
            modulo: 7,
            then: 1,
            otherwise: 3,
        },
    ]
}

pub fn star1() -> Result<u128, anyhow::Error> {
    let mut monkeys = monkeys();
    let mut seen_items = [0u128; 8];

    // 20 rounds
    for _ in 0..20 {
        for idx in 0..monkeys.len() {
            while let Some(item) = monkeys[idx].items.pop_front() {
                seen_items[idx] += 1;
                let new_item = (monkeys[idx].operation)(item) / 3;
                let throw_to = if new_item % monkeys[idx].modulo == 0 {
                    monkeys[idx].then
                } else {
                    monkeys[idx].otherwise
                };
                monkeys[throw_to].items.push_back(new_item);
            }
        }
    }

    // Find most seen items.
    seen_items.sort();
    Ok(seen_items[6] * seen_items[7])
}

pub fn star2() -> Result<u128, anyhow::Error> {
    let mut monkeys = monkeys();
    let mut seen_items = [0u128; 8];

    // we don't divide by 3 a bunch any more, so how do we keep the numbers from
    // growing loads? Well, the tests are all modulo based. If we modulo all numbers
    // by a modulo that is the multiplication of all of those, I think this means that
    // all of the modulo tests will pan out the same. (I wasn't certain without checking,
    // but it produced the right answer!)
    let all_mod = monkeys.iter().map(|m| m.modulo).fold(1, |a, b| a * b);

    for _ in 0..10_000 {
        for idx in 0..monkeys.len() {
            while let Some(item) = monkeys[idx].items.pop_front() {
                seen_items[idx] += 1;
                let new_item = (monkeys[idx].operation)(item) % all_mod;
                let throw_to = if new_item % monkeys[idx].modulo == 0 {
                    monkeys[idx].then
                } else {
                    monkeys[idx].otherwise
                };
                monkeys[throw_to].items.push_back(new_item);
            }
        }
    }

    // Find most seen items.
    seen_items.sort();
    Ok(seen_items[6] * seen_items[7])
}