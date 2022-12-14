use std::{collections::HashMap, hash::Hash, thread::sleep, time::Duration};

use crate::{solver::Solver, utils::clear_terminal};

const MIN_X: u8 = 0;
const MAX_X: u8 = 7;

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        if value == '<' {
            Self::Left
        } else {
            Self::Right
        }
    }
}

#[derive(Debug)]
enum Shape {
    Flat,
    Cross,
    Angle,
    Tall,
    Block,
}

impl Shape {
    fn get_pieces(&self, x: u8) -> [u8; 4] {
        match self {
            Shape::Flat => [0b1111000 >> x, 0, 0, 0],
            Shape::Cross => [0b0100000 >> x, 0b1110000 >> x, 0b0100000 >> x, 0],
            Shape::Angle => [0b1110000 >> x, 0b0010000 >> x, 0b0010000 >> x, 0],
            Shape::Tall => [
                0b1000000 >> x,
                0b1000000 >> x,
                0b1000000 >> x,
                0b1000000 >> x,
            ],
            Shape::Block => [0b1100000 >> x, 0b1100000 >> x, 0, 0],
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct P(u8, usize);

struct Rock<'a> {
    shape: &'a Shape,
    pos: P,
}

impl Rock<'_> {
    fn can_move_down(&self, rocks: &[u8]) -> bool {
        // base case is rock is at y == 0 on the floor
        if self.pos.1 == 0 {
            return false;
        }

        !self
            .shape
            .get_pieces(self.pos.0)
            .iter()
            .enumerate()
            .any(|(y, p)| {
                if let Some(r) = rocks.get(y + self.pos.1 - 1) {
                    r & p != 0
                } else {
                    false
                }
            })
    }

    fn can_move_left(&self, rocks: &[u8]) -> bool {
        if self.pos.0 == MIN_X {
            return false;
        }

        !self
            .shape
            .get_pieces(self.pos.0 - 1)
            .iter()
            .enumerate()
            .any(|(y, p)| {
                if let Some(r) = rocks.get(y + self.pos.1) {
                    r & p != 0
                } else {
                    false
                }
            })
    }

    fn can_move_right(&self, rocks: &[u8]) -> bool {
        let x = self.pos.0 + 1;

        if match self.shape {
            Shape::Flat => x + 3 == MAX_X,
            Shape::Cross => x + 2 == MAX_X,
            Shape::Angle => x + 2 == MAX_X,
            Shape::Tall => x == MAX_X,
            Shape::Block => x + 1 == MAX_X,
        } {
            return false;
        }

        !self.shape.get_pieces(x).iter().enumerate().any(|(y, p)| {
            if let Some(r) = rocks.get(y + self.pos.1) {
                r & p != 0
            } else {
                false
            }
        })
    }
}

#[derive(Default, Debug)]
pub struct Solution {
    jets: Vec<Dir>,
    shapes: Vec<Shape>,
}

// how many rows of rocks to use in cache key
const WINDOW_SIZE: usize = 3;
type CacheKey = ([u8; WINDOW_SIZE], usize, usize);

impl Solution {
    fn get_rock_height(&self, target_rock_count: u64) -> u64 {
        let mut jets = self.jets.iter().enumerate().cycle();
        let mut shapes = self.shapes.iter().enumerate().cycle();
        let mut rocks: Vec<u8> = Vec::new();
        let mut pattern_cache: HashMap<CacheKey, (u64, u64)> = HashMap::new();

        let mut extra_height = 0;
        let mut rock_count = 0;
        while rock_count < target_rock_count {
            rock_count += 1;
            let (shape_index, shape) = shapes.next().unwrap();
            let mut rock = Rock {
                pos: P(MIN_X + 2, rocks.len() + 3),
                shape,
            };

            // I don't know why linter complains about this
            // not being used when it clearly is...
            #[allow(unused_assignments)]
            let mut jet_index = 0;
            loop {
                let jet = jets.next().unwrap();
                jet_index = jet.0;
                match jet.1 {
                    Dir::Left => {
                        if rock.can_move_left(&rocks) {
                            rock.pos.0 -= 1;
                        }
                    }
                    Dir::Right => {
                        if rock.can_move_right(&rocks) {
                            rock.pos.0 += 1;
                        }
                    }
                }

                if rock.can_move_down(&rocks) {
                    rock.pos.1 -= 1;
                } else {
                    break;
                }
            }

            let pieces = rock.shape.get_pieces(rock.pos.0);

            for (y, section) in pieces.iter().enumerate() {
                // some sections of our shapes are just padding
                // so we can have arrays
                if section == &0u8 {
                    break;
                }

                let y = y + rock.pos.1;
                if y >= rocks.len() {
                    rocks.push(*section);
                } else {
                    let new_section = rocks[y] | section;
                    rocks[y] = new_section;
                }
            }

            if rocks.len() >= WINDOW_SIZE {
                let mut last_group: [u8; WINDOW_SIZE] = [0; WINDOW_SIZE];
                last_group.clone_from_slice(&rocks[rocks.len() - WINDOW_SIZE..]);

                let key = (last_group, jet_index, shape_index);
                if let Some((old_height, old_rock_count)) = pattern_cache.get(&key) {
                    let rock_diff = rock_count - *old_rock_count;
                    if rock_diff < 50 {
                        continue;
                    }

                    let height_diff = rocks.len() as u64 - *old_height;
                    let catchup = (target_rock_count - rock_diff) / rock_diff;
                    extra_height += height_diff * catchup;
                    rock_count += rock_diff * catchup;
                }

                pattern_cache.insert(key, (rocks.len() as u64, rock_count));
            }

            // print_view(&rock, &rocks);
        }

        rocks.len() as u64 + extra_height
    }
}

impl Solver for Solution {
    fn get_input(&self) -> &'static str {
        INPUT
    }

    fn with_input(&mut self, input: &str) {
        self.jets = input.chars().map(Dir::from).collect();
        self.shapes = vec![
            Shape::Flat,
            Shape::Cross,
            Shape::Angle,
            Shape::Tall,
            Shape::Block,
        ];
    }

    fn solve_part1(&self) -> String {
        self.get_rock_height(2022).to_string()
    }

    fn solve_part2(&self) -> String {
        self.get_rock_height(1_000_000_000_000).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_example_input<'a>() -> &'a str {
        // INPUT
        ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
    }

    #[test]
    fn test_solution_example1() {
        let mut solver = Solution::default();
        solver.with_input(get_example_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "3068");
    }

    #[test]
    fn test_solution_example2() {
        let mut solver = Solution::default();
        solver.with_input(get_example_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "1514285714288");
    }

    #[test]
    fn test_solution_part1() {
        let mut solver = Solution::default();
        solver.with_input(solver.get_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "3147");
    }

    #[test]
    fn test_solution_part2() {
        let mut solver = Solution::default();
        solver.with_input(solver.get_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "1532163742758");
    }
}

#[allow(dead_code)]
fn print_view(rock: &Rock, rocks: &Vec<u8>) {
    clear_terminal();
    let max_y = rocks.len();
    println!("--{max_y:0>4}--");
    let max_y = max_y.max(30);
    let min_y = if max_y > 30 { max_y - 30 } else { 0 };
    let shape = rock.shape.get_pieces(rock.pos.0);
    for y in (min_y..max_y + 10).rev() {
        print!("|");
        let rocks = rocks.get(y).unwrap_or(&0u8);
        let rock = if y >= rock.pos.1 && y < rock.pos.1 + shape.len() {
            shape.get(y - rock.pos.1).unwrap_or(&0u8)
        } else {
            &0u8
        };

        let line = format!("{:#09b}", rock | rocks)
            .trim_start_matches("0b")
            .replace('0', ".")
            .replace('1', "X");

        print!("{line}");
        println!("|{:0>4}", y + 1);
        if y == 0 {
            println!("+-------+");
        }
    }
    sleep(Duration::from_millis(140));
}

const INPUT: &str = "><<<><<>>>><<><<<<>>>><<<>>><<<>>>><<<><>><<<>>>><<<><>>><<>>>><>>>><>><<<>>><<<<>><>><<>>><<<<>>>><>>><<<>><<<<><<<<>>><<>>>><><>>><<<<>><<>>>><<><<<<><<>>>><<<><<><>>>><<<><<<>>><<<>>>><<<>>><<>>>><<<><<<<>>>><<>><<<<>>>><<>>>><<<>><<>>>><>>>><<<<>>><>>>><>>>><<<<>><<<>><<<>>>><<<>>>><<<><<<>>>><>><<>><>>><<<>>><>><<<<><<<<><>>><<>>>><<<<>>>><<<<><<<>>>><><<>>><<<>>>><<<>>>><><<>>><<<>>><<<<>><<<<>>>><>><<<<><<<>>><<>>>><>><<<>>>><<<>>>><<>>>><<<><>><>>><<<>><>>>><<<<>><<<>><<><<<<>>><<<>>>><>>><<<>>><<<<>>><<>><<><<<<>>>><>><>>>><>>>><<<>>>><>>><<<>>><<<<>>><<>>>><<>>><<<>>><<>>><<<>><<<>>><<<>><>>><<>><<<><<><<<<><<<<>><>><><>>><<<><>><<<>>>><<<>>>><<<>><<<>><<<>>>><>><<<><<<><>>><<<>><<<><<<<><>>><<<<>>>><>>><<<>><><<>>>><<<>><<<>>>><<<>>><<><<>><<<<><<>>>><<<>><<<<>>>><<>>><<<<>>><<>>><<<<><><<<<>>>><<>>>><>>>><<<>>><<<<>><<>>><<<>><<>>>><>>>><<<>><<<>>><<<>><<>><<>>>><<<<>><<>><<>>>><<>>><<>>><<<>>><<<<>>><<>><<<>><>>>><<<<><<>>>><<<<>>><>>><>><>><<<<><>>>><<<<><<<<>>><<<<>>>><>>><<<<><<<>>>><<<>>><><>>>><>><>><<<<>>><<<<>>>><<<<>>><<<<>><<<>>>><<<><<>>><>><>><<>>>><>><<<<>><<><>><<<>>><><<>>>><>>>><>>>><<<>>>><<<>>>><<<>>><<>>>><<<<>><<<><<<>>>><<<<>>>><<<<>><<>><<<<>><<><>>><><<<<><><>><<>>>><<<<>>>><>>>><<<>>><<>>>><>>><<>>>><<<<><<>>>><<<>>>><>>><<>>>><>>><>><<<<><><>><<<<>>>><>>>><<>>>><<<<>>>><>>>><<><<<>><<<>><<<><<<<>>>><<>>><<>>><<><<<>>><<<>>>><<<>>>><<<<><<<<><<<>>><>>>><<<>><<<>>>><<<>>>><<<<>>><<>>>><>>>><>><<<<>><<>><>><<<<>>>><<><<<><<<<>>><<<>><<<>>><>>><<<<><<>>><<>><<>>>><<>><<><<<>><<<>>>><<><<<<>>>><<<<>>>><<<>><<<>><<<<>>>><<<>><<<<>>><>>>><<<<>><><><<>>><<<><<<>><><<<>>>><<>>>><<>>>><<<>>>><<<<>><<<<>>><<>><<<<>>><<<>><<<><<<<>><>>><<<><<<<>>>><<<<>>><<><<>><><<<<><<>><>>>><<<><<<<>>><<>><<<<>><<<>>>><<>>>><<><>>>><><<<>>>><<<<><><<<<>>><<<>>><<<><<<<>>>><<<<>>>><<<<>>>><<<<><><<<<>>><<<<>><>>><>>><<<>><>>>><<<<>><>><<<<><<<><>><<>>>><<<<><<<><<<>>><<<>>>><>>><<><<>><<<<><<<>>>><<<<>><<>>>><><<<<><><<>><<<><<<<><<>>><<>><<<>>>><<<<>>><<<>>>><<<<>>>><<<<>>><<<<>><<<<>><<<>>>><><<>>>><>>>><<<<>><><<<><><<<>>>><>>>><<>><>>>><>><<<<>>><<>>>><<>><<<<><<<<>>>><>><>>><<<<>><><<<>><<><<<<>><<>><<<<><<>><>>>><<<>><<<>>>><<<>>>><<<>>>><<<>>><<><<<><<>>><<>><<>>>><<<<>><<<<><<<<>>><>><>>>><<<<>>>><<<>>><<>>><<>>><<>><<<>><>>>><>>><>><>><<>>>><><<<<>><<<<><>>><<<>>><<<<>>><<><<>>><>><<><<<<>>>><<>>><<<<>>>><<<<><<<<>><><<<<>>><<<>><<<<>>>><<>>><<>><<<<>>><<>>><<<>><>>>><>>>><><<>><<<>>>><<<<>>>><<><<<<>>>><<<><<><<<<><<<<>>><<<<><<<<><<>><<<<>>>><<>><<<<>><<>>><<<>>>><<<><<<>><<<>>><>>><>><<<>><<>><<<>><<<>><<<<>>>><<<<>><>><<>>><<<><>><<<<>>><<<<>>><><<><<<<>>><<>>>><<<<>>>><<<<><>>>><<<<>>>><<<>>>><<<<>><<>><<>>><><>><>>><>>>><>>><>>>><<<>>><<><>>><>>><><>>>><<<><<>>>><<><<<><<>>><><<<>>><<>><<<<>><<<><>><<>><<<>><<<<>><<>><<>><>>>><>>><<<<>>>><<>>><<<>>>><<<<>>>><<><>><<>><<<<>>><>>>><>>><<>><<>>>><>>>><<>>>><<<>>><<>><<<><<<<>>><<><<>>><<<>>><><>><<<<>><<><<<>><><<<<><><<<><<<><<<<>><<>>>><<>><<<>>><<<><<<<>><<>><<<>>><<<<><>>><><<<<>>><<><<><>><>>>><>>><<<<><<>>><<<>><<<>><<<>>><>><>>>><<<>>><<<>>><><>>><<><<>><<<><<<>><<>>><<>>>><<<>>><<><>><>><<<>>><<<<>><<<><>>>><<<>>>><<>>>><<<<>>>><<<>>>><<>>>><<<><<<><<<>><>>>><<>>><>>>><<>>><<<>>>><>>>><<<<>>>><>>>><<<><<<<>><<<>>>><<<>>><<<>>>><<<<>><<<>><<<>>>><<<<>>>><<<<>>>><<<>>><>>><>>>><<<<>><<>>><>>>><>>><>>><<<>>><<>>>><<<<>>>><>>><<<>><<>><<><<<>>>><>>><<><>>><<>>><<>>><<<<>><<<>>>><<<><<<<><<><<<<>>><<>>>><<<>><>><<>>><<<<>>>><<>>>><<<<>>>><<<>>><<<>>>><>>><<><<<<>><<<><<<<>>>><>>><<<<>>><<<<>><<<>>>><>><<<<><<<><<<>>>><<<>>><>>>><<<<>><<<<>>><<>>>><>>>><<<>>>><<><>>><<<><<<<><<<<>><><<<<>><>>><<>>><<<>><<<<>>>><<<<>><<<<><<<<>><<<>>>><<<<>>><<<>><<<>><<<<>><>>>><>><<<>>>><<<<>>>><<><<<>><><<<><<>>><>>>><><<<<>>>><<>><<<>>>><<<<>><<<<><>>>><<<>>><<<<><<<><>><<<>>><>><<<><<<<><<<>>>><<<<>>>><<<>>>><<<<><>><<<<>>><<<><<>>><>><<>><<<<>><<>>>><>><<>><<<><<>>><<>><<<<>><<<<>>>><<>>><<<>>><<><>>>><<>>>><<><<<>><<<>>><>><><<<<>>>><>>><<>><<><<<<>>>><<>>>><<<>><<<>>><<<<>>>><>>>><<>><>>><<<>><>><>>>><<<<><<>>>><>>>><><>>><<<>><<<>><<<><>>>><<>>>><<<<>>><<><<>>>><<<<>>>><><<<>><<<>>><<<>>>><<>>>><<<<><<<<>>><<<<>><>>><<<>>>><>>><<<<>>><<>>><<<>>>><<><<>>><<<><<<>>>><<<>>><<<><><<<<>><<<>>>><<><<<<><>>>><><<<>>><<<>><<<>>><>>><<<>>>><<><<<><<>>><<>><<>>>><<<>><<>>>><<>><<<>>>><<<>><<<>><><<><<<>><<<>>><>><>>>><<>><<>><<>><<<>>><<<<>><<>>>><<<>><<<><<><<<>>><>><<<>><<>>>><<<<>><<<>><<<>>><<>>><<>>><<<><>><<>>><<>><<<<>>><<><>>><<<>>><<>>>><<<<>><<<<>><>>><><<>>><>>>><<><<<<><<<<><>>>><<>>>><<<<>>><>>><<><<<><>>><<<>>><<<<>>>><<<><>><<<<>><<>>><<<<><>>><>>><<<<>>>><>>><<>><<<>>>><>>>><<>>>><<<<>>><<<>><>>>><<<><><<<<><<<<>>>><<<>>><><<><<<>><>><>><<<<>>><<><<<<>>><<>>>><>>><<><<<<>><<>>>><<><<<<>><<>><>><<<><<><<<>>>><<<<><<<<>>><<>>>><<<<>>><<<<>>>><<<>>>><><<<><>>>><>>><<>>>><<<>>>><>>>><>>><>><>>><>>><<<>>><>>>><>><<<<><<<><<<<>>><<<<>>><<<<>><<<><<<>><<<>>><<<>>>><<>><<>>>><<>>>><><<><><<<<><>><<<<><<<<>>>><<>>>><<<<>>><<><<<>>>><>>><<><<>>><<<<>><>>><<<>>><<>><<<<>>><<<<>>><><<<><><<<>><>>><<>>>><<>><<<<>>><<<<><<<><>><<<<>><><><<>><>>>><<<<><<<>>>><<<<>>>><<>><<<<><><<><>>>><<<>>><<>>>><<<><>>><><<<>>>><<<<><<<<>><<<>><>><<<<>>>><<<<>>><<<>><>><<>>><><<<>>>><<<<>>><<><>>><<>><<<><><<<<>>>><<<>>><<>>>><><>><><<<<>><<<<>>>><>>><><<<>>>><>><><<<<>>>><>>><<<<>><<<<>>>><<><>>>><<<<>><<<<>>>><<<>>><<<<><<<<>><>>><<>>>><<>>>><<<><<>>>><<<>>>><><>>>><<<<>><<<>><<>>><<<<><<<>>>><<>><<<><<><<<<>><<<>><><>>>><<>><<<>>>><<<<>>><<>><<<<><<>><>>><><<<<><>>>><<<>>>><<<<>><>>>><><>><<>>><<<>>>><>><<><<<<>>>><<<>><<<>><<>>><<><<<<>>><<<>><<<>>><<>>>><<>>>><<<<>>>><>><<<><<>><<>>><<<<>><<<>><<<<>>><><>>>><<>>>><>>><>><<<<>>><>>><>>><<<><<>>>><<<>>>><<>>><<<>>><<<>><<<<><>>>><<<>><<<>><<<<>><>>>><<><<<<><>>><<<<>>><<>>><<<<>>><<<><>><<<>>><<>><<<<>>>><<>>><<<<>>>><<<<><<<>><><<<>>>><<<<>>>><<<<>>><<>>>><<>><<>><<<>><<<>><<<<>><><>><>><<<><>>>><<<><<<>><<<><<<>>><<<><<<<>>>><><<>>><><<<<><<<<>>>><<<><<<>><<><<>><<<>>>><><<>>>><<<<>><>><><>>>><<<>>>><<>>>><<>>>><>>><>>><<<<><<<<><<<<><<<<>>><><<<<>>>><><>>>><<<><><<>>><<<<><<<>><<<<>><<>>><><<<<>><><<<>>>><>>>><>>><<<>><>><<<<>><>><<<>>>><<><<>><>><<<<>>><<>><<>>>><<<<>><<<<>><>><<<>>><>><><<<<>><<<<><>><<<>>>><<<<><>>>><<<<>><>>>><>>><<<>>><<>>><>>><<<<>><<><<<>>><>><>>><>>><<<<>>>><<><>>><<>>>><<<<>>>><<>><<<>><>><<<<>>>><<<>><<>><>><>><><<><>><><<>>><<<>><>>>><<<<><<<<><<<<>>><<<<>>>><<<<>>>><<<<>>>><<>><>><<<><>>><<<>>>><<<>>><<<><<>>>><>><<>>>><<><>>><<>><<<<>><>><<<<>>>><<<>>>><<<>>><<<<>>>><<><<>>>><><<>>><>><<>><<<<>>>><<><>><>><<<<>><>><<<<>>>><><<<<><>><<<>>>><<<<>>>><<>>>><<>>><<<<>>><>>>><<><<<<>><>><>><<<<>>>><<>>>><<<<>>><<><<<>><<<>>>><<>>>><>>><>><<<>>><<>>>><><<<<>>>><<<>>>><<><<<>>><><><<<>><<>>>><<<>><<><<<<>><>>><>>><><<>>><<>><<>>>><<>><><<>>><>><>>><>>>><<<<>>>><<<<>><>><<<<>><<<<><<>>>><>>>><<>><<<>>><><<<<>>>><<<<><<><<<<>>><<><>><<<<>><<>><>>>><><<<>>>><<<<>>><<<>>>><<>><<<>>><<<<><>>>><<<<>>>><<<<><<<<>>><<<>><><<<<>><<>>><<<>>>><<>>>><<<><<>><<<<><<><>>>><<<<><>><<<>><<<<>><>><<<>>>><<>><><<<>>><<<<>>>><>>>><<>><<<<><<>>>><<>>><>>><<>>>><>>><<<<>>><<<>>><<<>><<<>>><<>>><<<<>>>><<<>><<<<>>>><<<><<<<>>><<<<>><<<<>>><<<<>><>><<<>>>><>>><<>>>><<>>><<>><>>><<<<>><>>>><<<>>>><<<>>><<<<><>>><>><<>>><>>>><<<>><<>><<>>><<<><>>><<>>><>>><<>>>><>>><<<<><>>>><>><<<>><<<<>><<>>><<>>>><<<>><<<>>><<>>>><<<><<<<>>>><<<>><>>><<<>><<>>><>>><<<>>>><<><<>>>><>><<<<>>>><<><<<>>>><<<<>>><<><<<>>>><<<>><<>><<<<>>><>>>><>>>><<>>><><<<<>>>><<<<>>><<<><<<<>><<<<>><<<<>><<<>><<<<>><>>>><<<><<<<><><<>>>><<>><>><<<<>>><<<>>><<<<>>>><<<>>>><<<>><<<<><<<>>>><<>>><<>><<<><<>>>><<<<>>><>>><<<>>>><>><<>>><<<<>>><<<>>>><><<>><<>><<<<><<>><>>><<<>>><<>>>><<<<>>><<<>>><<<>>><<><<<<>>>><<<>>><><<<><>><<>>>><><>><><<>>><>>><<<<>><<<<>><>>>><><<<><><><<>><>>><<>>><<<<>>>><<><<<<>><<<><>>>><>>><<>>>><>>><><><<<>>>><<<<>>>><<>><<<<>><<><>><>>>><>><<<>>>><><<<>><<<>><<>><<<>>><><>><><<>>><<<>><<<><<<<>>>><>>>><<<<><>>><<<<>>>><>>>><<<>>>><<<<>><<<>>><<><>>><<<<><<>>><<<<><<>>>><<<<>>><>>><<<><<<>><>>><<<<>>>><<<<>>><<>>>><<<<>>>><<<>>><<<<><<<>>><>>>><<><>>><<<>>>><>><<<<><>><>>>><>><<>><>><<>>><<<<>>><>>>><<<<>><<><>>><<<<>>><>>>><>>><<>><<<<>><<><<<>>>><>>><<><<>>>><<>>><<>>>><>>><>>>><<<<><<<<>>><<<><<>>>><<<<>><<<>><<><<><<><<<><<<<><>><<>>>><<<<>>><<<>>><<<<>>><<<><<<>>><<<>>>><<<>>><<<<>><<<<>><<><<<<>>>><<<>>><<>>><>><>><<<><<<<>>><<><<<>>><<<<>>>><>>><<<>>>><<<>><<>>><<<>>><<<>><>><>>>><<<<>>>><<<>><>><<<>><<<<>>>><<>>>><<<<><<><>>><<>>><<<<>>>><<<<><<>>><<<>><>><<<<>>>><<<<>><<<><<><<><><<<<><<>>><<<><<>><<<<><<<<><<<>><>><<<<>>>><<<>><<<<><<>><<<>>>><<>><<>><>><<<<>><>>>><<<<>><<<>><<>>>><<><<<>><<<>>>><<>>>><<<<>><>>><<<<>><>>><<<>>><<>>>><<<<>><<<><>><<><<<>><<>>>><<<>>><>><>>>><<>><>><<>><<<><<<<>>>><<<<>>>><<>>>><<<<>>>><>>>><<>>><>>>><<>>><<<<>><<<<>><<<>><<>>><<<<>>>><<<<>>>><<<>>>><<>><<<>>><<>>>><<>><<>>>><<>>>><<><<<<>><<<>>><<<><<>>><<>><<<<><<>>><<<<>>>><<><<<<><>><<>>><<<<>>>><<<<><<<>>><><>>>><>><<<><>>>><<<>>>><>>><<><<>><><<<<>>>><<>>>><<<><<>>>><>>>><<<<><<<<>>>><<><>>>><>>><><<>>><<<>>><<<<><>><<<>><<<><<<><<<<>><<<>>><<<<>>>><<<>>>><<<>>><<<>>><>>><<>><<<>>><<><<<><>>>><>>><<<<>>><<><<<<>>>><<<><<<<>><<>>>><<>>><<<<>>>><<>>><><<<>>><<>><<>><<>>>><<<<><<<>>><<<<>>><<<<>>>><<<>>>><<<<>>><<<><>>><>>><<>>>><>>><<>>><<>><<>>>><<>>><<<<>>>><<<<>><<<>>>><<<<>>>><<<<>><<<><>><<>><>>><<<<><<<>><><<<>>>><<<>>><<>>><<>>>><<<<>>>><<<><<<>>><<<>><<<>><<<<>>><<<<>><<<>>><<<><<<><<>><><>><<<>>><<<>>><<>>><<<>>><<>>><<<<>><<<>><>><<<<>><<<<>><<>><<<>><<<<>>>><<<><<<<><<<<>><>>><<>><<><<>>><<<><<>>><<<><<<<>><>>><<>><>>><<>>>><<>>>><<<<>><<><<>>><<<>>>><<><<<<>>>><><<<<>>><<<<>><<<<>><<>>><<<<>>>><>>><<<<>><<<<>><><<<<>>>><<<>>><<>><<<<>>><<<><<<>>><>><<>>>><<>>><>><>><<<<>>>><><<<<><>>><<><<>><<<>>><>><<<>>>><<<<>>><<<>><<<<>>>><>><<<>>>><<<>>><<>>>><<>><<<<>>><<>>><<<>><<><>><>><<>><<<><><>>><<<>><<<>><<<<>>><>>><><<<>><<<>>>><<<><<>>>><>>>><>><>>><<<<><<<>><<<<>><<><<<<>>>><<>><<>>>><<<><<<>>><<<<>>><<<>>>><<>>>><<<>>><<<<>><<<>>><>>><>>>><<<>><<<<><<<<><><<<>>><><<<<>><<<>>>><<<<>>>><<><<><<<><<<><><<>>";
