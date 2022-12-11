use super::File;

pub fn star1(file: File) -> Result<usize, anyhow::Error> {
    let n = parse_input(&file.contents).filter(|(a, b)| {
        (a.0 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.1 <= a.1)
    }).count();

    Ok(n)
}

pub fn star2(file: File) -> Result<usize, anyhow::Error> {
    let n = parse_input(&file.contents).filter(|(a, b)| {
        (a.0 >= b.0 && a.0 <= b.1) || (a.1 >= b.0 && a.1 <= b.1) ||
        (b.0 >= a.0 && b.0 <= a.1) || (b.1 >= a.1 && b.1 <= a.1)
    }).count();

    Ok(n)
}

fn parse_input(input: &str) -> impl Iterator<Item=Instruction> + '_ {
    let addx_re = regex!("addx (-?[0-9]+)");
    input.trim().lines().map(|l| l.trim()).filter_map(move |l| {
        if let Some(caps) = addx_re.captures(l) {
            let n = caps.get(1).unwrap().as_str().parse().unwrap();
            Some(Instruction::Addx(n))
        } else if l == "noop" {
            Some(Instruction::Noop)
        } else {
            None
        }
    })
}

#[derive(Clone,Copy)]
enum Instruction {
    Addx(i64),
    Noop
}

impl Instruction {
    fn delay(&self) -> usize {
        match self {
            Self::Addx(_) => 2,
            Self::Noop => 1
        }
    }
}

struct Machine {
    // How many cycles done?
    counter: usize,
    // Which instruction idx to run next?
    idx: usize,
    // Value of X
    x: i64,
    // Current instruction + remaining delay.
    instruction: Option<(usize, Instruction)>,
    // Instructions to run.
    instructions: Vec<Instruction>
}

impl Machine {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            counter: 0,
            idx: 0,
            x: 1,
            instruction: None,
            instructions
        }
    }
    fn step(&mut self) -> bool {
        // load current instruction
        let (delay, ins) = match &mut self.instruction {
            Some(i) => i,
            None => {
                let Some(&ins) = self.instructions.get(self.idx) else {
                    return false
                };
                self.instruction = Some((ins.delay(), ins));
                self.instruction.as_mut().unwrap()
            }
        };

        // move one cycle forwards:
        *delay -= 1;
        self.counter += 1;

        // if instruction "ready", run it.
        if *delay == 0 {
            match ins {
                Instruction::Addx(n) => {
                    self.x += n;
                },
                Instruction::Noop => {
                    // do nothing.
                }
            }
            self.idx += 1;
            self.instruction = None;
        }

        true
    }
}