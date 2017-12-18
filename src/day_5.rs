#[derive(Debug)]
struct Simulator {
    idx: isize,
    ticks: u32,
    program: Vec<i16>,
}

impl Simulator {
    pub fn new(program: Vec<i16>) -> Simulator {
        Simulator {
            idx: 0,
            ticks: 0,
            program
        }
    }

    pub fn tick(&mut self) {
        self.ticks += 1;
        let increment = self.program[self.idx as usize];
        self.program[self.idx as usize] += 1;
        self.idx += increment as isize;
    }

    pub fn has_escaped(&self) -> bool {
        self.idx < 0 || self.idx as usize >= self.program.len()
    }

    pub fn tick_count(&self) -> u32 {
        self.ticks
    }
}

pub fn solve(puzzle: &str) -> u32 {
    let program: Vec<i16> = puzzle.lines().map(|line| line.parse::<i16>().expect("Should have been a number")).collect();
    let mut simulator = Simulator::new(program);

    while !simulator.has_escaped() {
        simulator.tick();
    }

    simulator.tick_count()
}

pub fn solve2(_puzzle: &str) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(5, solve("0\n3\n0\n1\n-3"));
    }

    #[test]
    fn test_solve2() {
        assert_eq!(0, solve2(""));
    }
}
