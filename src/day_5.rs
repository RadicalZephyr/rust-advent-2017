#[derive(Debug)]
struct Simulator<F> {
    idx: isize,
    ticks: u32,
    increment_rule: F,
    program: Vec<i16>,
}

impl<F> Simulator<F>
    where F: Fn(i16) -> i16
{
    pub fn new(program: Vec<i16>, increment_rule: F) -> Simulator<F> {
        Simulator {
            idx: 0,
            ticks: 0,
            increment_rule,
            program
        }
    }

    pub fn tick(&mut self) {
        self.ticks += 1;
        let offset = self.program[self.idx as usize];
        self.program[self.idx as usize] = (self.increment_rule)(offset);
        self.idx += offset as isize;
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
    let mut simulator = Simulator::new(program, |offset| offset + 1);

    while !simulator.has_escaped() {
        simulator.tick();
    }

    simulator.tick_count()
}

pub fn solve2(puzzle: &str) -> u32 {
    let program: Vec<i16> = puzzle.lines().map(|line| line.parse::<i16>().expect("Should have been a number")).collect();
    let mut simulator = Simulator::new(program, |offset| {
        if offset >= 3 {
            offset - 1
        } else {
            offset + 1
        }
    });

    while !simulator.has_escaped() {
        simulator.tick();
    }

    simulator.tick_count()
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
        assert_eq!(10, solve2("0\n3\n0\n1\n-3"));
    }
}
