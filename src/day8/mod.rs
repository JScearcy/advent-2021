use std::str::Split;
use std::collections::HashMap;

use crate::handler::{AdventSolution, SolveError, DayHandler};

#[derive(Debug)]
pub enum Day8Error {}

impl Into<SolveError> for Day8Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day8Error: {:?}", self))
    }
}

pub struct Day8Handler {}
impl<'a> Day8Handler {
    pub fn new() -> DayHandler<'a, &'a str> { DayHandler::new(Day8Handler {}) }
    pub fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day8Error> {
        let mut count = 0;
        for line in input_lines {
            let back_half = line.split(" | ").last().unwrap();
            for segment in back_half.split(" ") {
                match segment.len() {
                    2 => { count += 1; },
                    3 => { count += 1; },
                    4 => { count += 1; },
                    7 => { count += 1; },
                    _ => {}
                }
            }
        }

        Ok(format!("{}", count))
    }

    pub fn solve_2(&self, input_lines: Split<&str>) -> Result<String, Day8Error> {
        let mut final_val = 0;
        let digit_hash = HashMap::from([
            ("467889", 0),
            ("89", 1),
            ("47788", 2),
            ("77889", 3),
            ("6789", 4),
            ("67789", 5),
            ("467789", 6),
            ("889", 7),
            ("4677889", 8),
            ("677889", 9),
        ]);

        for line in input_lines {
            let front_half_digits_raw = line.clone().split(" | ").take(1).last().unwrap().split(" ").collect::<Vec<&str>>().join("");
            let front_half_digits: Vec<&str> = front_half_digits_raw.split("").filter(|val| val.len() > 0).collect();
            let counts = front_half_digits.iter().fold(HashMap::new(), |chars, chr| {
                let chr_count_opt = chars.get(chr);
                let mut new_chars = chars.clone();
                if let Some(char_count) = chr_count_opt {
                    new_chars.insert(*chr, char_count + 1);
                } else {
                    new_chars.insert(*chr, 1);
                }

                new_chars
            });

            let back_half = line.clone().split(" | ").last().unwrap();
            let digits = back_half.split(" ").fold(vec![], |mut digits, digit| {
                let mut segment_places: Vec<i32> = digit.split("")
                    .filter(|chr| chr.len() > 0)    
                    .map(|chr| *counts.get(chr).clone().unwrap())
                    .collect();
                segment_places.sort();
                let segment_hash = segment_places.iter().map(|val| val.to_string()).collect::<Vec<String>>().join("");
                let next_digit = digit_hash.get(segment_hash.as_str()).unwrap();
                digits.push(next_digit);
                digits
            });

            let value = digits.iter().fold(0, |acc, elem| acc * 10 + *elem);
            final_val += value;
        }

        Ok(format!("{}", final_val))
    }
}

impl<'a> AdventSolution<&str> for Day8Handler {
    fn get_day(&self) -> String { String::from("8") }
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
    use crate::{load_input::load, handler::AdventSolution};
    use super::Day8Handler;

    #[test]
    fn get_day() {
        let handler = Day8Handler::new();
        assert!(&handler.get_day() == "8");
    }

    async fn solution(day: &str, sol: &str) -> String {
        let input = load(day, "", false, None).await.unwrap();
        let handler = Day8Handler::new();
        let solution = handler.solve(sol, &input).unwrap();

        solution
    }

    #[tokio::test]
    async fn solution_1() {
        let solution = solution("8", "1").await;
        assert!(solution == String::from("321"));
    }

    #[tokio::test]
    async fn solution_2() {
        let solution = solution("8", "2").await;
        assert!(solution == String::from("1028926"));
    }
}
