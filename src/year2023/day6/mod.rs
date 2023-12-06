use std::str::Split;

use crate::handler::{AdventSolution, SolveError, DayHandler};

#[derive(Debug)]
pub enum Day6Error {}

impl Into<SolveError> for Day6Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day6Error: {:?}", self))
    }
}

fn ints_from_line(line: &str) -> Vec<usize> {
    let numbers_str: String = line.split(":").skip(1).take(1).collect();
    let numbers: Vec<usize> = numbers_str.split(" ").filter(|val| val.len() > 0).map(|val| usize::from_str_radix(val, 10).unwrap()).collect();
    numbers
}


fn int_from_line(line: &str) -> usize {
    let number_str: String = line.split(":").skip(1).take(1).collect();
    let number: String = number_str.split(" ").filter(|val| val.len() > 0).collect();
    usize::from_str_radix(&number, 10).unwrap()
}

// d = hold_time * time_left 
// 
fn get_distance(total_time: usize, hold_time: usize) -> usize {
    hold_time * (total_time - hold_time)
}

pub struct Day6Handler {}
impl<'a> Day6Handler {
    pub fn new() -> DayHandler<'a, &'a str> { DayHandler::new(Day6Handler {}) }
    pub fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day6Error> {
        let time_string: String = input_lines.clone().take(1).collect();
        let distance_string: String = input_lines.skip(1).take(1).collect();
        let time_list = ints_from_line(&time_string);
        let distance_list = ints_from_line(&distance_string);
        let mut totals = vec![];
        for (time, distance) in time_list.into_iter().zip(distance_list) {
            let mut time_distance: usize = 0;
            for pressed in 0..time {
                let distance_by_hold = get_distance(time, pressed);
                if distance_by_hold > distance {
                    time_distance += 1;
                }
            }
            totals.push(time_distance);
        }
        let total = totals.iter().fold(1, |val, curr| val * curr);
        Ok(total.to_string())
    }
    
    pub fn solve_2(&self, input_lines: Split<&str>) -> Result<String, Day6Error> {
        let time_string: String = input_lines.clone().take(1).collect();
        let distance_string: String = input_lines.skip(1).take(1).collect();
        let time = int_from_line(&time_string);
        let distance = int_from_line(&distance_string);
        let mut time_distance: usize = 0;
        for pressed in 0..time {
            let distance_by_hold = get_distance(time, pressed);
            if distance_by_hold > distance {
                time_distance += 1;
            }
        }
        Ok(time_distance.to_string())
    }
}

impl<'a> AdventSolution<&str> for Day6Handler {
    fn get_day(&self) -> String { String::from("6") }
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
    use super::Day6Handler;

    fn get_input<'a>() -> &'a str {
"Time:      7  15   30
Distance:  9  40  200"
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
        assert!(solution == String::from("288"));
    }

    #[tokio::test]
    async fn solution_2() {
        let solution = solution("2").await;
        assert!(solution == String::from("71503"));
    }
}
