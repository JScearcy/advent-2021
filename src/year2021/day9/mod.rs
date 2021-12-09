use std::str::Split;
use std::collections::{BinaryHeap, VecDeque, HashSet};

use crate::handler::{AdventSolution, SolveError, DayHandler};

#[derive(Debug)]
pub enum Day9Error {}

impl Into<SolveError> for Day9Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day9Error: {:?}", self))
    }
}

pub struct Day9Handler {}
impl<'a> Day9Handler {
    pub fn new() -> DayHandler<'a, &'a str> { DayHandler::new(Day9Handler {}) }
    fn get_map(&self, input_lines: Split<&str>) -> (Vec<Vec<usize>>, (usize, usize)) {
        let mut map: Vec<Vec<usize>> = vec![]; 
        for line in input_lines {
            let row: Vec<usize> = line.split("")
                .filter(|val| val.len() > 0)
                .map(|val| val.parse::<usize>().unwrap())
                .collect();
            map.push(row);
        }
        let x = map[0].len();
        let y = map.len();
        (map, (x, y))
    }

    fn low_points(&self, input_lines: Split<&str>) -> (Vec<(usize, (usize, usize))>, Vec<Vec<usize>>, (usize, usize)) {
        let (map, size) = self.get_map(input_lines);

        let mut low_points: Vec<(usize, (usize, usize))> = vec![];
        for (row_i, row) in (0usize..).zip(map.iter()) {
            for (col_i, col) in (0usize..).zip(row.iter()) {
                let neighbors = self.get_neighbors(&(col_i, row_i), &map, &size);
                let is_smallest_neighbor = neighbors.iter().fold(Some(col), |acc, neighbor_opt| {
                    acc.and_then(|curr_col| {
                        if !neighbor_opt.is_some() || &neighbor_opt.unwrap().0 > curr_col { Some(curr_col) }
                        else { None }
                    })
                });

                if is_smallest_neighbor.is_some() {
                    low_points.push((*col, (col_i, row_i)));
                }

            }
        }

        (low_points, map, size)
    }

    fn get_neighbors(&self, point: &(usize, usize), map: &Vec<Vec<usize>>, size: &(usize, usize)) -> [Option<(usize, (usize, usize))>; 4] {
        let (col, row) = *point;
        let (col_len, row_len) = *size;
        let north = if row > 0 { 
            Some((map[row - 1][col], (col, row - 1)))
        } else { None };

        let south = if row + 1 < row_len {
            Some((map[row + 1][col], (col, row + 1)))
        } else { None };

        let east = if col + 1 < col_len {
            Some((map[row][col + 1], (col + 1, row)))
        } else { None };

        let west = if col > 0 {
            Some((map[row][col - 1], (col - 1, row)))
        } else { None };

        [north, south, east, west]
    }

    fn get_basin(&self, center: &(usize, usize), map: &Vec<Vec<usize>>, size: &(usize, usize)) -> Vec<usize> {
        let mut basin: Vec<usize> = vec![map[center.1][center.0]];
        let mut check_neighbors = VecDeque::from(vec![*center]);
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut counted: HashSet<(usize, usize)> = HashSet::new();
        counted.insert(*center);
        while check_neighbors.len() > 0 {
            let point = check_neighbors.pop_back().unwrap();
            let point_height = map[point.1][point.0];
            let neighbors = self.get_neighbors(&point, map, size);
            visited.insert(point);
            let mut items: Vec<usize> = neighbors.iter()
                .filter(|neighbor_opt| {
                    if let Some(neigbor) = neighbor_opt {
                        !visited.contains(&neigbor.1)
                    } else {
                        false
                    }
                })
                .fold(vec![], |mut acc, neighbor| {
                    let (neighbor_point_height, neighbor_point) = neighbor.unwrap();
                    if !counted.contains(&neighbor_point) && neighbor_point_height > point_height && neighbor_point_height != 9 {
                        counted.insert(neighbor_point);
                        check_neighbors.push_back(neighbor_point);
                        acc.push(neighbor_point_height);
                    }
                    acc
                });

            basin.append(&mut items);
        }

        basin
    }

    pub fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day9Error> {
        let (low_points, _, _) = self.low_points(input_lines);
        let risk_level = low_points.iter().fold(0, |acc, (val, _)| acc + val + 1);
        Ok(format!("{}", risk_level))
    }
    
    pub fn solve_2(&self, input_lines: Split<&str>) -> Result<String, Day9Error> {
        let (low_points, map, size) = self.low_points(input_lines.clone());
        let top_basins = low_points.iter().fold(BinaryHeap::new(), |mut acc, point| {
            acc.push(self.get_basin(&point.1, &map, &size).len());
            acc
        });
        let basin_count = top_basins.iter().take(3).fold(1, |acc, curr| acc * curr);
        Ok(format!("{}", basin_count))
    }
}

impl<'a> AdventSolution<&str> for Day9Handler {
    fn get_day(&self) -> String { String::from("9") }
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
    use super::Day9Handler;

    fn get_input<'a>() -> &'a str {
"2199943210
3987894921
9856789892
8767896789
9899965678"
    }

    #[test]
    fn get_day() {
        let handler = Day9Handler::new();
        assert!(&handler.get_day() == "9");
    }

    async fn solution(sol: &str) -> String {
        let handler = Day9Handler::new();
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
        assert!(solution == String::from("1134"));
    }
}
