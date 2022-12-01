use helper::AOCError;

fn p1_helper(input: &str) -> Vec<u64> {
    let mut totals: Vec<u64> = input
        .split("\n\n")
        .map(|person| {
            person
                .split('\n')
                .map(|food| food.parse::<u64>().unwrap_or(0))
                .sum()
        })
        .collect();

    totals.sort_unstable_by(|a, b| b.cmp(a));
    totals
}

pub fn p1(input: &str) -> Result<impl ToString, AOCError> {
    let totals = p1_helper(input);
    Ok(totals[0])
}

pub fn p2(input: &str) -> Result<impl ToString, AOCError> {
    let totals = p1_helper(input);
    let totals: u64 = totals.iter().take(3).sum();
    Ok(totals)
}
