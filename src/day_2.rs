fn row_difference(row: Vec<u8>) -> u8 {
    assert!(row.len() > 0);
    let max = row.iter().max().unwrap();
    let min = row.iter().min().unwrap();
    max - min
}

pub fn solve(puzzle: &str) -> u32 {
    0
}

pub fn solve2(puzzle: &str) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_row_difference() {
        assert_eq!(8, row_difference(vec![5, 1, 9, 5]));
    }

    #[test]
    fn test_row_difference2() {
        assert_eq!(4, row_difference(vec![7, 5, 3]));
    }
}
