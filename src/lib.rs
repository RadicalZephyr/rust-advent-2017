#[macro_use]
extern crate nom;
#[cfg(test)]
extern crate criterion;

#[macro_export]
macro_rules! advent {
    ($name:ident, $mod:ident) => {
        use std::fs::File;
        use std::io::Read;

        use advent_2017::$mod;

        pub fn $name() {
            let mut f = File::open(concat!("data/", stringify!($mod), ".txt")).expect("File not found");
            let mut s = String::new();
            f.read_to_string(&mut s).expect("Couldn't read file into string");

            println!("Part 1: {}", $mod::solve(s.trim()));
            println!("Part 2: {}", $mod::solve2(s.trim()));
        }
    }
}

pub mod day_1;
pub mod day_2;
pub mod day_3;
