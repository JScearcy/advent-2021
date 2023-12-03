use std::collections::BinaryHeap;
use std::str::Split;
use crate::handler::{AdventSolution, SolveError, DayHandler};

#[derive(Debug)]
pub enum Day1Error {}

impl Into<SolveError> for Day1Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day1Error: {:?}", self))
    }
}

pub struct Day1Handler {}
impl<'a> Day1Handler {
    pub fn new() -> DayHandler<'a, &'a str> { DayHandler::new(Day1Handler {}) }
    pub fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day1Error> {
        let (max_cal, _) = input_lines.fold((0u64, 0u64), |(max_elf_cal, curr_elf_cal), line| {
            if line.len() == 0 {
                if curr_elf_cal > max_elf_cal {
                    return (curr_elf_cal, 0);
                } else {
                    return (max_elf_cal, 0);
                }
            } else {
                let val = line.parse::<u64>().unwrap();
                return (max_elf_cal, curr_elf_cal + val);
            }
        });

        Ok(max_cal.to_string())
    }
    
    pub fn solve_2(&self, input_lines: Split<&str>) -> Result<String, Day1Error> {
        let mut heap: BinaryHeap<u64> = BinaryHeap::new();
        let _max_cal = input_lines.fold(0u64, |curr_elf_cal, line| {
            if line.len() == 0 {
                heap.push(curr_elf_cal);
                return 0;
            } else {
                let val = line.parse::<u64>().unwrap();
                return curr_elf_cal + val;
            }
        });
        let first = heap.pop().unwrap();
        let second = heap.pop().unwrap();
        let third = heap.pop().unwrap();

        Ok((first + second + third).to_string())
    }
}

impl<'a> AdventSolution<&str> for Day1Handler {
    fn get_day(&self) -> String { String::from("1") }
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
    use super::Day1Handler;

    fn get_input<'a>() -> &'a str {
"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"
    }

    #[test]
    fn get_day() {
        let handler = Day1Handler::new();
        assert!(&handler.get_day() == "1");
    }

    async fn solution(sol: &str) -> String {
        let handler = Day1Handler::new();
        handler.solve(sol, get_input()).unwrap()
    }

    #[tokio::test]
    async fn solution_1() {
        let solution = solution("1").await;
        assert!(solution == String::from("24000"));
    }

    #[tokio::test]
    async fn solution_2() {
        let solution = solution("2").await;
        println!("solution: {}", solution);
        assert!(solution == String::from("45000"));
    }
}
