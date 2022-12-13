use std::str::FromStr;

use crate::solver::Solver;

#[derive(Debug)]
enum Dir {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl FromStr for Dir {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amt) = s.split_once(' ').unwrap();
        let amt = amt.parse::<usize>().map_err(|_| "nope".to_string())?;
        match dir {
            "forward" => Ok(Dir::Forward(amt)),
            "down" => Ok(Dir::Down(amt)),
            "up" => Ok(Dir::Up(amt)),
            _ => Err("nope".to_string()),
        }
    }
}

#[derive(Default, Debug)]
pub struct Solution {
    input: Vec<Dir>,
}

impl Solver for Solution {
    fn parse(&mut self, input: &str) {
        self.input = input.split('\n').map(|l| l.parse().unwrap()).collect();
    }

    fn solve_part1(&self) -> String {
        let mut horizontal = 0;
        let mut depth = 0;
        self.input.iter().for_each(|d| match d {
            Dir::Forward(amt) => horizontal += amt,
            Dir::Down(amt) => depth += amt,
            Dir::Up(amt) => depth -= amt,
        });
        (horizontal * depth).to_string()
    }

    fn solve_part2(&self) -> String {
        let mut horizontal = 0;
        let mut depth = 0;
        let mut aim = 0;
        self.input.iter().for_each(|d| match d {
            Dir::Forward(amt) => {
                horizontal += amt;
                depth += aim * amt;
            }
            Dir::Down(amt) => aim += amt,
            Dir::Up(amt) => aim -= amt,
        });
        (horizontal * depth).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_input<'a>() -> &'a str {
        "\
forward 5
down 5
forward 8
up 3
down 8
forward 2"
    }

    #[test]
    fn test_parse() {
        let mut solver = Solution::default();
        solver.parse(get_input());
        println!("{:#?}", solver);
    }

    #[test]
    fn test_solution_example1() {
        let mut solver = Solution::default();
        solver.parse(get_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "150");
    }

    #[test]
    fn test_solution_example2() {
        let mut solver = Solution::default();
        solver.parse(get_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "900");
    }

    #[test]
    fn test_solution_part1() {
        let mut solver = Solution::default();
        solver.parse(INPUT);
        let solution = solver.solve_part1();
        assert_eq!(solution, "");
    }

    #[test]
    fn test_solution_part2() {
        let mut solver = Solution::default();
        solver.parse(INPUT);
        let solution = solver.solve_part2();
        assert_eq!(solution, "");
    }
}

const INPUT: &'static str = "\
forward 6
down 3
forward 8
down 5
forward 9
down 2
up 9
down 9
forward 8
down 3
down 8
forward 2
down 1
up 3
up 6
up 9
down 7
up 7
down 1
forward 7
down 7
up 4
down 2
forward 8
up 3
up 1
down 1
down 6
up 2
down 5
forward 4
down 5
down 3
forward 4
down 3
up 8
forward 1
up 9
forward 2
up 7
down 2
down 9
down 1
down 6
down 8
down 6
down 1
down 1
down 9
down 9
down 2
forward 9
down 1
forward 4
down 2
forward 6
down 8
forward 4
forward 8
forward 4
forward 4
up 4
up 9
down 6
forward 2
forward 5
down 2
forward 1
down 9
forward 2
down 8
down 2
forward 5
down 7
forward 7
down 4
up 3
down 9
forward 3
down 7
up 4
down 5
down 4
forward 8
down 2
down 2
forward 9
down 9
down 5
down 1
down 5
forward 5
down 1
up 7
down 2
forward 7
forward 6
forward 5
forward 4
down 3
forward 9
up 1
down 1
up 8
down 4
down 7
forward 2
down 1
up 9
up 3
down 4
down 1
down 9
down 4
forward 4
forward 7
down 7
down 1
up 6
forward 8
down 8
forward 2
down 4
up 4
forward 3
down 1
up 8
up 2
forward 3
forward 5
forward 7
down 5
up 2
down 6
forward 9
forward 3
down 1
forward 7
up 1
down 4
up 2
forward 5
down 1
forward 2
down 3
forward 9
down 1
down 6
down 7
up 9
down 5
down 1
forward 5
forward 7
down 6
forward 1
down 3
forward 3
forward 1
down 7
forward 9
forward 7
forward 4
up 1
down 8
up 8
down 3
forward 9
up 2
down 4
down 4
down 3
forward 7
forward 3
down 5
up 4
up 7
down 6
forward 2
down 2
down 9
down 9
down 7
down 7
forward 5
forward 8
up 2
forward 9
forward 5
down 2
up 6
down 2
up 2
down 6
down 3
down 2
down 3
down 9
forward 6
up 9
down 3
forward 9
forward 4
forward 1
down 3
down 4
forward 8
forward 4
down 7
forward 9
forward 2
forward 9
down 2
down 3
down 1
down 6
forward 5
down 3
forward 1
down 3
forward 7
down 3
forward 3
up 2
up 8
down 2
down 3
down 7
forward 6
forward 7
up 5
forward 4
forward 6
down 1
forward 1
forward 9
down 2
down 8
forward 6
down 8
down 5
forward 9
forward 3
down 6
forward 3
forward 1
up 7
down 2
down 9
up 6
forward 7
down 9
up 8
forward 5
forward 2
forward 9
down 3
up 7
forward 7
down 4
up 6
up 5
forward 6
forward 2
down 9
forward 9
forward 3
down 4
forward 5
forward 4
forward 4
down 8
forward 4
forward 2
up 8
down 8
forward 6
up 4
down 7
forward 8
up 9
forward 3
forward 5
forward 8
down 5
up 6
up 6
down 5
forward 2
down 3
up 1
down 8
forward 3
down 4
up 9
forward 8
forward 5
forward 2
forward 6
forward 8
up 5
forward 5
down 2
down 4
down 8
forward 3
up 9
down 1
down 9
forward 7
forward 9
down 4
down 2
forward 3
down 1
forward 2
down 2
down 5
forward 2
forward 3
forward 9
down 2
forward 3
forward 9
forward 6
forward 7
down 6
forward 5
up 7
forward 6
up 1
down 7
down 6
down 3
down 7
forward 2
forward 8
forward 3
down 3
forward 7
down 3
up 8
forward 1
down 5
down 9
down 6
forward 1
forward 1
down 1
down 1
forward 8
forward 7
forward 1
up 2
down 4
up 7
down 3
up 8
up 7
forward 3
up 9
down 5
forward 4
down 6
up 8
forward 6
forward 7
down 1
up 7
down 9
down 9
up 9
forward 7
down 6
down 4
down 6
down 7
down 7
up 7
down 4
up 7
forward 1
down 8
down 3
down 2
forward 9
up 7
down 1
down 2
forward 1
forward 5
down 7
up 4
down 7
down 4
down 5
up 8
down 6
down 2
down 4
up 5
down 8
down 3
down 9
forward 6
forward 5
down 1
down 3
down 2
down 3
forward 8
forward 4
forward 6
forward 9
up 1
forward 6
forward 8
down 2
down 1
forward 4
forward 2
forward 3
forward 2
forward 5
forward 2
forward 7
down 5
forward 2
forward 3
forward 9
down 3
down 4
down 7
down 9
down 5
forward 5
down 4
down 8
up 3
forward 1
forward 2
forward 6
up 2
down 9
down 8
up 8
up 3
forward 2
down 6
forward 9
down 3
down 3
forward 7
down 5
forward 2
down 4
down 1
forward 1
down 5
up 4
down 2
forward 8
down 9
down 5
up 4
forward 9
down 3
down 8
forward 8
forward 9
forward 3
up 5
forward 6
down 7
forward 5
down 4
down 9
down 1
up 4
down 8
forward 4
up 4
forward 4
forward 8
forward 3
forward 6
down 9
forward 5
up 4
forward 8
forward 2
down 2
down 1
up 3
forward 5
down 3
down 6
forward 7
down 8
down 1
forward 9
down 8
forward 7
forward 7
forward 7
up 9
up 5
forward 5
forward 2
down 4
up 8
up 7
forward 5
forward 3
forward 7
up 1
down 2
up 1
forward 3
up 8
down 3
forward 1
forward 5
forward 2
forward 5
down 8
up 1
forward 9
down 3
down 7
up 5
down 5
down 1
down 4
down 6
up 9
forward 5
forward 3
down 8
down 7
forward 3
down 9
forward 8
down 3
up 2
up 7
forward 3
down 9
down 5
down 9
up 6
down 9
down 1
down 1
up 4
up 5
up 6
forward 5
down 3
up 1
forward 9
forward 8
forward 8
forward 3
forward 5
forward 8
forward 1
down 8
up 7
down 3
forward 9
forward 1
up 8
down 7
up 4
down 2
down 5
forward 3
down 5
forward 8
forward 4
down 6
up 7
up 7
forward 8
down 6
down 8
down 9
forward 8
forward 1
forward 6
up 2
up 1
up 8
forward 8
forward 1
forward 4
forward 7
forward 2
down 7
down 8
up 5
up 4
up 4
up 7
forward 3
down 2
up 5
down 8
forward 6
up 9
forward 1
down 2
forward 7
down 4
down 6
down 3
down 7
down 9
down 3
forward 1
forward 5
down 2
down 6
up 7
up 2
up 3
up 5
forward 9
down 6
up 1
down 1
forward 3
forward 5
up 8
forward 5
forward 9
up 5
up 4
down 6
up 8
down 8
down 7
down 2
down 6
up 1
up 1
forward 8
down 4
up 3
down 2
down 1
forward 2
down 4
down 6
forward 2
up 8
forward 9
up 1
up 4
forward 2
down 9
down 4
forward 7
forward 6
forward 2
forward 2
forward 5
forward 6
down 3
forward 1
up 9
forward 2
down 3
down 1
down 3
up 9
forward 5
up 5
up 7
down 5
down 4
down 9
down 3
down 3
down 9
down 4
down 3
down 9
forward 9
down 1
down 6
down 7
down 7
down 5
down 8
down 5
forward 1
forward 3
up 1
forward 2
up 5
up 8
down 1
up 8
up 6
up 4
up 1
forward 3
forward 2
forward 4
up 3
down 6
down 1
down 6
up 8
up 7
forward 8
down 9
down 3
forward 2
forward 8
forward 8
down 1
forward 9
down 2
down 3
down 9
down 2
forward 8
down 2
down 6
forward 8
forward 1
up 1
forward 3
down 5
down 6
down 5
down 4
forward 6
forward 3
down 7
down 8
down 7
up 7
down 9
down 8
forward 6
down 1
forward 8
forward 9
up 4
down 1
forward 1
forward 9
down 4
down 2
forward 4
down 5
forward 4
down 7
forward 6
down 3
forward 3
forward 2
forward 7
down 2
forward 2
down 3
up 9
forward 4
forward 1
forward 8
forward 8
forward 6
forward 7
up 8
down 4
up 6
forward 3
up 8
forward 3
forward 1
forward 3
forward 9
up 2
up 5
forward 8
forward 6
forward 6
forward 4
down 6
forward 7
forward 3
forward 2
forward 2
forward 6
forward 5
down 7
up 1
forward 5
up 1
up 9
forward 5
up 3
forward 1
down 2
up 2
down 4
forward 7
forward 4
forward 1
down 1
up 4
down 4
up 2
up 5
down 5
forward 7
up 1
down 6
up 4
forward 3
forward 8
down 6
forward 4
down 2
down 3
down 5
down 4
down 9
up 4
forward 5
up 1
up 2
forward 7
forward 2
up 1
down 8
forward 4
forward 4
up 8
down 3
down 4
up 7
down 8
down 6
down 2
down 3
forward 9
forward 7
forward 6
down 2
down 7
forward 5
forward 2
up 5
down 5
forward 5
down 3
down 1
forward 4
forward 3
down 2
up 1
down 3
down 5
forward 6
forward 5
up 5
down 3
forward 8
down 9
up 4
up 4
down 8
forward 5
down 7
down 3
up 1
down 4
down 5
forward 4
forward 2
forward 4
up 9
down 5
forward 4
forward 6
forward 9
forward 7
forward 5
forward 6
up 4
forward 8
down 4
forward 4
forward 6
up 8
down 4
forward 3
down 8
forward 4
down 9
forward 5
down 4
up 8
forward 2
down 6
up 3
down 5
down 1
down 6
down 9
forward 9
down 1
down 5
up 8
forward 5
down 6
down 9
forward 1
down 6
down 8
down 1
down 2
down 1
forward 5
up 7
forward 5
down 2
down 4
down 1
forward 7
down 7
down 8
forward 4
forward 7
down 2
down 3
forward 2
up 9
down 4
down 5
forward 4
forward 4
forward 6
down 5
forward 8
down 9
forward 8
down 7
up 7
forward 9
up 1
forward 4
up 3
down 2
down 4
down 5
forward 2
forward 8
up 3
up 1
down 1
forward 7
forward 9
forward 6
up 1
down 2
forward 1
up 5
forward 3
down 7
down 6
forward 9
forward 6
forward 3
forward 8
down 2
down 7
forward 1
down 6
up 3
down 6
down 9
up 2
forward 8
forward 1
down 9
forward 8
forward 8
down 3
up 9
down 6
up 3
forward 3
forward 5
forward 7";
