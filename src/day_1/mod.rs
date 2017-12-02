fn digit(byte: &u8) -> u32 {
    (byte - 48) as u32
}

fn value_of(current: &u8, last: &u8) -> u32 {
    if current == last {
        digit(current)
    } else {
        0
    }
}

pub fn run(puzzle: &str) -> u32 {
    let bytes = puzzle.as_bytes();
    let mut sum = 0;
    let mut last = &0;

    for byte in bytes {
        sum += value_of(byte, last);
        last = byte;
    }
    sum += value_of(&bytes[0], last);

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(3, run("1122"));
    }

    #[test]
    fn test_two() {
        assert_eq!(4, run("1111"));
    }
}
