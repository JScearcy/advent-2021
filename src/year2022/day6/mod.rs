use std::{
    collections::{hash_map::RandomState, HashSet},
    iter::FromIterator,
    str::Split,
};

use crate::handler::{AdventSolution, DayHandler, SolveError};

#[derive(Debug)]
pub enum Day6Error {}

impl Into<SolveError> for Day6Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day6Error: {:?}", self))
    }
}

fn message_unique(message: &[char]) -> bool {
    let message_set: HashSet<&char, RandomState> = HashSet::from_iter(message.iter());

    message.len() == message_set.len()
}

pub struct Day6Handler {}
impl<'a> Day6Handler {
    pub fn new() -> DayHandler<'a, &'a str> {
        DayHandler::new(Day6Handler {})
    }
    pub fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day6Error> {
        let mut first_idx: Option<usize> = None;
        for line in input_lines {
            let line_vec: Vec<char> = line.chars().collect();
            for (idx, window) in line_vec.windows(4).enumerate() {
                if message_unique(window) {
                    first_idx = Some(idx + window.len());
                    break;
                }
            }
        }

        Ok(first_idx.unwrap().to_string())
    }

    pub fn solve_2(&self, input_lines: Split<&str>) -> Result<String, Day6Error> {
        let mut first_idx: Option<usize> = None;
        for line in input_lines {
            let line_vec: Vec<char> = line.chars().collect();
            for (idx, window) in line_vec.windows(14).enumerate() {
                if message_unique(window) {
                    first_idx = Some(idx + window.len());
                    break;
                }
            }
        }

        Ok(first_idx.unwrap().to_string())
    }
}

impl<'a> AdventSolution<&str> for Day6Handler {
    fn get_day(&self) -> String {
        String::from("6")
    }
    fn solve(&self, problem: &str, input: &str) -> Result<String, SolveError> {
        let input_lines = input.split("\n");
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
    use super::Day6Handler;
    use crate::handler::AdventSolution;

    fn get_input<'a>() -> &'a str {
        "bvwbjplbgvbhsrlpgdmjqwftvncz"
    }

    #[test]
    fn get_day() {
        let handler = Day6Handler::new();
        assert!(&handler.get_day() == "6");
    }

    async fn solution(sol: &str) -> String {
        let handler = Day6Handler::new();
        handler.solve(sol, get_input()).unwrap()
    }

    #[tokio::test]
    async fn solution_1() {
        let solution = solution("1").await;
        assert!(solution == String::from("5"));
    }

    #[tokio::test]
    async fn solution_2() {
        let solution = solution("2").await;
        assert!(solution == String::from("23"));
    }
}
