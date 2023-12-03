use std::{cell::RefCell, cmp, str::Split};

use crate::handler::{AdventSolution, DayHandler, SolveError};

#[derive(Debug)]
pub enum Day8Error {}

impl Into<SolveError> for Day8Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day8Error: {:?}", self))
    }
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

struct VisibleDir {
    top: Option<usize>,
    right: Option<usize>,
    down: Option<usize>,
    left: Option<usize>,
}

struct Tree {
    height: usize,
    position: Point,
    visible_dir: VisibleDir,
}

impl Tree {
    pub fn new(height_str: &str, col: usize, row: usize) -> Self {
        Tree {
            height: height_str.parse::<usize>().unwrap(),
            position: Point { x: col, y: row },
            visible_dir: VisibleDir {
                top: None,
                right: None,
                down: None,
                left: None,
            },
        }
    }

    pub fn is_visible(&mut self, forest: &Vec<Vec<RefCell<Tree>>>) -> bool {
        let row_len = forest.len();
        let col_len = forest.first().unwrap().len();

        let top = if self.position.y == 0 { 0 } else { forest[self.position.y - 1][self.position.x].borrow().visible_top_inclusive(forest) };
        let right = if self.position.x == col_len - 1 { 0 } else { forest[self.position.y][self.position.x + 1].borrow().visible_right_inclusive(forest) };
        let down = if self.position.y == row_len - 1 { 0 } else { forest[self.position.y + 1][self.position.x].borrow().visible_down_inclusive(forest) };
        let left = if self.position.x == 0 { 0 } else { forest[self.position.y][self.position.x - 1].borrow().visible_left_inclusive(forest) };

        let is_visible = self.is_edge(forest) || self.height > top || self.height > right || self.height > down || self.height > left;

        self.visible_dir.top = Some(cmp::max(self.height, top));
        self.visible_dir.right = Some(cmp::max(self.height, right));
        self.visible_dir.down = Some(cmp::max(self.height, down));
        self.visible_dir.left = Some(cmp::max(self.height, left));

        is_visible
    }

    pub fn get_scenic(&self, forest: &Vec<Vec<RefCell<Tree>>>) {

    }

    fn is_edge(&self, forest: &Vec<Vec<RefCell<Tree>>>) -> bool {
        let row_len = forest.len();
        let col_len = forest.first().unwrap().len();
        self.position.x == 0 || self.position.y == 0 || self.position.x == col_len - 1 || self.position.y == row_len - 1
    }

    pub fn visible_top_inclusive(&self, forest: &Vec<Vec<RefCell<Tree>>>) -> usize {
        if self.visible_dir.top.is_none() {
            if self.position.y == 0 {
                return self.height;
            } else {
                return cmp::max(self.height, forest[self.position.y - 1][self.position.x].borrow().visible_top_inclusive(forest));
            }
        }
        
        return self.visible_dir.top.unwrap();
    }

    pub fn visible_right_inclusive(&self, forest: &Vec<Vec<RefCell<Tree>>>) -> usize {
        if self.visible_dir.right.is_none() {
            if self.position.x == forest.first().unwrap().len() - 1 {
                return self.height;
            } else {
                return cmp::max(self.height, forest[self.position.y][self.position.x + 1].borrow().visible_right_inclusive(forest));
            }
        }

        self.visible_dir.right.unwrap()
    }

    pub fn visible_down_inclusive(&self, forest: &Vec<Vec<RefCell<Tree>>>) -> usize {
        if self.visible_dir.down.is_none() {
            if self.position.y == forest.len() - 1 {
                return self.height;
            } else {
                return cmp::max(self.height, forest[self.position.y + 1][self.position.x].borrow().visible_down_inclusive(forest));
            }
        }

        self.visible_dir.down.unwrap()
    }

    pub fn visible_left_inclusive(&self, forest: &Vec<Vec<RefCell<Tree>>>) -> usize {
        if self.visible_dir.left.is_none() {
            if self.position.x == 0 {
                return self.height;
            } else {
                return cmp::max(self.height, forest[self.position.y][self.position.x - 1].borrow().visible_left_inclusive(forest));
            }
        }

        self.visible_dir.left.unwrap()
    }
}

pub struct Day8Handler {}
impl<'a> Day8Handler {
    pub fn new() -> DayHandler<'a, &'a str> {
        DayHandler::new(Day8Handler {})
    }
    pub fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day8Error> {
        let forest = input_lines
            .enumerate()
            .fold(vec![], |mut inter_forest, (row, line)| {
                let forest_row = line.split("").filter(|val| val.len() > 0).enumerate().fold(
                    vec![],
                    |mut inter_forest_line, (col, chr)| {
                        inter_forest_line.push(RefCell::new(Tree::new(chr, col, row)));
                        inter_forest_line
                    },
                );
                inter_forest.push(forest_row);

                inter_forest
            });

        let mut count = 0;
        for row in &forest {
            for tree in row {
                let is_visible = tree.borrow_mut().is_visible(&forest);
                if is_visible {
                    count += 1;
                }
            }
        }

        Ok(count.to_string())
    }

    pub fn solve_2(&self, _input_lines: Split<&str>) -> Result<String, Day8Error> {
        todo!("Implement day 8 challenge 2");
    }
}

impl<'a> AdventSolution<&str> for Day8Handler {
    fn get_day(&self) -> String {
        String::from("8")
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
    use super::Day8Handler;
    use crate::handler::AdventSolution;

    fn get_input<'a>() -> &'a str {
"30373
25512
65332
33549
35390"
// 30373
// 25512
// 65332
// 33549
// 35390
    }

    #[test]
    fn get_day() {
        let handler = Day8Handler::new();
        assert!(&handler.get_day() == "8");
    }

    async fn solution(sol: &str) -> String {
        let handler = Day8Handler::new();
        handler.solve(sol, get_input()).unwrap()
    }

    #[tokio::test]
    async fn solution_1() {
        let solution = solution("1").await;
        assert!(solution == String::from("21"));
    }

    #[tokio::test]
    async fn solution_2() {
        let solution = solution("2").await;
        assert!(solution == String::from(""));
    }
}
