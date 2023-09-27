pub fn solution_a() {
    let data = include_str!("../../inputs/day5.txt")
        .split("\n\n")
        .take(2)
        .collect::<Vec<_>>();

    let (map, moves) = (data[0], data[1]);
    let mut rs: [Vec<char>; 9] = Default::default();

    map.split('\n').rev().skip(1).for_each(|l| {
        l.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, ch)| *ch != ' ')
            .for_each(|(i, ch)| rs[i].push(ch));
    });

    moves.split('\n').filter(|l| !l.is_empty()).for_each(|l| {
        let mut it = l
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .map(|l| l.parse::<usize>().unwrap());

        let (a, b, c) = (it.next().unwrap(), it.next().unwrap(), it.next().unwrap());

        for _ in 0..a {
            let d = rs[b - 1].pop().unwrap();
            rs[c - 1].push(d);
        }
    });

    print!(" | a - ");
    for r in rs {
        print!("{}", r.last().unwrap())
    }
    println!();
}
