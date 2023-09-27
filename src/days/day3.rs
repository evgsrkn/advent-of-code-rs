pub fn solution_a() {
    let res = include_bytes!("../../inputs/day3.txt")
        .split(|b| *b == b'\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.split_at(l.len() / 2))
        .map(|(a, b)| b.iter().filter(|c| a.contains(c)).next().unwrap())
        .map(|b| {
            if *b >= b'a' {
                (b - b'a') as i16 + 1
            } else {
                (b - b'A') as i16 + 27
            }
        })
        .sum::<i16>();

    println!(" | a - {}", res)
}

pub fn solution_b() {
    let res = include_bytes!("../../inputs/day3.txt")
        .split(|b| *b == b'\n')
        .filter(|b| !b.is_empty())
        .collect::<Vec<&[u8]>>()
        .chunks(3)
        .map(|c| {
            c[0].iter()
                .find(|b| c[1].contains(b) && c[2].contains(b))
                .unwrap()
        })
        .map(|b| {
            if *b >= b'a' {
                (b - b'a') as i16 + 1
            } else {
                (b - b'A') as i16 + 27
            }
        })
        .sum::<i16>();

    println!(" | b - {}", res)
}
