use std::{str::Split, vec};

use crate::handler::{AdventSolution, SolveError, DayHandler};

#[derive(Debug)]
pub enum Day3Error {}

impl Into<SolveError> for Day3Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day3Error: {:?}", self))
    }
}

const PRIORITIES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
fn get_score(letter: &str) -> usize{
    let (index, _chr) = PRIORITIES.char_indices().find(|(_i, chr)| &chr.to_string() == letter).unwrap();
    index + 1
}

fn get_line_split(line: &str) -> (&str, &str) {
    let mid = line.len() / 2 as usize;
    line.split_at(mid)
}

fn get_overlap(front: &str, back: &str) -> Vec<char> {
    let mut match_chars: Vec<char> = vec![];
    for f in front.chars() {
        for b in back.chars() {
            if f == b {
                match_chars.push(f);
            }
        }
    }

    if match_chars.len() == 0 {
        println!("Didn't find match in: front: {}, back: {}", front, back);
    }
    match_chars
}

pub struct Day3Handler {}
impl<'a> Day3Handler {
    pub fn new() -> DayHandler<'a, &'a str> { DayHandler::new(Day3Handler {}) }
    pub fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day3Error> {
        let final_total = input_lines.fold(0usize, |total, line| {
            if line.len() > 0 {
                let (front, back) = get_line_split(line);
                let overlap_chars = get_overlap(front, back);
                let overlap = overlap_chars.get(0).unwrap().to_string();
                let score = get_score(&overlap);
                total + score
            } else {
                total
            }
        });

        Ok(final_total.to_string())
    }
    
    pub fn solve_2(&self, input_lines: Split<&str>) -> Result<String, Day3Error> {
        let line_iter = input_lines.into_iter();
        let mut outcome: Vec<usize> = vec![];
        for [l1, l2, l3] in line_iter.array_chunks() {
            let l1_l2 = get_overlap(l1, l2);
            if l1_l2.len() == 1 {
                let score = get_score(&l1_l2.get(0).unwrap().to_string());
                outcome.push(score);
                continue;
            }
            let l2_l3 = get_overlap(l2, l3);
            if l2_l3.len() == 1 {
                let score = get_score(&l2_l3.get(0).unwrap().to_string());
                outcome.push(score);
                continue;
            }
            let l1_l2_match: String = l1_l2.iter().collect();
            let l2_l3_match: String = l2_l3.iter().collect();
            let l1_l3 = get_overlap(&l1_l2_match, &l2_l3_match);
            let score = get_score(&l1_l3.get(0).unwrap().to_string());
            outcome.push(score);
        }
        let sum = outcome.iter().fold(0usize, |total, curr| total + curr);
        Ok(sum.to_string())
    }
}

impl<'a> AdventSolution<&str> for Day3Handler {
    fn get_day(&self) -> String { String::from("3") }
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
    use super::Day3Handler;

    fn get_input<'a>() -> &'a str {
"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"
    }

    #[test]
    fn get_day() {
        let handler = Day3Handler::new();
        assert!(&handler.get_day() == "3");
    }

    async fn solution(sol: &str) -> String {
        let handler = Day3Handler::new();
        handler.solve(sol, get_input()).unwrap()
    }

    #[tokio::test]
    async fn solution_1() {
        let solution = solution("1").await;
        assert!(solution == String::from("157"));
    }

    #[tokio::test]
    async fn solution_2() {
        let solution = solution("2").await;
        assert!(solution == String::from("70"));
    }
}
