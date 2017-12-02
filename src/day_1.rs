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

pub fn solve(puzzle: &str) -> u32 {
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

pub fn solve2(puzzle: &str) -> u32 {
    let bytes = puzzle.as_bytes();
    let length = bytes.len();
    let offset = length / 2;
    let mut sum = 0;

    for index in 0..length {
        sum += value_of(&bytes[index], &bytes[(index+offset) % length]);
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(3, solve("1122"));
    }

    #[test]
    fn test_two() {
        assert_eq!(4, solve("1111"));
    }

    #[test]
    fn test_three() {
        assert_eq!(0, solve("1234"));
    }

    #[test]
    fn test_four() {
        assert_eq!(9, solve("91212129"));
    }

    #[test]
    fn test_five() {
        assert_eq!(6, solve2("1212"));
    }

    #[test]
    fn test_six() {
        assert_eq!(0, solve2("1221"));
    }

    #[test]
    fn test_seven() {
        assert_eq!(4, solve2("123425"));
    }

    #[test]
    fn test_eight() {
        assert_eq!(12, solve2("123123"));
    }

    #[test]
    fn test_nine() {
        assert_eq!(4, solve2("12131415"));
    }
}
