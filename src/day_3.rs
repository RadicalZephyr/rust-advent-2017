// header
#![allow(dead_code)]

use std::collections::HashMap;

enum Direction {
    Up, Down,
    Left, Right,
}

impl Direction {
    fn turn(&mut self) {
        use self::Direction::*;
        *self = match *self {
            Right => Up,
            Up    => Left,
            Left  => Down,
            Down  => Right,
        };
    }
}

struct State {
    x: i32,
    y: i32,
    side_index: u64,
    side_length: u64,
    turn_count: u8,
    direction: Direction,
}

impl State {
    fn new() -> State {
        State {
            x: 0,
            y: 0,
            side_index: 0,
            side_length: 1,
            turn_count: 0,
            direction: Direction::Right,
        }
    }

    fn step(&mut self) {
        use self::Direction::*;

        self.side_index += 1;

        match self.direction {
            Right => self.x += 1,
            Up    => self.y += 1,
            Left  => self.x -= 1,
            Down  => self.y -= 1,
        }

        if self.side_index >= self.side_length {
            self.side_index = 0;
            self.turn();
        }
    }

    fn turn(&mut self) {
        self.direction.turn();
        self.turn_count += 1;

        if self.turn_count >= 2 {
            self.turn_count = 0;
            self.side_length += 1;
        }
    }
}

impl Iterator for State {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        let result = (self.x, self.y);

        self.step();

        Some(result)
    }
}

fn neighbors(coord: (i32, i32)) -> Vec<(i32, i32)> {
    let mut result = vec![];
    let (x, y) = coord;
    for ix in (x-1)..(x+2) {
        for iy in  (y-1)..(y+2) {
            if ix == x && iy == y {
                continue;
            }
            result.push((ix, iy));
        }
    }
    result
}

fn count_squares(index: usize) -> (i32, i32) {
    let mut state = State::new();
    for _i in 1..index {
        state.next();
    }

    state.next().unwrap()
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

fn coordinates_of(index: usize) -> (i32, i32) {
    if index == 1 {
        return (0, 0);
    }

    let root = (index as f32).sqrt().floor() as u32;
    let root_kind = Kind::new(root);

    let offset = index as u32 - (root*root);
    let coords = root_kind.coordinates_of(offset);
    coords
}

pub fn solve(puzzle: &str) -> u32 {
    let mem_index: usize = puzzle.parse().expect("Puzzle input must be a number.");
    let (x, y) = coordinates_of(mem_index);
    // let (x, y) = count_squares(mem_index);
    (x.abs() + y.abs()) as u32
}

pub fn solve2(puzzle: &str) -> u32 {
    let threshold: u32 = puzzle.parse().expect("Puzzle input must be a number.");
    let mut state = State::new();
    let mut storage = HashMap::new();

    // Prime the storage
    storage.insert((0,0), 1);
    state.next();

    for coord in state {
        let total = neighbors(coord).into_iter().map(|coord| storage.get(&coord).unwrap_or(&0)).sum();
        if total > threshold {
            return total;
        }

        storage.insert(coord, total);
    }

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
    fn test_neighbors() {
        let expected = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
        assert_eq!(expected, neighbors((0, 0)));
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

    #[test]
    fn test_four() {
        assert_eq!(1, solve("4"));
    }

    #[test]
    fn test_five() {
        assert_eq!(2, solve("5"));
    }

    #[test]
    fn test_six() {
        assert_eq!(1, solve("6"));
    }

    #[test]
    fn test_seven() {
        assert_eq!(2, solve("7"));
    }

    #[test]
    fn test_eight() {
        assert_eq!(1, solve("8"));
    }

    #[test]
    fn test_ten() {
        assert_eq!(3, solve("10"));
    }

    #[test]
    fn test_eleven() {
        assert_eq!(2, solve("11"));
    }

    #[test]
    fn test_twelve() {
        assert_eq!(3, solve("12"));
    }

    #[test]
    fn test_fourteen() {
        assert_eq!(3, solve("14"));
    }

    #[test]
    fn test_fifteen() {
        assert_eq!(2, solve("15"));
    }

    #[test]
    fn test_fifty_nine() {
        assert_eq!(6, solve("59"));
    }

    #[test]
    fn test_second_nine() {
        assert_eq!(10, solve2("9"));
    }

    #[test]
    fn test_second_twelve() {
        assert_eq!(23, solve2("12"));
    }

    use criterion::Criterion;

    // #[test]
    fn criterion_benchmark() {
        Criterion::default()
            .bench_function("direct solve 100000",|b| b.iter(|| solve("100000")));
    }
}
