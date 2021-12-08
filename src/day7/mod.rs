use std::str::Split;

use crate::handler::{AdventSolution, SolveError, DayHandler};

#[derive(Debug)]
pub enum Day7Error {}

impl Into<SolveError> for Day7Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day7Error: {:?}", self))
    }
}

pub struct Day7Handler {}
impl<'a> Day7Handler {
    pub fn new() ->  DayHandler<'a, &'a str> { DayHandler::new(Day7Handler {}) }

    fn solve_shared(&self, input_lines: Split<&str>, calc_cost: &dyn Fn(isize) -> isize) -> isize {
        let input: Vec<isize> = input_lines.map(|num| num.parse::<isize>().unwrap()).collect();

        let mut min_cost = isize::MAX;
        let max = input.iter().fold(0, |max, val| *val.max(&max));
        for i in 0..=max {
            let mut curr_costs: Vec<isize> = vec![];
            for j in input.iter() {
                curr_costs.push(calc_cost((i-j).abs()))
            }
            let total_cost = curr_costs.iter().fold(0, |acc, val| acc + val);
            
            if total_cost < min_cost { min_cost = total_cost; }
        }
        
        min_cost
    }

    pub fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day7Error> {
        let min_cost = self.solve_shared(input_lines, &|dist| dist);
        Ok(format!("{}", min_cost))
    }
    
    pub fn solve_2(&self, input_lines: Split<&str>) -> Result<String, Day7Error> {
        let min_cost = self.solve_shared(input_lines.clone(), &|dist| dist * (dist + 1) / 2);
        Ok(format!("{}", min_cost))
    }
}

impl<'a> AdventSolution<&str> for Day7Handler {
    fn get_day(&self) -> String { String::from("7") }
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
    use crate::{load_input::load, handler::AdventSolution};
    use super::Day7Handler;

    #[test]
    fn get_day() {
        let handler = Day7Handler::new();
        assert!(&handler.get_day() == "7");
    }

    async fn solution(day: &str, sol: &str) -> String {
        let input = load(day, "", false, None).await.unwrap();
        let handler = Day7Handler::new();
        let solution = handler.solve(sol, &input).unwrap();

        solution
    }

    #[tokio::test]
    async fn solution_1() {
        let solution = solution("7", "1").await;
        assert!(solution == String::from("326132"));
    }

    #[tokio::test]
    async fn solution_2() {
        let solution = solution("7", "2").await;
        assert!(solution == String::from("88612508"));
    }
}
