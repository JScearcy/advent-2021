use std::{str::Split, collections::HashMap};

use crate::handler::{AdventSolution, SolveError, DayHandler};

#[derive(Debug)]
pub enum Day10Error {}

impl Into<SolveError> for Day10Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day10Error: {:?}", self))
    }
}

pub struct Day10Handler {}
impl<'a> Day10Handler {
    pub fn new() -> DayHandler<'a, &'a str> { DayHandler::new(Day10Handler {}) }

    pub fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day10Error> {
        let pairs: HashMap<char, char> = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
        let err_score: HashMap<char, usize> = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
        let mut corrupted: Vec<char> = vec![];
        for line in input_lines {
            let mut stack: Vec<char> = vec![];
            for chr in line.chars() {
                if pairs.contains_key(&chr) {
                    stack.push(chr);
                } else {
                    let latest_open = stack.pop().unwrap();
                    let expected_close = pairs.get(&latest_open).unwrap();
                    if &chr != expected_close {
                        corrupted.push(chr);
                        continue;
                    }
                }
            };
        }

        let score = corrupted.iter().fold(0, |total, chr| {
            let val = err_score.get(chr).unwrap();
            val + total
        });
        Ok(format!("{}", score))
    }
    
    pub fn solve_2(&self, input_lines: Split<&str>) -> Result<String, Day10Error> {
        let auto_score: HashMap<char, usize> = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
        let pairs: HashMap<char, char> = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
        let mut scores = vec![];
        let _test = input_lines.clone().fold(vec![], |mut scrs, line| {
            let score = line.chars().fold(Some(vec![]), |stack_opt, chr| {
                if let Some(mut stack) = stack_opt {
                    if pairs.contains_key(&chr) {
                        stack.push(chr);
                        return Some(stack);
                    } else {
                        let latest_open = stack.pop().unwrap();
                        let expected_close = pairs.get(&latest_open).unwrap();
                        if &chr != expected_close {
                            return None;
                        } else {
                            return Some(stack);
                        }
                    }
                } else {
                    return None;
                }
            });
            scrs.push(score);

            scrs
        });
        'linefor: for line in input_lines {
            let mut stack: Vec<char> = vec![];
            for chr in line.chars() {
                if pairs.contains_key(&chr) {
                    stack.push(chr);
                } else {
                    let latest_open = stack.pop().unwrap();
                    let expected_close = pairs.get(&latest_open).unwrap();
                    if &chr != expected_close {
                        continue 'linefor;
                    }
                }
            };
            let complete = stack.iter().rev().fold(vec![], |mut tag, open| {
                tag.push(*pairs.get(open).unwrap());
                tag
            });
            let score = complete.iter().fold(0, |total, chr| 5 * total + auto_score.get(chr).unwrap());
            scores.push(score);
        }
        scores.sort();
        let middle = (scores.len() as f64 / 2.).floor() as usize;

        Ok(format!("{}", scores.iter().skip(middle).take(1).last().unwrap()))
    }
}

impl<'a> AdventSolution<&str> for Day10Handler {
    fn get_day(&self) -> String { String::from("10") }
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
    use super::Day10Handler;

    fn get_input<'a>() -> &'a str {
"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
    }

    #[test]
    fn get_day() {
        let handler = Day10Handler::new();
        assert!(&handler.get_day() == "10");
    }

    async fn solution(sol: &str) -> String {
        let handler = Day10Handler::new();
        handler.solve(sol, get_input()).unwrap()
    }

    #[tokio::test]
    async fn solution_1() {
        let solution = solution("1").await;
        assert!(solution == String::from("26397"));
    }

    #[tokio::test]
    async fn solution_2() {
        let solution = solution("2").await;
        assert!(solution == String::from("288957"));
    }
}
