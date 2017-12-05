// header
#![allow(dead_code)]

enum Direction {
    Up, Down,
    Left, Right,
}

impl Direction {
    fn turn(&self) -> Direction {
        use self::Direction::*;
        match *self {
            Right => Up,
            Up => Left,
            Left => Down,
            Down => Right,
        }
    }
}

struct State {
    x: i32,
    y: i32,
    side_length: u64,
    direction: Direction,
}

impl State {
    fn new() -> State {
        State {
            x: 0,
            y: 0,
            side_length: 1,
            direction: Direction::Right,
        }
    }
}

impl Iterator for State {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        let result = (self.x, self.y);
        self.x += 1;
        Some(result)
    }
}

fn count_squares(mut index: usize) -> (i32, i32) {
    let state = State::new();
    state.skip(index - 1).next().unwrap()
}

enum Kind {
    Even(i32),
    Odd(i32),
}

impl Kind {
    fn new(num: u32) -> Kind {
        match num {
            0 => Kind::Even(0),
            1 => Kind::Odd(0),
            n if (n % 2) == 0 => Kind::Even((n/2) as i32),
            n if (n % 2) == 1 => Kind::Odd(((n-1)/2) as i32),
            _ => unreachable!(),
        }
    }

    fn coordinates(&self) -> (i32, i32) {
        match *self {
            Kind::Even(k) => (-k+1,  k),
            Kind::Odd(k)  => ( k  , -k),
        }
    }

    #[allow(unused_variables)]
    fn coordinates_of(&self, offset: u32) -> (i32, i32) {
        if offset == 0 {
            return self.coordinates();
        }

        let offset = offset as i32;
        match *self {
            Kind::Even(k) => {
                let side_length = 2*k;
                let offset = offset - 1; // this one is going from -k+1 -> -k on X
                if offset < side_length {
                    (-k,k-offset)
                } else {
                    let offset = offset - side_length;
                    (-k+offset, -k)
                }
            },
            Kind::Odd(k)  => {
                let side_length = (2*k)+1;
                let offset = offset - 1; // this one is going from k -> k+1 on X
                if offset <= side_length {
                    (k+1, -k+offset)
                } else {
                    let offset = offset - side_length;
                    (k+1-offset, k+1)
                }
            },
        }
    }
}

fn coordinates_of(index: u32) -> (i32, i32) {
    if index == 1 {
        return (0, 0);
    }

    let root = (index as f32).sqrt().floor() as u32;
    let root_kind = Kind::new(root);

    let offset = index - (root*root);
    let coords = root_kind.coordinates_of(offset);
    coords
}

pub fn solve(puzzle: &str) -> u32 {
    let mem_index: usize = puzzle.parse().expect("Puzzle input must be a number.");
    let (x, y) = count_squares(mem_index);
    (x.abs() + y.abs()) as u32
}

pub fn solve2(_puzzle: &str) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_coords_of_1() {
        assert_eq!((0, 0), coordinates_of(1))
    }

    #[test]
    fn test_coords_of_4() {
        assert_eq!((0, 1), coordinates_of(4))
    }

    #[test]
    fn test_many_coords() {
        assert_eq!(( 1, -1), coordinates_of(9));
        assert_eq!((-1,  2), coordinates_of(16));
        assert_eq!(( 2, -2), coordinates_of(25));
        assert_eq!((-2,  3), coordinates_of(36));
        assert_eq!(( 3, -3), coordinates_of(49));
    }

    #[test]
    fn test_one() {
        assert_eq!(0, solve("1"));
    }

    #[test]
    fn test_two() {
        assert_eq!(1, solve("2"));
    }

    #[test]
    fn test_three() {
        assert_eq!(2, solve("3"));
    }

    // #[test]
    // fn test_four() {
    //     assert_eq!(1, solve("4"));
    // }

    // #[test]
    // fn test_five() {
    //     assert_eq!(2, solve("5"));
    // }

    // #[test]
    // fn test_six() {
    //     assert_eq!(1, solve("6"));
    // }

    // #[test]
    // fn test_seven() {
    //     assert_eq!(2, solve("7"));
    // }

    // #[test]
    // fn test_eight() {
    //     assert_eq!(1, solve("8"));
    // }

    // #[test]
    // fn test_ten() {
    //     assert_eq!(3, solve("10"));
    // }

    // #[test]
    // fn test_eleven() {
    //     assert_eq!(2, solve("11"));
    // }

    // #[test]
    // fn test_twelve() {
    //     assert_eq!(3, solve("12"));
    // }

    // #[test]
    // fn test_fourteen() {
    //     assert_eq!(3, solve("14"));
    // }

    // #[test]
    // fn test_fifteen() {
    //     assert_eq!(2, solve("15"));
    // }

    // #[test]
    // fn test_fifty_nine() {
    //     assert_eq!(6, solve("59"));
    // }

    // use criterion::Criterion;

    // #[test]
    // fn criterion_benchmark() {
    //     Criterion::default()
    //         .bench_function("direct solve 100000",|b| b.iter(|| solve("100000")));
    // }
}
