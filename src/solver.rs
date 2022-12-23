use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use crate::{
    utils::{print_time_results, DayPerfMetric},
    year_2020, year_2021, year_2022,
};

pub trait Solver {
    fn get_input(&self) -> &'static str;
    fn with_input(&mut self, input: &str);
    fn solve_part1(&self) -> String;
    fn solve_part2(&self) -> String;
}

pub struct Solvers;

impl Solvers {
    pub fn new() -> Self {
        Self
    }

    #[allow(clippy::box_default)]
    fn get_solver(year: usize, day: usize) -> Option<Box<dyn Solver>> {
        match (year, day) {
            (2020, 1) => Some(Box::new(year_2020::day_01::Solution::default())),
            (2020, 2) => Some(Box::new(year_2020::day_02::Solution::default())),
            (2020, 3) => Some(Box::new(year_2020::day_03::Solution::default())),
            (2020, 4) => Some(Box::new(year_2020::day_04::Solution::default())),

            (2021, 1) => Some(Box::new(year_2021::day_01::Solution::default())),
            (2021, 2) => Some(Box::new(year_2021::day_02::Solution::default())),
            (2021, 25) => Some(Box::new(year_2021::day_25::Solution::default())),

            (2022, 1) => Some(Box::new(year_2022::day_01::Solution::default())),
            (2022, 2) => Some(Box::new(year_2022::day_02::Solution::default())),
            (2022, 3) => Some(Box::new(year_2022::day_03::Solution::default())),
            (2022, 4) => Some(Box::new(year_2022::day_04::Solution::default())),
            (2022, 5) => Some(Box::new(year_2022::day_05::Solution::default())),
            (2022, 6) => Some(Box::new(year_2022::day_06::Solution::default())),
            (2022, 7) => Some(Box::new(year_2022::day_07::Solution::default())),
            (2022, 8) => Some(Box::new(year_2022::day_08::Solution::default())),
            (2022, 9) => Some(Box::new(year_2022::day_09::Solution::default())),
            (2022, 10) => Some(Box::new(year_2022::day_10::Solution::default())),
            (2022, 11) => Some(Box::new(year_2022::day_11::Solution::default())),
            (2022, 12) => Some(Box::new(year_2022::day_12::Solution::default())),
            (2022, 13) => Some(Box::new(year_2022::day_13::Solution::default())),
            (2022, 14) => Some(Box::new(year_2022::day_14::Solution::default())),
            (2022, 15) => Some(Box::new(year_2022::day_15::Solution::default())),
            (2022, 16) => Some(Box::new(year_2022::day_16::Solution::default())),
            (2022, 17) => Some(Box::new(year_2022::day_17::Solution::default())),
            (2022, 18) => Some(Box::new(year_2022::day_18::Solution::default())),
            // (2022, 19) => Some(Box::new(year_2022::day_19::Solution::default())),
            (2022, 20) => Some(Box::new(year_2022::day_20::Solution::default())),
            (2022, 21) => Some(Box::new(year_2022::day_21::Solution::default())),
            _ => None,
        }
    }

    pub fn run_all(year: usize, runs: usize) {
        let (res_tx, res_rx) = mpsc::channel::<DayPerfMetric>();

        let handles: Vec<JoinHandle<_>> = (1..=25)
            .map(|day| {
                let res_tx = res_tx.clone();
                thread::spawn(move || {
                    if let Some(result) = Solvers::run(year, day, runs) {
                        res_tx
                            .send(DayPerfMetric {
                                day: day,
                                part1: result.0,
                                part2: result.1,
                            })
                            .unwrap();
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap()
        }
        drop(res_tx);
        println!("finished");
        let mut results: Vec<DayPerfMetric> = res_rx.iter().collect();
        results.sort_by_key(|r| r.day);

        print_time_results(results, runs);
    }

    pub fn run(year: usize, day: usize, runs: usize) -> Option<(Duration, Duration)> {
        if Solvers::get_solver(year, day).is_none() {
            return None;
        }

        let part1_start = Instant::now();
        let mut solution1 = "".to_string();
        for _ in 0..runs {
            if let Some(mut solver) = Solvers::get_solver(year, day) {
                solver.with_input(solver.get_input());
                solution1 = solver.solve_part1();
            }
        }
        let part1_dur = part1_start.elapsed() / runs as u32;

        let part2_start = Instant::now();
        let mut solution2 = "".to_string();
        for _ in 0..runs {
            if let Some(mut solver) = Solvers::get_solver(year, day) {
                solver.with_input(solver.get_input());
                solution2 = solver.solve_part2();
            }
        }
        let part2_dur = part2_start.elapsed() / runs as u32;

        println!("\n--- YEAR {year} - DAY {day:0>2} ---\npart 1: {solution1}\npart 2: {solution2}");

        Some((part1_dur, part2_dur))
    }
}
