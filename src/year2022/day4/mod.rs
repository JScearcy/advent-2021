use std::str::Split;

use crate::handler::{AdventSolution, SolveError, DayHandler};

#[derive(Debug)]
pub enum Day4Error {}

impl Into<SolveError> for Day4Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day4Error: {:?}", self))
    }
}

struct Pair {
    start: usize,
    end: usize,
}

impl Pair {
    pub fn new(pair_str: &str) -> Self {
        let pair_split: Vec<&str> = pair_str.split("-").collect();
        let start = pair_split[0].parse::<usize>().unwrap();
        let end = pair_split[1].parse::<usize>().unwrap();

        Pair { start, end }
    }

    pub fn contains(&self, other_pair: &Pair) -> bool {
        self.start <= other_pair.start && self.end >= other_pair.end
    }

    pub fn overlaps(&self, other_pair: &Pair) -> bool {
        other_pair.start >= self.start && other_pair.start <= self.end ||
        other_pair.end >= self.start && other_pair.end <= self.end
    }
}

pub struct Day4Handler {}
impl<'a> Day4Handler {
    pub fn new() -> DayHandler<'a, &'a str> { DayHandler::new(Day4Handler {}) }
    pub fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day4Error> {
        let result = input_lines.fold(0usize, |count, pair| {
            if pair.len() <= 0 {
                return count;
            }
            let pairs: Vec<&str> = pair.split(",").collect();
            let first_pair = Pair::new(pairs[0]);
            let second_pair = Pair::new(pairs[1]);
            if first_pair.contains(&second_pair) || second_pair.contains(&first_pair) {
                return count + 1;
            }

            count
        });

        Ok(result.to_string())
    }
    
    pub fn solve_2(&self, input_lines: Split<&str>) -> Result<String, Day4Error> {
        let result = input_lines.fold(0usize, |count, pair| {
            if pair.len() <= 0 {
                return count;
            }
            let pairs: Vec<&str> = pair.split(",").collect();
            let first_pair = Pair::new(pairs[0]);
            let second_pair = Pair::new(pairs[1]);
            if first_pair.overlaps(&second_pair) || second_pair.overlaps(&first_pair) {
                return count + 1;
            }

            count
        });

        Ok(result.to_string())
    }
}

impl<'a> AdventSolution<&str> for Day4Handler {
    fn get_day(&self) -> String { String::from("4") }
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
    use super::Day4Handler;

    fn get_input<'a>() -> &'a str {
"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"
    }

    #[test]
    fn get_day() {
        let handler = Day4Handler::new();
        assert!(&handler.get_day() == "4");
    }

    async fn solution(sol: &str) -> String {
        let handler = Day4Handler::new();
        handler.solve(sol, get_input()).unwrap()
    }

    #[tokio::test]
    async fn solution_1() {
        let solution = solution("1").await;
        assert!(solution == String::from("2"));
    }

    #[tokio::test]
    async fn solution_2() {
        let solution = solution("2").await;
        assert!(solution == String::from(""));
    }
}
