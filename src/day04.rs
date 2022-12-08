use super::File;

fn parse_input(input: &str) -> impl Iterator<Item=((usize,usize),(usize,usize))> + '_ {
    let re = regex!("([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)");
    input.trim().lines().filter_map(move |l| {
        let caps = re.captures(l)?;
        let as_num = |n| caps.get(n)?.as_str().parse().ok();
        Some(((as_num(1)?, as_num(2)?),(as_num(3)?,as_num(4)?)))
    })
}

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