use std::{collections::HashMap, str::FromStr};

use crate::solver::Solver;

#[derive(Debug)]
struct RuckSack {
    compartment_1: String,
    compartment_2: String,
}

impl FromStr for RuckSack {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let partition = s.len() / 2;
        Ok(RuckSack {
            compartment_1: s[..partition].to_string(),
            compartment_2: s[partition..].to_string(),
        })
    }
}

impl RuckSack {
    fn find_matching_item(&self) -> char {
        self.compartment_1
            .chars()
            .find(|c| self.compartment_2.contains(*c))
            .unwrap()
    }
}

#[derive(Default)]
pub struct Solution {
    item_map: HashMap<char, usize>,
    input: Vec<String>,
}

impl Solver for Solution {
    fn parse(&mut self, input: &str) {
        self.item_map = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .zip(1..53)
            .collect();
        self.input = input.split('\n').map(|l| l.to_owned()).collect();
    }

    fn solve_part1(&self) -> String {
        self.input
            .iter()
            .map(|l| l.parse::<RuckSack>().unwrap().find_matching_item())
            .map(|i| self.item_map.get(&i).unwrap())
            .sum::<usize>()
            .to_string()
    }

    fn solve_part2(&self) -> String {
        self.input
            .chunks(3)
            .map(|g| {
                g[0].chars()
                    .find(|c| g[1].contains(*c) && g[2].contains(*c))
                    .unwrap()
            })
            .map(|i| self.item_map.get(&i).unwrap())
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_input<'a>() -> &'a str {
        "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
    }

    #[test]
    fn test_solution_part1() {
        let mut solver = Solution::default();
        solver.parse(get_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "157");
    }

    #[test]
    fn test_solution_part2() {
        let mut solver = Solution::default();
        solver.parse(get_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "70");
    }
}