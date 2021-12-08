use std::{str::Split, collections::VecDeque, ops::Add};

use crate::handler::{AdventSolution, DayHandler, SolveError};

#[derive(Debug)]
pub enum Day6Error {}

#[derive(Clone, Debug)]
struct LanternfishGroup(usize);

impl Add for LanternfishGroup {
    type Output=LanternfishGroup;

    fn add(self, rhs: Self) -> Self::Output {
        let LanternfishGroup(lhs_fish) = self;
        let LanternfishGroup(rhs_fish) = rhs;

        LanternfishGroup(lhs_fish + rhs_fish)
    }
}

#[derive(Debug)]
struct Runtime {
    days: VecDeque<LanternfishGroup>,
}

impl Runtime {
    pub fn new(initial_fish: Vec<u64>) -> Runtime {
        let mut lanternfish = VecDeque::from(vec![LanternfishGroup(0); 9]);
        for fish_day in initial_fish {
            let day = lanternfish[fish_day as usize].clone();
            let new_fish = LanternfishGroup(1) + LanternfishGroup(0);
            lanternfish[fish_day as usize] = day + new_fish;
        }
        Runtime {
            days: lanternfish
        }
    }
}

impl Iterator for Runtime {
    type Item=usize;

    fn next(&mut self) -> Option<Self::Item> {
        let reproduce = self.days.pop_front().unwrap();
        let day_6 = self.days[6].clone();
        self.days.push_back(reproduce.clone());
        self.days[6] = day_6 + reproduce;
        
        let total: usize = self.days.iter().fold(0, |acc, grp| {
            let LanternfishGroup(add) = grp;

            acc + add
        });

        Some(total)
    }
}

impl Into<SolveError> for Day6Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day6Error: {:?}", self))
    }
}

pub struct Day6Handler {}
impl<'a> Day6Handler {
    pub fn new() -> DayHandler<'a, &'a str> { DayHandler::new(Day6Handler {}) }
    pub fn solve_challenge(&self, input_lines: Split<&str>, days: usize) -> Result<String, Day6Error> {
        let initial_fish: Vec<u64> = input_lines.map(|days| days.parse::<u64>().unwrap()).collect();
        let mut runtime = Runtime::new(initial_fish);

        let mut latest_fish = 0;
        for (_day, fish) in (1..=days).zip(&mut runtime) {
            latest_fish = fish;
        }

        Ok(format!("{}", latest_fish))
    }

    pub fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day6Error> {
        self.solve_challenge(input_lines, 80)
    }
    
    pub fn solve_2(&self, input_lines: Split<&str>) -> Result<String, Day6Error> {
        self.solve_challenge(input_lines, 256)
    }
}

impl<'a> AdventSolution<&str> for Day6Handler {
    fn get_day(&self) -> String { String::from("6") }
    fn solve(&self, problem: &str, input: &str) -> Result<String, SolveError> {
        let input_lines = input.split(",");
        let result = if problem == "1" {
            self.solve_1(input_lines)
        } else {
            self.solve_2(input_lines)
        };

        result.map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::handler::AdventSolution;
    use super::Day6Handler;

    fn get_input<'a>() -> &'a str {
        "3,4,3,1,2"
    }

    #[tokio::test]
    async fn solution_1() {
        let handler = Day6Handler::new();
        let solution = handler.solve("1", get_input()).unwrap();
        assert!(solution == String::from("5934"));
    }

    #[tokio::test]
    async fn solution_2() {
        let handler = Day6Handler::new();
        let solution = handler.solve("2", get_input()).unwrap();
        assert!(solution == String::from("26984457539"));
    }
}
