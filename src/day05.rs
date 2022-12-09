use super::File;

#[derive(Debug)]
struct Command {
    count: usize,
    from: usize,
    to: usize
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Command>) {
    let mut split = input.split("\n\n");

    // Turn:
    //
    // [D]
    // [N] [C]
    // [Z] [M] [P]
    //  1   2   3
    //
    // Into:
    //
    // vec![vec![Z,N,D], vec![M,C], vec![P]]
    let stack = {
        let stack_str = split.next().unwrap_or_default();
        let stack_strs: Vec<&[u8]> = stack_str
            .lines()
            .filter(|l| l.contains('['))
            .map(|l| l.as_bytes())
            .collect();

        let mut stack_chars: Vec<Vec<char>> = Vec::new();
        let mut idx = 1; // look at specific indexes for letters; 1, 5, 9...
        'lo: loop {
            let mut chars = Vec::new();
            // Start from the bottom of the stack in each column
            for (i,s) in stack_strs.iter().rev().enumerate() {
                let Some(c) = s.get(idx) else {
                    if i == 0 {
                        // if no entry on the bottom, we're done
                        break 'lo
                    } else {
                        // else we are just done with this column.
                        break;
                    }
                };
                if (b'A'..=b'Z').contains(&c) {
                    chars.push(*c as char);
                }
            }
            if !chars.is_empty() {
                stack_chars.push(chars);
            }
            idx += 4;
        }
        stack_chars
    };

    // And now, the commands
    let cmd_regex = regex!("move ([0-9]+) from ([0-9]+) to ([0-9]+)");
    let cmds = split.next().unwrap_or_default().lines().filter_map(|l| {
        let caps = cmd_regex.captures(l)?;
        let as_num = |n| caps.get(n)?.as_str().parse().ok();
        Some(Command { count: as_num(1)?, from: as_num(2)?, to: as_num(3)? })
    });

    (stack, cmds.collect())
}

pub fn star1(file: File) -> Result<String, anyhow::Error> {
    let (mut stack, commands) = parse_input(&file.contents);

    for Command { count, from, to } in commands {
        for _ in 0..count {
            if let Some(item) = stack[from-1].pop() {
                stack[to-1].push(item);
            }
        }
    }

    let s: String = stack.iter().filter_map(|items| items.last().copied()).collect();
    Ok(s)
}

pub fn star2(file: File) -> Result<String, anyhow::Error> {
    let (mut stack, commands) = parse_input(&file.contents);

    for Command { count, from, to } in commands {
        let mut carried = Vec::new();
        for _ in 0..count {
            if let Some(item) = stack[from-1].pop() {
                carried.push(item);
            }
        }
        while let Some(item) = carried.pop() {
            stack[to-1].push(item);
        }
    }

    let s: String = stack.iter().filter_map(|items| items.last().copied()).collect();
    Ok(s)
}