use std::{str::Split, num::ParseIntError};

use crate::handler::{AdventSolution, SolveError, DayHandler};

#[derive(Debug)]
pub enum Day2Error { 
    Parse(ParseIntError)
}

impl From<ParseIntError> for Day2Error {
    fn from(e: ParseIntError) -> Self {
        Day2Error::Parse(e)
    }
}

impl Into<SolveError> for Day2Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day2Error: {:?}", self))
    }
}

#[derive(Debug)]
pub struct Day2Handler {}
impl<'a> Day2Handler {
    pub fn new() -> DayHandler<'a, &'a str> { DayHandler::new(Day2Handler {}) }
    pub fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day2Error> {
        let mut horizontal = 0;
        let mut depth = 0;
        for line in input_lines {
            if line.len() > 0 {
                let line_cmd: Vec<&str> = line.split(" ").collect();
                let cmd = line_cmd[0];
                let len = line_cmd[1].parse::<u32>()?;
        
                match cmd {
                    "forward" => horizontal += len,
                    "down" => depth += len,
                    "up" => depth -= len,
                    _ => panic!("Unsupported cmd: {}", cmd),
                }
            }
        }
    
        Ok(format!("{}", horizontal * depth))
    }
    
    pub fn solve_2(&self, input_lines: Split<&str>) -> Result<String, Day2Error> {
        let mut aim = 0;
        let mut horizontal = 0;
        let mut depth = 0;
        for line in input_lines {
            if line.len() > 0 {
                let line_cmd: Vec<&str> = line.split(" ").collect();
                let cmd = line_cmd[0];
                let len = line_cmd[1].parse::<i32>()?;
        
                match cmd {
                    "forward" => { 
                        horizontal += len;
                        depth += len * aim;
                    },
                    "down" => aim += len,
                    "up" => aim -= len,
                    _ => panic!("Unsupported cmd: {}", cmd),
                }
            }
        }
    
        Ok(format!("{}", horizontal * depth))
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
