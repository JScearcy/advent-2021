use std::{str::Split, num::ParseIntError};

use crate::handler::{AdventSolution, SolveError, DayHandler};

#[derive(Debug)]
pub enum Day3Error {
    Parse(ParseIntError)
}

impl From<ParseIntError> for Day3Error {
    fn from(e: ParseIntError) -> Self {
        Day3Error::Parse(e)
    }
}

impl Into<SolveError> for Day3Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day3Error: {:?}", self))
    }
}

enum Place {
    All,
    Single(usize)
}

pub struct Day3Handler {}
impl<'a> Day3Handler {
    pub fn new() -> DayHandler<'a, &'a str> { DayHandler::new(Day3Handler {}) }
    fn get_counts<T: Iterator<Item=&'a str> + Clone>(&self, input_lines: T, place: Place) -> (u64, Vec<u64>) {
        let mut total_len = 0;
        let mut line_count: Vec<u64> = vec![];
        for line in input_lines {
            if line_count.len() == 0 { line_count = vec![0; line.len()]; };
            if let Place::Single(place_key) = place {
                let true_line: Vec<&str> = line.split("").filter(|l| l.len() > 0).collect();
                if true_line.len() == line_count.len() {
                    let place_val_res = true_line[place_key].parse::<u64>();
                    if let Ok(place_value) = place_val_res {
                        line_count[place_key] += place_value;
                    }
                }
            } else {
                for (place_key, place) in (0..).zip(line.split("").filter(|l| l.len() > 0)) {
                    let place_value_res = place.parse::<u64>();
                    if let Ok(place_value) = place_value_res {
                        line_count[place_key] += place_value;
                    };
                };
            }
            if line.len() > 0 { total_len += 1 };
        }
        (total_len, line_count)
    }

    pub fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day3Error> {
        let (total_len, line_count) = self.get_counts(input_lines, Place::All);

        let mut gamma: Vec<&str> = vec![""; line_count.len()];
        let mut eta: Vec<&str> = vec![""; line_count.len()];
        for (i, ones) in (0..).zip(line_count) {
            let zeroes = total_len - ones;
            gamma[i] = if ones > zeroes { "1" } else { "0" };
            eta[i] = if zeroes > ones { "1" } else { "0" };
        }

        let gamma_intval = isize::from_str_radix(&gamma.join(""), 2).unwrap();
        let eta_intval = isize::from_str_radix(&eta.join(""), 2).unwrap();
        Ok(format!("{}", gamma_intval * eta_intval))
    }


    
    pub fn solve_2(&self, input_lines: Split<&str>) -> Result<String, Day3Error> {
        let input_list: Vec<&str> = input_lines.clone().collect();
        let (init_total, init_line) = self.get_counts(input_list.clone().into_iter(), Place::Single(0));
        let ones = init_line[0];
        let zeroes = init_total - ones;
        let init_check = bool_to_int(ones > zeroes).to_string();
        let (mut o2_list, mut co2_list): (Vec<&str>, Vec<&str>) = input_lines.clone().partition(|val| val.len() > 0 && val.starts_with(&init_check));

        let mut o2_starts_with = init_check.clone();
        let mut co2_starts_with = bool_to_int(zeroes > ones).to_string();
        while o2_list.len() > 1 || co2_list.len() > 1 {
            if o2_list.len() > 1 {
                let (new_o2_starts_with, new_o2_list) = self.filter_step(o2_list.clone().into_iter(), &o2_starts_with, false);
                o2_starts_with = new_o2_starts_with;
                o2_list = new_o2_list;
            }

            if co2_list.len() > 1 {
                let (new_co2_starts_with, new_co2_list) = self.filter_step(co2_list.clone().into_iter(), &co2_starts_with, true);
                co2_starts_with = new_co2_starts_with;
                co2_list = new_co2_list;
            }
        }

        let o2_intval = isize::from_str_radix(&o2_list[0], 2).unwrap();
        let co2_intval = isize::from_str_radix(&co2_list[0], 2).unwrap();

        Ok(format!("{}", o2_intval * co2_intval))
    }

    fn filter_step<T: Iterator<Item=&'a str> + Clone>(&self, list: T, starts_with: &str, invert: bool) -> (String, Vec<T::Item>) {
        let mut owned_starts_with = String::from(starts_with);
        let (total_len, line_count) = self.get_counts(list.clone().into_iter(), Place::Single(owned_starts_with.len()));
        let ones = line_count[owned_starts_with.len()];
        let zeroes = total_len - ones;
        let comp = if invert { ("1", "0") } else { ("0", "1") };
        if zeroes > ones { owned_starts_with.push_str(comp.0) } else { owned_starts_with.push_str(comp.1) };

        let new_list: Vec<T::Item> = list.into_iter().filter(|val| val.len() > 0 && val.starts_with(&owned_starts_with)).collect();

        (owned_starts_with, new_list)
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

fn bool_to_int(b: bool) -> u8 {
    if b { 1 } else { 0 }
}