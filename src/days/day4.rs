pub fn solution_a() {
    let res = include_str!("../../inputs/day4.txt")
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(a, b)| (a.split_once('-').unwrap(), b.split_once('-').unwrap()))
        .map(|((a, b), (c, d))| {
            (
                (a.parse::<u8>().unwrap(), b.parse::<u8>().unwrap()),
                (c.parse::<u8>().unwrap(), d.parse::<u8>().unwrap()),
            )
        })
        .filter(|((a, b), (c, d))| (a >= c && b <= d) || (c >= a && d <= b))
        .count();

    println!(" | a - {}", res);
}

pub fn solution_b() {
    let res = include_str!("../../inputs/day4.txt")
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(a, b)| (a.split_once('-').unwrap(), b.split_once('-').unwrap()))
        .map(|((a, b), (c, d))| {
            (
                (a.parse::<u8>().unwrap(), b.parse::<u8>().unwrap()),
                (c.parse::<u8>().unwrap(), d.parse::<u8>().unwrap()),
            )
        })
        .filter(|((a, b), (c, d))| a <= d && b >= c)
        .count();

    println!(" | b - {}", res);
}
