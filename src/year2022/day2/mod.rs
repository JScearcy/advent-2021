use std::str::Split;

use crate::handler::{AdventSolution, SolveError, DayHandler};

#[derive(Debug)]
pub enum Day2Error {}

impl Into<SolveError> for Day2Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day2Error: {:?}", self))
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

#[derive(PartialEq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

fn letter_to_outcome(letter: &str) -> Outcome {
    match letter {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ =>  unimplemented!("Unsupported outcome provided: {}", letter)
    }
}

fn letter_to_shape(letter: &str) -> Shape {
    match letter {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        "C" | "Z" => Shape::Scissors,
        _ =>  unimplemented!("Unsupported shape provided: {}", letter)
    }
}

fn get_shape_score(shape: &Shape) -> u64 {
    match shape {
        &Shape::Rock => 1,
        &Shape::Paper => 2,
        &Shape::Scissors => 3,
    }
}

fn get_win_shape(opp_shape: &Shape) -> Shape {
    match opp_shape {
        &Shape::Rock => Shape::Paper,
        &Shape::Paper => Shape::Scissors,
        &Shape::Scissors => Shape::Rock,
    }
}

fn get_outcome_shape(opp_shape: &Shape, outcome: &Outcome) -> Shape {
    match outcome {
        &Outcome::Draw => opp_shape.clone(),
        &Outcome::Win => get_win_shape(opp_shape),
        &Outcome::Lose => get_win_shape(&get_win_shape(opp_shape)),
    }
}

fn get_result_score(opp_shape_letter: &str, shape_letter: &str) -> u64 {
    let shape = letter_to_shape(shape_letter);
    let opp_shape = letter_to_shape(opp_shape_letter);
    let shape_score = get_shape_score(&shape);
    let win_shape = get_win_shape(&opp_shape);
    if opp_shape == shape {
        return 3 + shape_score;
    } else if shape == win_shape {
        return 6 + shape_score;
    } else {
        return shape_score;
    }
}

fn get_outcome_score(opp_shape_letter: &str, outcome_letter: &str) -> u64 {
    let outcome = letter_to_outcome(outcome_letter);
    let opp_shape = letter_to_shape(opp_shape_letter);
    let outcome_shape = get_outcome_shape(&opp_shape, &outcome);
    let shape_score = get_shape_score(&outcome_shape);
    if outcome == Outcome::Draw {
        return 3 + shape_score;
    } else if outcome == Outcome::Win {
        return 6 + shape_score;
    } else {
        return shape_score;
    }

}

pub struct Day2Handler {}
impl<'a> Day2Handler {
    pub fn new() -> DayHandler<'a, &'a str> { DayHandler::new(Day2Handler {}) }
    pub fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day2Error> {
        let score = input_lines.fold(0u64, |running, line| {
            if line.len() > 0 {
                let shapes: Vec<&str> = line.split(" ").collect();
                return running + get_result_score(shapes[0], shapes[1]);
            }
            running
        });

        Ok(score.to_string())
    }
    
    pub fn solve_2(&self, input_lines: Split<&str>) -> Result<String, Day2Error> {
        let score = input_lines.fold(0u64, |running, line| {
            if line.len() > 0 {
                let shapes: Vec<&str> = line.split(" ").collect();
                return running + get_outcome_score(shapes[0], shapes[1]);
            }
            running
        });

        Ok(score.to_string())
    }
}

impl<'a> AdventSolution<&str> for Day2Handler {
    fn get_day(&self) -> String { String::from("2") }
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
    use super::Day2Handler;

    fn get_input<'a>() -> &'a str {
"A Y
B X
C Z
"
    }

    #[test]
    fn get_day() {
        let handler = Day2Handler::new();
        assert!(&handler.get_day() == "2");
    }

    async fn solution(sol: &str) -> String {
        let handler = Day2Handler::new();
        handler.solve(sol, get_input()).unwrap()
    }

    #[tokio::test]
    async fn solution_1() {
        let solution = solution("1").await;
        assert!(solution == String::from("15"));
    }

    #[tokio::test]
    async fn solution_2() {
        let solution = solution("2").await;
        assert!(solution == String::from("12"));
    }
}
