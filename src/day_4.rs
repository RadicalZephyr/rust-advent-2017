use std::collections::HashSet;

fn is_valid(passphrase: &str) -> bool {
    let mut words = HashSet::new();
    for word in passphrase.split(' ') {
        if words.contains(word) {
            return false;
        }
        words.insert(word);
    }
    return true;
}

fn is_valid2(passphrase: &str) -> bool {
    let mut words = HashSet::new();
    for word in passphrase.split(' ') {
        let mut bytes: Vec<u8> = word.bytes().collect();
        bytes.sort();

        if words.contains(&bytes) {
            return false;
        }
        words.insert(bytes);
    }
    return true;
}

pub fn solve(puzzle: &str) -> u32{
    puzzle.lines().filter(|phrase| is_valid(phrase)).count() as u32
}

pub fn solve2(puzzle: &str) -> u32 {
    puzzle.lines().filter(|phrase| is_valid2(phrase)).count() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_passphrase() {
        assert!(is_valid("aa bb cc dd ee"));
    }

    #[test]
    fn test_invalid_passphrase() {
        assert!(!is_valid("aa bb cc dd aa"));
    }

    #[test]
    fn test_solve() {
        assert_eq!(2, solve("aa bb cc dd e\naa bb cc dd aa\naa bb cc dd aaa"));

    }

    #[test]
    fn test_solve2() {
        assert_eq!(1, solve2("ab ba\nab bc"))
    }
}
