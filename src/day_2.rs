use std::{cmp, ptr, str, u32};
use std::str::FromStr;

use nom::{digit, IResult};

fn row_difference(row: Vec<u32>) -> u32 {
    assert!(row.len() > 0);
    let max = row.iter().max().unwrap();
    let min = row.iter().min().unwrap();
    max - min
}

fn row_divisible(row: Vec<u32>) -> u32 {
    assert!(row.len() > 0);

    for i in row.iter() {
        for j in row.iter() {
            if ptr::eq(i, j) {
                continue;
            }

            let larger = cmp::max(i, j);
            let smaller = cmp::min(i, j);

            match u32::checked_div(*larger, *smaller) {
                Some(res) => return res,
                None => continue,
            }
        }
    }

    0
}

named!(number<u32>,
       map_res!(
           map_res!(
               digit,
               str::from_utf8
           ),
           FromStr::from_str
       )
);

named!(delim, eat_separator!(&[9][..]));

named!(parse_row<Vec<u32>>,
       separated_nonempty_list!(delim, number)
);

fn iter_table<F: Fn(Vec<u32>) -> u32>(puzzle: &str, f: F) -> u32 {
    let mut sum = 0;

    for line in puzzle.trim().split('\n') {
        match parse_row(line.as_bytes()) {
            IResult::Done(_, row) => sum += f(row),
            IResult::Incomplete(content) => panic!(format!("{:?}", content)),
            IResult::Error(e) => panic!(format!("{}", e)),
        }
    }

    sum

}

pub fn solve(puzzle: &str) -> u32 {
    iter_table(puzzle, row_difference)
}

pub fn solve2(puzzle: &str) -> u32 {
    iter_table(puzzle, row_divisible)
}

#[cfg(test)]
mod test {
    use super::*;

    use nom::IResult;

    #[test]
    fn test_row_difference() {
        assert_eq!(8, row_difference(vec![5, 1, 9, 5]));
    }

    #[test]
    fn test_row_difference2() {
        assert_eq!(4, row_difference(vec![7, 5, 3]));
    }

    #[test]
    fn test_parse_row() {
        assert_eq!(IResult::Done(&b""[..], vec![5, 1, 9, 5]), parse_row("5\t1\t9\t5".as_bytes()));
    }

    #[test]
    fn test_parse_row2() {
        assert_eq!(IResult::Done(&b""[..], vec![1, 5, 5, 9]), parse_row("1\t5\t5\t9".as_bytes()));
    }

    #[test]
    fn test_parse_row3() {
        let line = "626	2424	2593	139	2136	163	1689	367	2235	125	2365	924	135	2583	1425	2502";
        assert_eq!(IResult::Done(&b""[..], vec![626, 2424, 2593, 139, 2136, 163, 1689, 367, 2235, 125, 2365, 924, 135, 2583, 1425, 2502]),
parse_row(line.as_bytes()));
    }

    #[test]
    fn test_the_whole_enchilada() {
        assert_eq!(18, solve("5\t1\t9\t5\n7\t5\t3\n2\t4\t6\t8\n"));
    }

    #[test]
    fn the_second_enchilada() {
        assert_eq!(9, solve2("5\t1\t9\t2\t8\n9\t4\t7\t3\n3\t8\t6\t5\n"));
    }
}
