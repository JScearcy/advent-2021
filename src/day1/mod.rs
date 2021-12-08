use std::{num::ParseIntError, str::Split};
use crate::handler::{SolveError, AdventSolution, DayHandler};


#[derive(Debug)]
pub enum Day1Error { 
    Parse(ParseIntError)
}

impl From<ParseIntError> for Day1Error {
    fn from(error: ParseIntError) -> Self {
        Day1Error::Parse(error)
    }
}

impl Into<SolveError> for Day1Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day1Error: {:?}", self))
    }
}

struct TripletIter<'a> {
    curr: Option<(&'a str, &'a str, &'a str)>,
    items: Split<'a, &'a str>,
}
impl<'a> TripletIter<'a> {
    fn from(items: Split<'a, &'a str> ) -> TripletIter<'a> {
        TripletIter {
            items,
            curr: Option::None
        }
    }
}
impl<'a> Iterator for TripletIter<'a> {
    type Item = (&'a str, &'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let next_val = self.items.next();
        self.curr = self.curr
            .and_then(|(_, b, c)| {
                if let Some(value) = next_val {
                    Some((b, c, value))
                } else {
                    None
                }
            })
            .or_else(|| {
                let b = self.items.next();
                let c = self.items.next();
                if next_val.is_some() && b.is_some() && c.is_some() {
                    Some((&next_val.unwrap(), &b.unwrap(), &c.unwrap()))
                } else {
                    None
                }
            });

        self.curr
    }
}

#[derive(Debug)]
pub struct Day1Handler {}
impl<'a> Day1Handler {
    pub fn new() -> DayHandler<'a, &'a str> { DayHandler::new(Day1Handler {}) }
    fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day1Error> {
        let increase = self.check_increase(input_lines, |val| val.parse::<u16>().map_err(|e| e.into()));
        Ok(format!("{}", increase))
    }
    
    fn solve_2(&self, input_lines: Split<'a, &'a str>) -> Result<String, Day1Error> {
        let window = TripletIter::from(input_lines);
        let increase = self.check_increase(window, |val| {
            let (a, b, c) = val;
            let a_val = a.parse::<u16>()?;
            let b_val = b.parse::<u16>()?;
            let c_val = c.parse::<u16>()?;
            Ok(a_val + b_val + c_val)
        });
        Ok(format!("{}", increase))
    }

    fn check_increase<T: Iterator, F: Fn(T::Item) -> Result<u16, Day1Error>>(&self, iter: T, get_val: F) -> u16 {
        let mut increase = 0;
        let mut prev: Option<u16> = Option::None;
        for val in iter {
            let value_res = get_val(val);
            if let Ok(value) = value_res {
                if let Some(prev_val) = prev {
                    increase = if value > prev_val { increase + 1 } else { increase }
                }
                prev = Some(value);
            }
        }
    
        increase
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
