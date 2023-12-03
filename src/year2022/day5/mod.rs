use std::collections::VecDeque;
use std::str::Split;
use pest::Parser;

use crate::handler::{AdventSolution, SolveError, DayHandler};

#[derive(Debug)]
pub enum Day5Error {}

impl Into<SolveError> for Day5Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day5Error: {:?}", self))
    }
}

#[derive(Parser)]
#[grammar = "year2022/day5/command.pest"]
struct CommandParser;

struct Command {
    amount: usize,
    from_stack: usize,
    to_stack: usize,
}

impl Command {
    pub fn new(command_line: &str) -> Self {
        let mut command_parse = CommandParser::parse(Rule::command, command_line).unwrap().next().unwrap().into_inner();
        let amount = command_parse.next().unwrap().as_str().trim().parse::<usize>().unwrap();
        let from = command_parse.next().unwrap().as_str().trim().parse::<usize>().unwrap();
        let to = command_parse.next().unwrap().as_str().trim().parse::<usize>().unwrap();

        Command {
            amount,
            from_stack: from - 1,
            to_stack: to - 1,
        }
    }

    pub fn process(&self, stacks: &mut Vec<VecDeque<String>>, all_at_once: bool) {
        let from_stack = stacks.get_mut(self.from_stack).unwrap();
        let split_point = from_stack.len() - (self.amount);
        let drained_init = from_stack.drain(split_point..);
        let mut drained: VecDeque<String> = if all_at_once { drained_init.collect() } else { drained_init.rev().collect() };

        let to_stack = stacks.get_mut(self.to_stack).unwrap();
        to_stack.append(&mut drained);
    }
}

fn parse_stacks(input_lines: &mut Split<&str>) -> Vec<VecDeque<String>> {
    let mut stacks: Vec<VecDeque<String>> = vec![];

    for line in input_lines {
        if line.len() == 0 {
            break;
        }

        let chunks = line.chars().array_chunks();

        for (idx, [_lw, n, _rw, _sp]) in chunks.enumerate() {
            if stacks.get(idx).is_none() {
                stacks.insert(idx, VecDeque::new());
            }
            if n == ' ' { continue; }
            if n == '1' { break; }

            let stack = stacks.get_mut(idx).unwrap();
            stack.push_front(n.to_string());
        }
    }
    
    stacks
}

pub struct Day5Handler {}
impl<'a> Day5Handler {
    pub fn new() -> DayHandler<'a, &'a str> { DayHandler::new(Day5Handler {}) }
    pub fn solve_1(&self, mut input_lines: Split<&str>) -> Result<String, Day5Error> {
        let mut stacks = parse_stacks(&mut input_lines);

        for line in input_lines {
            if line.len() == 0 { continue; }

            let command = Command::new(line);
            command.process(&mut stacks, false);
        }

        let mut tops = vec![];
        for stack in stacks {
            tops.push(stack.back().unwrap().clone());
        }

        Ok(tops.join(""))
    }
    
    pub fn solve_2(&self, mut input_lines: Split<&str>) -> Result<String, Day5Error> {
        let mut stacks = parse_stacks(&mut input_lines);

        for line in input_lines {
            if line.len() == 0 { continue; }

            let command = Command::new(line);
            command.process(&mut stacks, true);
        }

        let mut tops = vec![];
        for stack in stacks {
            tops.push(stack.back().unwrap().clone());
        }

        Ok(tops.join(""))
    }
}

impl<'a> AdventSolution<&str> for Day5Handler {
    fn get_day(&self) -> String { String::from("5") }
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
    use super::Day5Handler;

    fn get_input<'a>() -> &'a str {
"    [D]     
[N] [C]     
[Z] [M] [P] 
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"
    }

    #[test]
    fn get_day() {
        let handler = Day5Handler::new();
        assert!(&handler.get_day() == "5");
    }

    async fn solution(sol: &str) -> String {
        let handler = Day5Handler::new();
        handler.solve(sol, get_input()).unwrap()
    }

    #[tokio::test]
    async fn solution_1() {
        let solution = solution("1").await;
        assert!(solution == String::from("CMZ"));
    }

    #[tokio::test]
    async fn solution_2() {
        let solution = solution("2").await;
        assert!(solution == String::from("MCD"));
    }
}
