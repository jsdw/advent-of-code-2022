use super::File;
use itertools::Itertools;

pub fn star1(file: File) -> Result<i64, anyhow::Error> {
    let cmds = parse_input(&file.contents);
    let mut cpu = Machine::new(cmds.collect());

    let mut signal_strength = 0;
    while cpu.step() {
        // "during cycle 20" is the same result as "after cycle 19", so we add
        // 1 to the cycle counter to record the result.
        if [20,60,100,140,180,220].iter().contains(&(cpu.counter() + 1)) {
            signal_strength += cpu.x() * (cpu.counter() + 1) as i64;
        }
    }

    Ok(signal_strength)
}

pub fn star2(file: File) -> Result<&'static str, anyhow::Error> {
    let cmds = parse_input(&file.contents);
    let mut cpu = Machine::new(cmds.collect());

    // during cycle 1 (eg at cycle 0) print '#' to start:
    print!("#");
    let mut x = 1;

    while cpu.step() {
        if ((cpu.x()-1)..=(cpu.x()+1)).contains(&x) {
            print!("#");
        } else {
            print!(".");
        }

        x += 1;
        if x == 40 {
            x = 0;
            println!();
        }
    }

    Ok("")
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
    fn counter(&self) -> usize {
        self.counter
    }
    fn x(&self) -> i64 {
        self.x
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
                    self.x += *n;
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