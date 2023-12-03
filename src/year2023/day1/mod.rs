use std::{collections::HashMap, str::Split};

use crate::handler::{AdventSolution, DayHandler, SolveError};

#[derive(Debug)]
pub enum Day1Error {}

impl Into<SolveError> for Day1Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day1Error: {:?}", self))
    }
}

fn get_first_last(line: &str) -> [char; 2] {
    let ints: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;
    for chr in line.chars() {
        let is_int = ints.contains(&chr);
        if is_int && first.is_none() {
            first.replace(chr);
        }

        if is_int {
            last.replace(chr);
        }
    }

    [first.unwrap(), last.unwrap()]
}

fn get_matches(line: &str) -> [Option<(usize, &str)>; 2] {
    let numbers: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut first: Option<(usize, &str)> = None;
    let mut last: Option<(usize, &str)> = None;
    for number in numbers {
        let first_idx = line.find(number);
        if first_idx.is_some() && first.is_none() {
            first = Some((first_idx.unwrap(), number));
        } else if first_idx.is_some() && first.is_some() {
            let curr: (usize, &str) = first.clone().unwrap();
            let first_idx_un = first_idx.unwrap();
            if curr.0 > first_idx_un {
                first = Some((first_idx_un, number))
            }
        }
        let last_idx = line.rfind(number);

        if last_idx.is_some() && last.is_none() {
            last = Some((last_idx.unwrap(), number));
        } else if last_idx.is_some() && last.is_some() {
            let curr = last.clone().unwrap();
            let last_idx_un = last_idx.unwrap();
            if curr.0 < last_idx_un {
                last = Some((last_idx_un, number));
            }
        }
    }

    [first, last]
}

fn get_first_last_idx<F>(line: &str, matcher: F) -> [Option<(usize, char)>; 2]
where
    F: Fn(&char) -> bool,
{
    let mut first: Option<(usize, char)> = None;
    let mut last: Option<(usize, char)> = None;
    for (idx, chr) in line.chars().enumerate() {
        let matches = matcher(&chr);
        if matches && first.is_none() {
            first.replace((idx, chr));
        }

        if matches {
            last.replace((idx, chr));
        }
    }

    [first, last]
}

fn get_first_last_int(line: &str) -> usize {
    let [first, last] = get_first_last(line);
    let str_val = format!("{}{}", first, last);
    let val = usize::from_str_radix(&str_val, 10).unwrap();

    val
}

fn str_to_char(number_str: &str) -> char {
    let map = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
        ("zero", '0'),
    ]);

    map.get(number_str).unwrap().clone()
}

pub struct Day1Handler {}
impl<'a> Day1Handler {
    pub fn new() -> DayHandler<'a, &'a str> {
        DayHandler::new(Day1Handler {})
    }
    pub fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day1Error> {
        let solution = input_lines
            .map(get_first_last_int)
            .fold(0usize, |total, val| total + val);

        Ok(solution.to_string())
    }

    pub fn solve_2(&self, input_lines: Split<&str>) -> Result<String, Day1Error> {
        let ints: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
        let mut total = 0usize;
        for line in input_lines {
            let mut first: Option<char> = None;
            let mut last: Option<char> = None;
            let [first_char, last_char] = get_first_last_idx(line, |chr| ints.contains(chr));
            let [first_str, last_str] = get_matches(line);
            if first_str.is_some() && first_char.is_some() {
                let (str_idx, str) = first_str.unwrap();
                let (char_idx, chr) = first_char.unwrap();

                if str_idx < char_idx {
                    first = Some(str_to_char(str));
                } else {
                    first = Some(chr);
                }
            } else if first_str.is_some() {
                first = Some(str_to_char(first_str.unwrap().1));
            } else if first_char.is_some() {
                first = Some(first_char.unwrap().1);
            }

            if last_str.is_some() && last_char.is_some() {
                let (str_idx, str) = last_str.unwrap();
                let (char_idx, chr) = last_char.unwrap();

                if str_idx > char_idx {
                    last = Some(str_to_char(str));
                } else {
                    last = Some(chr);
                }
            } else if last_str.is_some() {
                last = Some(str_to_char(last_str.unwrap().1));
            } else if last_char.is_some() {
                last = Some(last_char.unwrap().1);
            }

            let str_val = format!("{}{}", first.unwrap(), last.unwrap());

            let val = usize::from_str_radix(&str_val, 10).unwrap();
            total += val;
        }

        Ok(total.to_string())
    }
}

impl<'a> AdventSolution<&str> for Day1Handler {
    fn get_day(&self) -> String {
        String::from("1")
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
    use super::Day1Handler;
    use crate::handler::AdventSolution;

    fn get_input_1<'a>() -> &'a str {
        "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
    }

    fn get_input_2<'a>() -> &'a str {
        "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
    }

    #[test]
    fn get_day() {
        let handler = Day1Handler::new();
        assert!(&handler.get_day() == "1");
    }

    #[tokio::test]
    async fn solution_1() {
        let handler = Day1Handler::new();
        let solution = handler.solve("1", get_input_1()).unwrap();
        assert!(solution == String::from("142"));
    }

    #[tokio::test]
    async fn solution_2() {
        let handler = Day1Handler::new();
        let solution = handler.solve("2", get_input_2()).unwrap();
        assert!(solution == String::from("281"));
    }
}
