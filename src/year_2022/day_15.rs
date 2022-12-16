use std::{
    collections::HashSet,
    ops::{Range, RangeBounds, RangeInclusive},
    str::FromStr,
};

use crate::solver::Solver;

#[derive(Debug, Hash)]
struct Position(isize, isize);

#[derive(Debug)]
struct Beacon {
    pos: Position,
}

#[derive(Debug)]
struct Sensor {
    pos: Position,
    dist_to_beacon: isize,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    beacon: Beacon,
}

impl FromStr for Sensor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sensor, beacon) = s
            .trim_start_matches("Sensor at x=")
            .split_once(": closest beacon is at x=")
            .unwrap();
        let (sx, sy) = sensor.split_once(", y=").unwrap();
        let (bx, by) = beacon.split_once(", y=").unwrap();
        let bx: isize = bx.parse().unwrap();
        let by: isize = by.parse().unwrap();
        let sx: isize = sx.parse().unwrap();
        let sy: isize = sy.parse().unwrap();

        let dist = (sx - bx).abs() + (sy - by).abs();

        let beacon = Beacon {
            pos: Position(bx, by),
        };

        let sensor = Sensor {
            pos: Position(sx, sy),
            dist_to_beacon: dist,
            min_x: sx - dist,
            max_x: sx + dist,
            min_y: sy - dist,
            max_y: sy + dist,
            beacon,
        };
        Ok(sensor)
    }
}

#[derive(Default, Debug)]
pub struct Solution {
    sensors: Vec<Sensor>,
    part1_row: isize,
    part2_max: isize,
}

impl Solver for Solution {
    fn get_input(&self) -> &'static str {
        INPUT
    }

    fn with_input(&mut self, input: &str) {
        self.part1_row = input.lines().next().unwrap().parse().unwrap();
        self.part2_max = self.part1_row * 2;
        self.sensors = input.lines().skip(1).map(|l| l.parse().unwrap()).collect();
    }

    fn solve_part1(&self) -> String {
        let mut no_beacons: HashSet<isize> = HashSet::new();

        let y = self.part1_row;
        let mut ranges = self
            .sensors
            .iter()
            .filter(|s| (s.min_y..=s.max_y).contains(&y))
            .map(|s| {
                let offset = (y - s.pos.1).abs();
                (s.min_x + offset, s.max_x - offset)
            })
            .collect::<Vec<(isize, isize)>>();

        ranges.sort_by_key(|r| r.0);
        let first = ranges.first().unwrap();
        let merged_ranges = ranges.iter().skip(1).fold(vec![first], |mut acc, r| {
            let last = acc.last_mut().unwrap();
            if r.0 <= last.1 {
                last.1 = r.1;
            } else {
                acc.push(r);
            }
            acc
        });

        let beacon_count = self.sensors.iter().filter(|s| s.beacon.pos.1 == y).count();

        no_beacons.len().to_string()
    }

    fn solve_part2(&self) -> String {
        let mut b = 0;
        for y in 0..self.part2_max {
            let ranges = self
                .sensors
                .iter()
                .filter(|s| (s.min_y..=s.max_y).contains(&y))
                .map(|s| {
                    let offset = (y - s.pos.1).abs();
                    0.max(s.min_x + offset)..=self.part2_max.max(s.max_x - offset)
                })
                .collect::<Vec<RangeInclusive<isize>>>();
            b += ranges.len();
        }

        b.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_example_input<'a>() -> &'a str {
        "\
10
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"
    }

    #[test]
    fn test_parse_example() {
        let mut solver = Solution::default();
        solver.with_input(get_example_input());
        println!("{:#?}", solver);
    }

    #[test]
    fn test_solution_example1() {
        let mut solver = Solution::default();
        solver.with_input(get_example_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "26");
    }

    #[test]
    fn test_solution_example2() {
        let mut solver = Solution::default();
        solver.with_input(get_example_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "");
    }

    #[test]
    fn test_parse() {
        let mut solver = Solution::default();
        solver.with_input(solver.get_input());
        println!("{:#?}", solver);
    }

    #[test]
    fn test_solution_part1() {
        let mut solver = Solution::default();
        solver.with_input(solver.get_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "5256611");
    }

    #[test]
    fn test_solution_part2() {
        let mut solver = Solution::default();
        solver.with_input(solver.get_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "");
    }
}

const INPUT: &str = "\
2000000
Sensor at x=1384790, y=3850432: closest beacon is at x=2674241, y=4192888
Sensor at x=2825953, y=288046: closest beacon is at x=2154954, y=-342775
Sensor at x=3553843, y=2822363: closest beacon is at x=3444765, y=2347460
Sensor at x=2495377, y=3130491: closest beacon is at x=2761496, y=2831113
Sensor at x=1329263, y=1778185: closest beacon is at x=2729595, y=2000000
Sensor at x=2882039, y=2206085: closest beacon is at x=2729595, y=2000000
Sensor at x=3903141, y=2510440: closest beacon is at x=4006219, y=3011198
Sensor at x=3403454, y=3996578: closest beacon is at x=3754119, y=4475047
Sensor at x=3630476, y=1048796: closest beacon is at x=3444765, y=2347460
Sensor at x=16252, y=2089672: closest beacon is at x=-276514, y=2995794
Sensor at x=428672, y=1150723: closest beacon is at x=-281319, y=668868
Sensor at x=2939101, y=3624676: closest beacon is at x=2674241, y=4192888
Sensor at x=3166958, y=2890076: closest beacon is at x=2761496, y=2831113
Sensor at x=3758241, y=3546895: closest beacon is at x=4006219, y=3011198
Sensor at x=218942, y=3011070: closest beacon is at x=-276514, y=2995794
Sensor at x=52656, y=3484635: closest beacon is at x=-276514, y=2995794
Sensor at x=2057106, y=405314: closest beacon is at x=2154954, y=-342775
Sensor at x=1966905, y=2495701: closest beacon is at x=2761496, y=2831113
Sensor at x=511976, y=2696731: closest beacon is at x=-276514, y=2995794
Sensor at x=3094465, y=2478570: closest beacon is at x=3444765, y=2347460
Sensor at x=806671, y=228252: closest beacon is at x=-281319, y=668868
Sensor at x=3011731, y=1976307: closest beacon is at x=2729595, y=2000000";
