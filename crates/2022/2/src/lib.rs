use helper::AOCError;
// A rock
// B
pub fn p1(input: &str) -> Result<impl ToString, AOCError> {
    let res: u64 = input
        .lines()
        .map(|line| {
            let p1 = line.chars().next().unwrap();
            let p2 = line.chars().nth(2).unwrap();
            match (p1, p2) {
                ('A', 'X') => 4,
                ('A', 'Y') => 8,
                ('A', 'Z') => 3,
                ('B', 'X') => 1,
                ('B', 'Y') => 5,
                ('B', 'Z') => 9,
                ('C', 'X') => 7,
                ('C', 'Y') => 2,
                ('C', 'Z') => 6,
                _ => unreachable!(),
            }
        })
        .sum();

    Ok(res)
}

pub fn p2(input: &str) -> Result<impl ToString, AOCError> {
    let res: u64 = input
        .lines()
        .map(|line| {
            let p1 = line.chars().next().unwrap();
            let p2 = line.chars().nth(2).unwrap();
            match (p1, p2) {
                ('A', 'X') => 3,
                ('A', 'Y') => 4,
                ('A', 'Z') => 8,
                ('B', 'X') => 1,
                ('B', 'Y') => 5,
                ('B', 'Z') => 9,
                ('C', 'X') => 2,
                ('C', 'Y') => 6,
                ('C', 'Z') => 7,
                _ => unreachable!(),
            }
        })
        .sum();

    Ok(res)
}
