use std::str::Split;

use crate::handler::{AdventSolution, SolveError, DayHandler};

#[derive(Debug)]
pub enum Day9Error {}

impl Into<SolveError> for Day9Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day9Error: {:?}", self))
    }
}

pub struct Day9Handler {}
impl<'a> Day9Handler {
    pub fn new() -> DayHandler<'a, &'a str> { DayHandler::new(Day9Handler {}) }
    pub fn solve_1(&self, _input_lines: Split<&str>) -> Result<String, Day9Error> {
        todo!("Implement day 9 challenge 1");
    }
    
    pub fn solve_2(&self, _input_lines: Split<&str>) -> Result<String, Day9Error> {
        todo!("Implement day 9 challenge 2");
    }
}

impl<'a> AdventSolution<&str> for Day9Handler {
    fn get_day(&self) -> String { String::from("9") }
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
    use crate::handler::AdventSolution;
    use super::Day9Handler;

    fn get_input<'a>() -> &'a str {
        todo!("Add test input to run tests")
    }

    #[test]
    fn get_day() {
        let handler = Day9Handler::new();
        assert!(&handler.get_day() == "9");
    }

    async fn solution(sol: &str) -> String {
        let handler = Day9Handler::new();
        handler.solve(sol, get_input()).unwrap()
    }

    #[tokio::test]
    async fn solution_1() {
        let solution = solution("1").await;
        assert!(solution == String::from(""));
    }

    #[tokio::test]
    async fn solution_2() {
        let solution = solution("2").await;
        assert!(solution == String::from(""));
    }
}
