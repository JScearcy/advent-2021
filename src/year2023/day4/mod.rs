use std::str::Split;

use crate::handler::{AdventSolution, DayHandler, SolveError};

#[derive(Debug)]
pub enum Day4Error {}

impl Into<SolveError> for Day4Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day4Error: {:?}", self))
    }
}

#[derive(Debug)]
struct Winning {
    numbers: Vec<usize>
}

impl Winning {
    fn from_card(card_str: &str) -> Self {
        let number_str: String = card_str.split(":").skip(1).take(1).collect();
        let winning_str: String = number_str.split("|").take(1).collect();
        let numbers: Vec<usize> = winning_str.split(" ").filter(|str| str.len() > 0).map(|num_str| usize::from_str_radix(num_str, 10).unwrap()).collect();

        Self { numbers }
    }
}

#[derive(Debug)]
struct Current {
    numbers: Vec<usize>
}

impl Current {
    fn from_card(card_str: &str) -> Self {
        let current_str: String = card_str.split("|").skip(1).take(1).collect();
        let numbers: Vec<usize> = current_str.split(" ").filter(|str| str.len() > 0).map(|num_str| usize::from_str_radix(num_str, 10).unwrap()).collect();

        Current { numbers }
    }
}

pub struct Day4Handler {}
impl<'a> Day4Handler {
    pub fn new() -> DayHandler<'a, &'a str> {
        DayHandler::new(Day4Handler {})
    }
    pub fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day4Error> {
        let mut total = 0;
        for card_str in input_lines {
            let mut card_matches = 0;
            let winning = Winning::from_card(card_str);
            let current = Current::from_card(card_str);
            for winning_num in winning.numbers {
                if current.numbers.contains(&winning_num) {
                    card_matches += 1;
                }
            }
            if card_matches > 0 {
                let safe_shift = if card_matches - 1 >= 0  { card_matches - 1 } else { 0 };
                total += 1 << safe_shift;
            }
        }

        Ok(total.to_string())
    }

    pub fn solve_2(&self, input_lines: Split<&str>) -> Result<String, Day4Error> {
        let lines_vec: Vec<&str> = input_lines.clone().collect();
        let mut carry: Vec<usize> = vec![1usize;  lines_vec.len()];
        for (card_idx, card_str) in input_lines.enumerate() {
            let mut card_matches = card_idx;
            let winning = Winning::from_card(card_str);
            let current = Current::from_card(card_str);
            for winning_num in winning.numbers {
                if current.numbers.contains(&winning_num) {
                    card_matches += 1;
                    carry[card_matches] += carry[card_idx];
                }
            }
        }
        let total = carry.iter().fold(0, |total, val| total + val);
        Ok(total.to_string())
    }
}

impl<'a> AdventSolution<&str> for Day4Handler {
    fn get_day(&self) -> String {
        String::from("4")
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
    use super::Day4Handler;
    use crate::handler::AdventSolution;

    fn get_input<'a>() -> &'a str {
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
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
        assert!(solution == String::from("13"));
    }

    #[tokio::test]
    async fn solution_2() {
        let solution = solution("2").await;
        assert!(solution == String::from("30"));
    }
}
