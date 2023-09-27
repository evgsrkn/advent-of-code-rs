pub fn solution_a() {
    let points = [[3, 0, 6], [6, 3, 0], [0, 6, 3]];

    let res = include_bytes!("../../inputs/day2.txt")
        .split(|b| *b == b'\n')
        .filter(|l| !l.is_empty())
        .map(|l| ((l[0] - b'A') as usize, (l[2] - b'X') as usize))
        .map(|(a, b)| points[b][a] + b + 1)
        .sum::<usize>();

    println!(" | a - {}", res)
}

pub fn solution_b() {
    let points = [[3, 1, 2], [1, 2, 3], [2, 3, 1]];

    let res = include_bytes!("../../inputs/day2.txt")
        .split(|b| *b == b'\n')
        .filter(|l| !l.is_empty())
        .map(|l| ((l[0] - b'A') as usize, (l[2] - b'X') as usize))
        .map(|(a, b)| points[b][a] + b * 3)
        .sum::<usize>();

    println!(" | b - {}", res)
}
