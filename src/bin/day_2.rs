extern crate advent_2017;

use std::fs::File;
use std::io::Read;

use advent_2017::day_2;

pub fn main() {
    let mut f = File::open("data/day_2.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Couldn't read file into string");

    println!("Part 1: {}", day_2::solve(s.trim()));
    println!("Part 2: {}", day_2::solve2(s.trim()));
}
