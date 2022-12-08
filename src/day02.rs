use super::File;

enum Letter1 {
    A,
    B,
    C
}

enum Letter2 {
    X,
    Y,
    Z
}

fn parse_input(input: &str) -> impl Iterator<Item = (Letter1, Letter2)> + '_ {
    input.lines().filter_map(|l| {
        let mut cs = l.trim().split(|c: char| c.is_whitespace());
        let fst = match cs.next()?.trim() {
            "A" => Letter1::A,
            "B" => Letter1::B,
            "C" => Letter1::C,
            _ => return None
        };
        let snd = match cs.next()?.trim() {
            "X" => Letter2::X,
            "Y" => Letter2::Y,
            "Z" => Letter2::Z,
            _ => return None
        };
        Some((fst,snd))
    })
}

pub fn star1(file: File) -> Result<usize, anyhow::Error> {
    // A|X = rock, B|Y = paper, C|Z = scissors.
    let score = parse_input(&file.contents).map(|(them, me)| {
        let shape_score = match me {
            Letter2::X => 1,
            Letter2::Y => 2,
            Letter2::Z => 3
        };
        let outcome_score = match (them, me) {
            (Letter1::A, Letter2::X) => 3,
            (Letter1::A, Letter2::Y) => 6,
            (Letter1::A, Letter2::Z) => 0,
            (Letter1::B, Letter2::X) => 0,
            (Letter1::B, Letter2::Y) => 3,
            (Letter1::B, Letter2::Z) => 6,
            (Letter1::C, Letter2::X) => 6,
            (Letter1::C, Letter2::Y) => 0,
            (Letter1::C, Letter2::Z) => 3,
        };
        shape_score + outcome_score
    }).sum();

    Ok(score)
}

pub fn star2(file: File) -> Result<usize, anyhow::Error> {
    // A = rock, B = paper, C = scissors.
    // X = lose, Y = draw, Z = win.
    let score = parse_input(&file.contents).map(|(them, me)| {
        let outcome_score = match me {
            Letter2::X => 0,
            Letter2::Y => 3,
            Letter2::Z => 6
        };
        let shape_score = match (them, me) {
            (Letter1::A, Letter2::X) => 3,
            (Letter1::A, Letter2::Y) => 1,
            (Letter1::A, Letter2::Z) => 2,
            (Letter1::B, Letter2::X) => 1,
            (Letter1::B, Letter2::Y) => 2,
            (Letter1::B, Letter2::Z) => 3,
            (Letter1::C, Letter2::X) => 2,
            (Letter1::C, Letter2::Y) => 3,
            (Letter1::C, Letter2::Z) => 1,
        };

        shape_score + outcome_score
    }).sum();

    Ok(score)
}