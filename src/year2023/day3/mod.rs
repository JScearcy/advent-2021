use std::str::Split;
use std::collections::VecDeque;

use crate::handler::{AdventSolution, DayHandler, SolveError};

#[derive(Debug)]
pub enum Day3Error {}

impl Into<SolveError> for Day3Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day3Error: {:?}", self))
    }
}

static INT_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn get_adjacent_idx(
    (x, y): (usize, usize),
    (width, height): (usize, usize),
) -> Vec<(usize, usize)> {
    let mut xs = vec![];
    let mut ys = vec![];
    if x > 0 {
        xs.push(x - 1);
    }
    xs.push(x);
    if x + 1 < width {
        xs.push(x + 1);
    }

    if y > 0 {
        ys.push(y - 1);
    }
    ys.push(y);
    if y + 1 < height {
        ys.push(y + 1);
    }

    let mut combined = vec![];

    for curr_x in xs {
        for curr_y in ys.iter() {
            if x != curr_x || y != *curr_y {
                combined.push((curr_x, *curr_y))
            }
        }
    }

    combined
}

fn get_number((x, y): (usize, usize), width: usize, grid: &Vec<Vec<char>>) -> usize {
    let row = &grid[y];
    let first_digit = row[x];
    let mut digit_chars: VecDeque<char> = VecDeque::new();
    digit_chars.push_front(first_digit);
    if x > 0 {
        let mut left = x;
        while left > 0usize {
            left -= 1;
            let curr_val = row[left];
            if !INT_CHARS.contains(&curr_val) {
                break;
            }
            digit_chars.push_front(curr_val);
        }
    }

    let mut right = x + 1;
    while right < width {
        let curr_val = row[right];
        if !INT_CHARS.contains(&curr_val) {
            break;
        }
        digit_chars.push_back(curr_val);
        right += 1;
    }

    let number_str: String = digit_chars.into_iter().collect();

    usize::from_str_radix(&number_str, 10).unwrap()
    
}

fn get_numbers((x, y): (usize, usize), row: &Vec<char>) -> Vec<(usize, usize)> {
    let mut numbers: Vec<(usize, usize)> = vec![];
    if x > 0 && INT_CHARS.contains(&row[x - 1]) {
        numbers.push((x - 1, y));
    }

    let mid_number = INT_CHARS.contains(&row[x]);
    
    if mid_number && numbers.len() == 0{
        numbers.push((x, y));
    }

    if x + 1 < row.len() && !mid_number && INT_CHARS.contains(&row[x + 1]) {
        numbers.push((x + 1, y));
    }
    
    numbers
}

pub struct Day3Handler {}
impl<'a> Day3Handler {
    pub fn new() -> DayHandler<'a, &'a str> {
        DayHandler::new(Day3Handler {})
    }
    pub fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day3Error> {
        let grid: Vec<Vec<char>> = input_lines.map(|line| line.chars().collect()).collect();
        let grid_height = grid.len();
        let grid_width = grid[0].len();
        let mut is_part = false;
        let mut part_numbers: Vec<usize> = vec![];
        let mut curr_part: Vec<char> = vec![];
        for (y, row) in grid.iter().enumerate() {
            for (x, chr) in row.iter().enumerate() {
                if !INT_CHARS.contains(chr) {
                    if is_part {
                        let number_string: String = curr_part.clone().into_iter().collect();
                        let part_number = usize::from_str_radix(&number_string, 10).unwrap();
                        part_numbers.push(part_number);
                    }
                    is_part = false;
                    curr_part = vec![];
                } else {
                    curr_part.push(*chr);
                    let adjacent = get_adjacent_idx((x, y), (grid_width, grid_height));
                    let has_symbol = adjacent.iter().any(|(adj_x, adj_y)| {
                        let adj_chr = grid[*adj_y][*adj_x];
                        !INT_CHARS.contains(&adj_chr) && adj_chr != '.'
                    });
                    if has_symbol {
                        is_part = true;
                    }
                }
            }
        }

        let total = part_numbers.iter().fold(0usize, |total, val| total + val);
        Ok(total.to_string())
    }

    pub fn solve_2(&self, input_lines: Split<&str>) -> Result<String, Day3Error> {
        let grid: Vec<Vec<char>> = input_lines.map(|line| line.chars().collect()).collect();
        let grid_height = grid.len();
        let grid_width = grid[0].len();
        let mut total = 0usize;
        for (y, row) in grid.iter().enumerate() {
            for (x, chr) in row.iter().enumerate() {
                if chr == &'*' {
                    let mut numbers: Vec<(usize, usize)> = vec![];
                    if y > 0 {
                        let mut top_numbers = get_numbers((x, y - 1), &grid[y - 1]);
                        numbers.append(&mut top_numbers);
                    }
                    let mut row_numbers = get_numbers((x, y), row);
                    numbers.append(&mut row_numbers);
                    if y + 1 < grid_height {
                        let mut bottom_numbers = get_numbers((x, y + 1), &grid[y + 1]);
                        numbers.append(&mut bottom_numbers);
                    }

                    if numbers.len() == 2 {
                        total += numbers.into_iter().map(|(x, y)| get_number((x, y), grid_width, &grid)).fold(1usize, |total, curr| total * curr);
                    }
                }
            }
        }

        Ok(total.to_string())
    }
}

impl<'a> AdventSolution<&str> for Day3Handler {
    fn get_day(&self) -> String {
        String::from("3")
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
    use super::Day3Handler;
    use crate::handler::AdventSolution;

    fn get_input<'a>() -> &'a str {
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
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
        assert!(solution == String::from("4361"));
    }

    #[tokio::test]
    async fn solution_2() {
        let solution = solution("2").await;
        assert!(solution == String::from("467835"));
    }
}
