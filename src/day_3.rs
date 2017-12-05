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
                    (-k+(offset-side_length), -k)
                }
            },
            Kind::Odd(k)  => {
                let side_length = 2*k;
                let offset = offset - 1; // this one is going from k -> k+1 on X
                (k+1,-k+offset)
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
    println!("Root: {}", root);
    println!("Offset: {}", offset);
    let coords = root_kind.coordinates_of(offset);
    println!("Coords: {:?}", coords);
    coords
}

pub fn solve(puzzle: &str) -> u32 {
    let mem_index: u32 = puzzle.parse().expect("Puzzle input must be a number.");
    let (x, y) = coordinates_of(mem_index);
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

    // #[test]
    // fn test_fifty_nine() {
    //     assert_eq!(6, solve("59"));
    // }
}
