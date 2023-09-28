use itertools::Itertools;

pub fn solution_a() {
    let data = include_bytes!("../../inputs/day6.rs");
    let res = data
        .windows(4)
        .enumerate()
        .find(|(_, n)| n.iter().unique().count() == 4)
        .unwrap();

    println!(" | a - {}", res.0 + 4);
}

pub fn solution_b() {
    let data = include_bytes!("../../inputs/day6.rs");
    let res = data
        .windows(14)
        .enumerate()
        .find(|(_, n)| n.iter().unique().count() == 14)
        .unwrap();

    println!(" | b - {}", res.0 + 14);
}
