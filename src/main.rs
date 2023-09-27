mod days;

use days::{day1, day2, day3, day4};

use crate::days::day5;

fn main() {
    println!("DAY 1");
    day1::solution();
    println!("DAY 2");
    day2::solution_a();
    day2::solution_b();
    println!("DAY 3");
    day3::solution_a();
    day3::solution_b();
    println!("DAY 4");
    day4::solution_a();
    day4::solution_b();
    println!("DAY 5");
    day5::solution_a();
    day5::solution_b();
}
