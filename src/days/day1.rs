use std::fs;

pub fn solution() {
    let input: Vec<String> = fs::read_to_string("./inputs/day1.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut res: Vec<u32> = Vec::new();
    let mut tmp = 0;

    for line in input {
        if !line.is_empty() {
            tmp += line.parse::<u32>().unwrap();
            continue;
        }

        res.push(tmp);

        tmp = 0;
    }

    res.sort_unstable();

    println!(" | b - {}", res.iter().rev().take(3).sum::<u32>());
}
