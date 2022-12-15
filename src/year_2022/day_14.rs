use std::{
    collections::HashSet,
    i16::MAX,
    ops::{Add, AddAssign},
    str::FromStr,
    thread,
    time::Duration,
};

use crate::solver::Solver;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(i16, i16);

impl FromStr for Point {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(Self(x.parse().unwrap(), y.parse().unwrap()))
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[derive(Debug)]
struct Line(Vec<Point>);

#[derive(Default, Debug)]
pub struct Solution {
    rock: HashSet<Point>,
    min_x: i16,
    min_y: i16,
    max_x: i16,
    max_y: i16,
}

impl Solver for Solution {
    fn get_input(&self) -> &'static str {
        INPUT
    }

    fn with_input(&mut self, input: &str) {
        self.min_x = MAX;
        self.min_y = MAX;
        input.lines().for_each(|l| {
            let lines: Vec<Point> = l
                .split(" -> ")
                .map(|c| c.parse().unwrap())
                .inspect(|Point(x, y)| {
                    self.min_x = self.min_x.min(*x);
                    self.min_y = self.min_y.min(*y);
                    self.max_y = self.max_y.max(*y);
                    self.max_x = self.max_x.max(*x);
                })
                .collect();
            lines.windows(2).for_each(|p| {
                let Point(x1, y1) = p[0];
                let Point(x2, y2) = p[1];

                if x1 == x2 {
                    for y in y1.min(y2)..=y1.max(y2) {
                        self.rock.insert(Point(x1, y));
                    }
                } else if y1 == y2 {
                    for x in x1.min(x2)..=x1.max(x2) {
                        self.rock.insert(Point(x, y1));
                    }
                } else {
                    println!("WARN: some lines are not purely vertical or horizontal")
                }
            })
        });
    }

    fn solve_part1(&self) -> String {
        let mut in_the_abyss = false;
        let mut sand_count = 0;

        let mut sands: HashSet<Point> = HashSet::new();
        let dirs = [Point(0, 1), Point(-1, 1), Point(1, 1)];
        while !in_the_abyss {
            let mut sand = Point(500, 0);
            let mut is_done = false;
            while !is_done {
                is_done = true;
                for dir in dirs {
                    let target = sand + dir;
                    if !self.rock.contains(&target) && !sands.contains(&target) {
                        sand = target;
                        is_done = false;
                        break;
                    }
                }
                if is_done {
                    sand_count += 1;
                    sands.insert(sand);
                } else if sand.1 > self.max_y {
                    in_the_abyss = true;
                    break;
                }
            }
        }

        sand_count.to_string()
    }

    fn solve_part2(&self) -> String {
        let mut standing_on_top = false;
        let mut sand_count = 0;
        // self.clear_terminal();
        // self.draw_rocks();
        let mut sands: HashSet<Point> = HashSet::new();
        let dirs = [Point(0, 1), Point(-1, 1), Point(1, 1)];
        while !standing_on_top {
            let mut sand = Point(500, 0);
            let mut is_done = false;
            while !is_done {
                is_done = true;
                if sand.1 != self.max_y + 1 {
                    for dir in dirs {
                        let target = sand + dir;
                        if !self.rock.contains(&target) && !sands.contains(&target) {
                            // self.animate(&target, &sand);
                            sand = target;
                            is_done = false;
                            break;
                        }
                    }
                }
                if is_done {
                    sand_count += 1;
                    sands.insert(sand);
                    if sand.1 == 0 {
                        standing_on_top = true;
                        break;
                    }
                }
            }

            if sand_count % 100 == 0 {
                thread::sleep(Duration::from_millis(50));
                self.clear_terminal();
                self.draw_sand(&sands);
                self.draw_rocks();
            }
        }

        // self.clear_terminal();
        // self.draw_sand(&sands);
        // self.draw_rocks();

        sand_count.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_example_input<'a>() -> &'a str {
        "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
    }

    #[test]
    fn test_parse_example() {
        let mut solver = Solution::default();
        solver.with_input(get_example_input());
        // solver.animate(&HashSet::new(), &Point(500, 0));
        // println!("{:#?}", solver);
    }

    #[test]
    fn test_solution_example1() {
        let mut solver = Solution::default();
        solver.with_input(get_example_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "24");
    }

    #[test]
    fn test_solution_example2() {
        let mut solver = Solution::default();
        solver.with_input(get_example_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "93");
    }

    #[test]
    fn test_parse() {
        let mut solver = Solution::default();
        solver.with_input(solver.get_input());
        // println!("{:#?}", solver);
    }

    #[test]
    fn test_solution_part1() {
        let mut solver = Solution::default();
        solver.with_input(solver.get_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "");
    }

    #[test]
    fn test_solution_part2() {
        let mut solver = Solution::default();
        solver.with_input(solver.get_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "");
    }
}

const X_OFFSET: i16 = -300;
const Y_OFFSET: i16 = 2;
impl Solution {
    fn clear_terminal(&self) {
        print!("{esc}c", esc = 27 as char);
        print!("{:\n>250}", "");
    }

    fn draw_sand(&self, sand: &HashSet<Point>) {
        sand.iter().for_each(|Point(x, y)| {
            print!("\x1b[{};{}H", *y + Y_OFFSET, *x + X_OFFSET);
            print!("o");
        });
    }
    fn draw_rocks(&self) {
        self.rock.iter().for_each(|Point(x, y)| {
            print!("\x1b[{};{}H", *y + Y_OFFSET, *x + X_OFFSET);
            print!("x");
        });

        for x in 0..500 {
            print!("\x1b[{};{}H", self.max_y + Y_OFFSET + 2, x);
            print!("x");
        }
    }

    fn animate(&self, sand: &Point, prev: &Point) {
        thread::sleep(Duration::from_millis(16));
        print!("\x1b[{};{}H", sand.1 + Y_OFFSET, sand.0 + X_OFFSET);
        print!("o");
        print!("\x1b[{};{}H", prev.1 + Y_OFFSET, prev.0 + X_OFFSET);
        print!(" ");
    }
}

const INPUT: &str = "\
521,171 -> 525,171
497,147 -> 497,137 -> 497,147 -> 499,147 -> 499,146 -> 499,147 -> 501,147 -> 501,142 -> 501,147 -> 503,147 -> 503,144 -> 503,147 -> 505,147 -> 505,140 -> 505,147
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
476,63 -> 476,58 -> 476,63 -> 478,63 -> 478,55 -> 478,63 -> 480,63 -> 480,56 -> 480,63 -> 482,63 -> 482,57 -> 482,63 -> 484,63 -> 484,55 -> 484,63 -> 486,63 -> 486,55 -> 486,63
503,109 -> 507,109
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
503,150 -> 503,154 -> 501,154 -> 501,162 -> 513,162 -> 513,154 -> 507,154 -> 507,150
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
503,150 -> 503,154 -> 501,154 -> 501,162 -> 513,162 -> 513,154 -> 507,154 -> 507,150
481,79 -> 481,81 -> 478,81 -> 478,89 -> 490,89 -> 490,81 -> 485,81 -> 485,79
488,38 -> 493,38
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
476,63 -> 476,58 -> 476,63 -> 478,63 -> 478,55 -> 478,63 -> 480,63 -> 480,56 -> 480,63 -> 482,63 -> 482,57 -> 482,63 -> 484,63 -> 484,55 -> 484,63 -> 486,63 -> 486,55 -> 486,63
482,93 -> 496,93 -> 496,92
476,63 -> 476,58 -> 476,63 -> 478,63 -> 478,55 -> 478,63 -> 480,63 -> 480,56 -> 480,63 -> 482,63 -> 482,57 -> 482,63 -> 484,63 -> 484,55 -> 484,63 -> 486,63 -> 486,55 -> 486,63
497,29 -> 502,29
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
497,147 -> 497,137 -> 497,147 -> 499,147 -> 499,146 -> 499,147 -> 501,147 -> 501,142 -> 501,147 -> 503,147 -> 503,144 -> 503,147 -> 505,147 -> 505,140 -> 505,147
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
501,32 -> 506,32
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
490,16 -> 490,20 -> 484,20 -> 484,26 -> 499,26 -> 499,20 -> 494,20 -> 494,16
503,115 -> 507,115
524,173 -> 528,173
481,79 -> 481,81 -> 478,81 -> 478,89 -> 490,89 -> 490,81 -> 485,81 -> 485,79
486,47 -> 490,47
497,147 -> 497,137 -> 497,147 -> 499,147 -> 499,146 -> 499,147 -> 501,147 -> 501,142 -> 501,147 -> 503,147 -> 503,144 -> 503,147 -> 505,147 -> 505,140 -> 505,147
476,63 -> 476,58 -> 476,63 -> 478,63 -> 478,55 -> 478,63 -> 480,63 -> 480,56 -> 480,63 -> 482,63 -> 482,57 -> 482,63 -> 484,63 -> 484,55 -> 484,63 -> 486,63 -> 486,55 -> 486,63
490,16 -> 490,20 -> 484,20 -> 484,26 -> 499,26 -> 499,20 -> 494,20 -> 494,16
476,63 -> 476,58 -> 476,63 -> 478,63 -> 478,55 -> 478,63 -> 480,63 -> 480,56 -> 480,63 -> 482,63 -> 482,57 -> 482,63 -> 484,63 -> 484,55 -> 484,63 -> 486,63 -> 486,55 -> 486,63
495,50 -> 499,50
492,41 -> 496,41
476,63 -> 476,58 -> 476,63 -> 478,63 -> 478,55 -> 478,63 -> 480,63 -> 480,56 -> 480,63 -> 482,63 -> 482,57 -> 482,63 -> 484,63 -> 484,55 -> 484,63 -> 486,63 -> 486,55 -> 486,63
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
497,147 -> 497,137 -> 497,147 -> 499,147 -> 499,146 -> 499,147 -> 501,147 -> 501,142 -> 501,147 -> 503,147 -> 503,144 -> 503,147 -> 505,147 -> 505,140 -> 505,147
522,177 -> 522,178 -> 536,178 -> 536,177
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
503,121 -> 507,121
503,150 -> 503,154 -> 501,154 -> 501,162 -> 513,162 -> 513,154 -> 507,154 -> 507,150
515,121 -> 519,121
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
476,63 -> 476,58 -> 476,63 -> 478,63 -> 478,55 -> 478,63 -> 480,63 -> 480,56 -> 480,63 -> 482,63 -> 482,57 -> 482,63 -> 484,63 -> 484,55 -> 484,63 -> 486,63 -> 486,55 -> 486,63
498,35 -> 503,35
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
503,171 -> 507,171
509,121 -> 513,121
494,12 -> 494,13 -> 507,13 -> 507,12
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
500,112 -> 504,112
488,124 -> 488,126 -> 485,126 -> 485,129 -> 496,129 -> 496,126 -> 491,126 -> 491,124
476,63 -> 476,58 -> 476,63 -> 478,63 -> 478,55 -> 478,63 -> 480,63 -> 480,56 -> 480,63 -> 482,63 -> 482,57 -> 482,63 -> 484,63 -> 484,55 -> 484,63 -> 486,63 -> 486,55 -> 486,63
488,124 -> 488,126 -> 485,126 -> 485,129 -> 496,129 -> 496,126 -> 491,126 -> 491,124
476,63 -> 476,58 -> 476,63 -> 478,63 -> 478,55 -> 478,63 -> 480,63 -> 480,56 -> 480,63 -> 482,63 -> 482,57 -> 482,63 -> 484,63 -> 484,55 -> 484,63 -> 486,63 -> 486,55 -> 486,63
512,173 -> 516,173
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
502,38 -> 507,38
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
503,150 -> 503,154 -> 501,154 -> 501,162 -> 513,162 -> 513,154 -> 507,154 -> 507,150
497,147 -> 497,137 -> 497,147 -> 499,147 -> 499,146 -> 499,147 -> 501,147 -> 501,142 -> 501,147 -> 503,147 -> 503,144 -> 503,147 -> 505,147 -> 505,140 -> 505,147
497,147 -> 497,137 -> 497,147 -> 499,147 -> 499,146 -> 499,147 -> 501,147 -> 501,142 -> 501,147 -> 503,147 -> 503,144 -> 503,147 -> 505,147 -> 505,140 -> 505,147
506,169 -> 510,169
490,16 -> 490,20 -> 484,20 -> 484,26 -> 499,26 -> 499,20 -> 494,20 -> 494,16
497,147 -> 497,137 -> 497,147 -> 499,147 -> 499,146 -> 499,147 -> 501,147 -> 501,142 -> 501,147 -> 503,147 -> 503,144 -> 503,147 -> 505,147 -> 505,140 -> 505,147
503,150 -> 503,154 -> 501,154 -> 501,162 -> 513,162 -> 513,154 -> 507,154 -> 507,150
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
488,124 -> 488,126 -> 485,126 -> 485,129 -> 496,129 -> 496,126 -> 491,126 -> 491,124
491,121 -> 495,121
497,147 -> 497,137 -> 497,147 -> 499,147 -> 499,146 -> 499,147 -> 501,147 -> 501,142 -> 501,147 -> 503,147 -> 503,144 -> 503,147 -> 505,147 -> 505,140 -> 505,147
518,173 -> 522,173
501,50 -> 505,50
497,115 -> 501,115
498,47 -> 502,47
476,63 -> 476,58 -> 476,63 -> 478,63 -> 478,55 -> 478,63 -> 480,63 -> 480,56 -> 480,63 -> 482,63 -> 482,57 -> 482,63 -> 484,63 -> 484,55 -> 484,63 -> 486,63 -> 486,55 -> 486,63
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
518,169 -> 522,169
476,63 -> 476,58 -> 476,63 -> 478,63 -> 478,55 -> 478,63 -> 480,63 -> 480,56 -> 480,63 -> 482,63 -> 482,57 -> 482,63 -> 484,63 -> 484,55 -> 484,63 -> 486,63 -> 486,55 -> 486,63
489,44 -> 493,44
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
481,79 -> 481,81 -> 478,81 -> 478,89 -> 490,89 -> 490,81 -> 485,81 -> 485,79
509,167 -> 513,167
476,63 -> 476,58 -> 476,63 -> 478,63 -> 478,55 -> 478,63 -> 480,63 -> 480,56 -> 480,63 -> 482,63 -> 482,57 -> 482,63 -> 484,63 -> 484,55 -> 484,63 -> 486,63 -> 486,55 -> 486,63
487,133 -> 487,134 -> 501,134
481,79 -> 481,81 -> 478,81 -> 478,89 -> 490,89 -> 490,81 -> 485,81 -> 485,79
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
512,118 -> 516,118
490,16 -> 490,20 -> 484,20 -> 484,26 -> 499,26 -> 499,20 -> 494,20 -> 494,16
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
512,165 -> 516,165
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
491,35 -> 496,35
500,173 -> 504,173
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
490,16 -> 490,20 -> 484,20 -> 484,26 -> 499,26 -> 499,20 -> 494,20 -> 494,16
503,150 -> 503,154 -> 501,154 -> 501,162 -> 513,162 -> 513,154 -> 507,154 -> 507,150
483,50 -> 487,50
481,79 -> 481,81 -> 478,81 -> 478,89 -> 490,89 -> 490,81 -> 485,81 -> 485,79
500,118 -> 504,118
497,147 -> 497,137 -> 497,147 -> 499,147 -> 499,146 -> 499,147 -> 501,147 -> 501,142 -> 501,147 -> 503,147 -> 503,144 -> 503,147 -> 505,147 -> 505,140 -> 505,147
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
492,47 -> 496,47
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
497,147 -> 497,137 -> 497,147 -> 499,147 -> 499,146 -> 499,147 -> 501,147 -> 501,142 -> 501,147 -> 503,147 -> 503,144 -> 503,147 -> 505,147 -> 505,140 -> 505,147
476,63 -> 476,58 -> 476,63 -> 478,63 -> 478,55 -> 478,63 -> 480,63 -> 480,56 -> 480,63 -> 482,63 -> 482,57 -> 482,63 -> 484,63 -> 484,55 -> 484,63 -> 486,63 -> 486,55 -> 486,63
495,38 -> 500,38
494,12 -> 494,13 -> 507,13 -> 507,12
476,63 -> 476,58 -> 476,63 -> 478,63 -> 478,55 -> 478,63 -> 480,63 -> 480,56 -> 480,63 -> 482,63 -> 482,57 -> 482,63 -> 484,63 -> 484,55 -> 484,63 -> 486,63 -> 486,55 -> 486,63
497,147 -> 497,137 -> 497,147 -> 499,147 -> 499,146 -> 499,147 -> 501,147 -> 501,142 -> 501,147 -> 503,147 -> 503,144 -> 503,147 -> 505,147 -> 505,140 -> 505,147
494,118 -> 498,118
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
488,124 -> 488,126 -> 485,126 -> 485,129 -> 496,129 -> 496,126 -> 491,126 -> 491,124
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
497,147 -> 497,137 -> 497,147 -> 499,147 -> 499,146 -> 499,147 -> 501,147 -> 501,142 -> 501,147 -> 503,147 -> 503,144 -> 503,147 -> 505,147 -> 505,140 -> 505,147
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
488,124 -> 488,126 -> 485,126 -> 485,129 -> 496,129 -> 496,126 -> 491,126 -> 491,124
509,115 -> 513,115
497,147 -> 497,137 -> 497,147 -> 499,147 -> 499,146 -> 499,147 -> 501,147 -> 501,142 -> 501,147 -> 503,147 -> 503,144 -> 503,147 -> 505,147 -> 505,140 -> 505,147
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
506,173 -> 510,173
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
481,79 -> 481,81 -> 478,81 -> 478,89 -> 490,89 -> 490,81 -> 485,81 -> 485,79
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
505,35 -> 510,35
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
506,118 -> 510,118
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
503,150 -> 503,154 -> 501,154 -> 501,162 -> 513,162 -> 513,154 -> 507,154 -> 507,150
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
509,171 -> 513,171
494,12 -> 494,13 -> 507,13 -> 507,12
509,38 -> 514,38
488,124 -> 488,126 -> 485,126 -> 485,129 -> 496,129 -> 496,126 -> 491,126 -> 491,124
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
476,63 -> 476,58 -> 476,63 -> 478,63 -> 478,55 -> 478,63 -> 480,63 -> 480,56 -> 480,63 -> 482,63 -> 482,57 -> 482,63 -> 484,63 -> 484,55 -> 484,63 -> 486,63 -> 486,55 -> 486,63
495,44 -> 499,44
482,93 -> 496,93 -> 496,92
515,171 -> 519,171
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
481,79 -> 481,81 -> 478,81 -> 478,89 -> 490,89 -> 490,81 -> 485,81 -> 485,79
522,177 -> 522,178 -> 536,178 -> 536,177
476,63 -> 476,58 -> 476,63 -> 478,63 -> 478,55 -> 478,63 -> 480,63 -> 480,56 -> 480,63 -> 482,63 -> 482,57 -> 482,63 -> 484,63 -> 484,55 -> 484,63 -> 486,63 -> 486,55 -> 486,63
497,147 -> 497,137 -> 497,147 -> 499,147 -> 499,146 -> 499,147 -> 501,147 -> 501,142 -> 501,147 -> 503,147 -> 503,144 -> 503,147 -> 505,147 -> 505,140 -> 505,147
515,167 -> 519,167
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
506,112 -> 510,112
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
522,177 -> 522,178 -> 536,178 -> 536,177
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
488,124 -> 488,126 -> 485,126 -> 485,129 -> 496,129 -> 496,126 -> 491,126 -> 491,124
487,133 -> 487,134 -> 501,134
497,121 -> 501,121
476,63 -> 476,58 -> 476,63 -> 478,63 -> 478,55 -> 478,63 -> 480,63 -> 480,56 -> 480,63 -> 482,63 -> 482,57 -> 482,63 -> 484,63 -> 484,55 -> 484,63 -> 486,63 -> 486,55 -> 486,63
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106
489,50 -> 493,50
465,76 -> 465,71 -> 465,76 -> 467,76 -> 467,72 -> 467,76 -> 469,76 -> 469,68 -> 469,76 -> 471,76 -> 471,73 -> 471,76 -> 473,76 -> 473,71 -> 473,76 -> 475,76 -> 475,70 -> 475,76 -> 477,76 -> 477,71 -> 477,76 -> 479,76 -> 479,73 -> 479,76 -> 481,76 -> 481,74 -> 481,76 -> 483,76 -> 483,72 -> 483,76
512,169 -> 516,169
490,16 -> 490,20 -> 484,20 -> 484,26 -> 499,26 -> 499,20 -> 494,20 -> 494,16
494,32 -> 499,32
490,16 -> 490,20 -> 484,20 -> 484,26 -> 499,26 -> 499,20 -> 494,20 -> 494,16
488,106 -> 488,97 -> 488,106 -> 490,106 -> 490,100 -> 490,106 -> 492,106 -> 492,101 -> 492,106 -> 494,106 -> 494,105 -> 494,106 -> 496,106 -> 496,96 -> 496,106 -> 498,106 -> 498,102 -> 498,106 -> 500,106 -> 500,101 -> 500,106 -> 502,106 -> 502,102 -> 502,106 -> 504,106 -> 504,100 -> 504,106";
