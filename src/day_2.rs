use std::str;
use std::str::FromStr;

use nom::digit;

fn row_difference(row: Vec<u8>) -> u8 {
    assert!(row.len() > 0);
    let max = row.iter().max().unwrap();
    let min = row.iter().min().unwrap();
    max - min
}

named!(number<u8>,
       map_res!(
           map_res!(
               digit,
               str::from_utf8
           ),
           FromStr::from_str
       )
);

named!(delim, eat_separator!(&b"\t"[..]));

named!(parse_row<Vec<u8>>,
       separated_nonempty_list!(delim, number)
);

pub fn solve(puzzle: &str) -> u32 {
    0
}

pub fn solve2(puzzle: &str) -> u32 {
    0
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
}
