use std::{str::Split, ops::RangeInclusive, iter::Rev};

use ndarray::Array2;

use crate::handler::{AdventSolution, SolveError, DayHandler};

#[derive(Debug)]
pub enum Day5Error {}

impl Into<SolveError> for Day5Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day5Error: {:?}", self))
    }
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Pair(u16, u16);
impl Pair {
    pub fn from_str(pair: &str) -> Pair {
        let pair_vec: Vec<u16> = pair.split(",").map(|x| x.parse::<u16>().unwrap()).collect();
        Pair(pair_vec[0], pair_vec[1])
    }
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
enum Slope {
    Vertical,
    Horizontal,
    Linear1,
    Invalid,
}

impl Slope {
    fn is_valid(&self, allow_linear: bool) -> bool {
        match self {
            Slope::Invalid => false,
            Slope::Linear1 => allow_linear,
            _ => true
        }
    }

    fn from_num(slope: f32) -> Slope {
        if slope == 1. { Slope::Linear1 }
        else if slope == 0. { Slope::Horizontal }
        else if f32::is_infinite(slope) { Slope::Vertical }
        else { Slope::Invalid }
    }
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Line(Pair, Pair, Slope);
impl Line {
    pub fn from_vec(pairs: Vec<Pair>) -> Option<Line> {
        let Pair(x1, y1) = pairs[0];
        let Pair(x2, y2) = pairs[1];
        let slope = ((y2 as f32 - y1 as f32)/(x2 as f32 - x1 as f32)).abs();
        if pairs.len() == 2 {
            Some(Line(pairs[0].clone(), pairs[1].clone(), Slope::from_num(slope)))
        } else {
            None
        }
    }
}

pub struct Day5Handler {}
impl<'a> Day5Handler {
    pub fn new() -> DayHandler<'a, &'a str> { DayHandler::new(Day5Handler {}) }

    fn get_lines(input_lines: Split<&str>, allow_linear: bool) -> (u16, u16, Vec<Line>) {
        input_lines
            .fold((0, 0, vec![]), |acc: (u16, u16, Vec<Line>) , line_str| {
                let line = Day5Handler::get_line(line_str.split(" -> "));
                let (max_x, max_y, mut curr_lines) = acc;
                if line.2.clone().is_valid(allow_linear) {
                    let Line(Pair(x1, y1), Pair(x2, y2), _) = line;
                    let mut xs = [&max_x, &x1, &x2];
                    let mut ys = [&max_y, &y1, &y2];
                    let new_max_x = Day5Handler::get_max(&mut xs);
                    let new_max_y = Day5Handler::get_max(&mut ys);
                    curr_lines.push(line);
                    
                    (new_max_x, new_max_y, curr_lines)
                } else {
                    (max_x, max_y, curr_lines)
                }
            })
    }

    fn get_line(line: Split<&str>) -> Line {
        let pairs: Vec<Pair> = line.map(|pair_str| Pair::from_str(pair_str)).collect();
        Line::from_vec(pairs).unwrap()
    }

    fn get_max(nums: &mut [&u16; 3]) -> u16 {
        nums.sort();
        **nums.last().unwrap()
    }

    fn solve_map(input_lines: Split<&str>, allow_linear: bool) -> Result<String, Day5Error> {
        let size_lines: (u16, u16, Vec<Line>) = Day5Handler::get_lines(input_lines, allow_linear);
        let (max_x, max_y, lines) = size_lines;
        let mut map = Array2::<u8>::default((max_y as usize + 1, max_x as usize + 1));
        for line in lines {
            let Line(Pair(x1, y1), Pair(x2, y2), _) = line;
            for (x, y) in AxisIter::from_points(x1, x2).zip(AxisIter::from_points(y1, y2)) {
                map[[y as usize, x as usize]] += 1;
            }
        }

        let overlaps = map.fold(0, |total, el| if el > &1 { total + 1 } else { total });
        Ok(format!("{}", overlaps))
    }

    pub fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day5Error> {
        Day5Handler::solve_map(input_lines, false)
    }
    
    pub fn solve_2(&self, input_lines: Split<&str>) -> Result<String, Day5Error> {
        Day5Handler::solve_map(input_lines, true)
    }
}

enum AxisIter {
    Rev(Rev<RangeInclusive<u16>>),
    Rng(RangeInclusive<u16>),
    One(u16)
}

impl AxisIter {
    fn from_points(p1: u16, p2: u16) -> AxisIter {
        if p1 == p2 { AxisIter::One(p1) }
        else if p1 > p2 { AxisIter::Rev((p2..=p1).rev()) }
        else { AxisIter::Rng(p1..=p2) }
    }
}

impl Iterator for AxisIter {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            AxisIter::Rev(rev) => rev.next(),
            AxisIter::Rng(rng) => rng.next(),
            AxisIter::One(val) => Some(*val),
        }
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
"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
    }

    #[tokio::test]
    async fn solution_1() {
        let handler = Day5Handler::new();
        let solution = handler.solve("1", get_input()).unwrap();
        println!("soln1: {}", solution);
        assert!(solution == String::from("5"));
    }

    #[tokio::test]
    async fn solution_2() {
        let handler = Day5Handler::new();
        let solution = handler.solve("2", get_input()).unwrap();
        println!("soln2: {}", solution);
        assert!(solution == String::from("12"));
    }
}
