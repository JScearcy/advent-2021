use std::{str::Split, collections::{VecDeque, HashSet}};

use crate::handler::{AdventSolution, SolveError, DayHandler};

#[derive(Debug)]
pub enum Day13Error {}

impl Into<SolveError> for Day13Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day13Error: {:?}", self))
    }
}

#[derive(Clone,Debug)]
enum Fold {
    Up(usize),
    Left(usize),
}

#[derive(Clone,Debug)]
struct InstructionPage {
    folds: VecDeque<Fold>,
    height: usize,
    marks: HashSet<(usize, usize)>,
    width: usize,
}

impl InstructionPage {
    fn new(lines: Split<&str>) -> InstructionPage {
        let (marks, folds, width, height) = lines.fold((HashSet::new(), VecDeque::new(), 0usize, 0usize), |acc, line| {
            let (mut marks, mut folds, mut max_x, mut max_y) = acc;
            if line.len() > 0 {
                if line.contains(",") {
                    let dot_v: Vec<usize> = line.split(",").filter(|chr| chr.len() > 0).map(|val| val.parse::<usize>().unwrap()).collect();
                    let x = dot_v[0];
                    let y = dot_v[1];
                    marks.insert((x, y));
                    if x > max_x { max_x = x }
                    if y > max_y { max_y = y }
                } else {
                    let fold: Vec<&str> = line.split("=").collect();
                    let fold_val = fold[1].parse::<usize>().unwrap();
                    if fold[0].contains("x") {
                        folds.push_back(Fold::Left(fold_val));
                    } else {
                        folds.push_back(Fold::Up(fold_val));
                    }
                }
            }

            (marks, folds, max_x, max_y)
        });
        InstructionPage {
            folds,
            height,
            marks,
            width,
        }
    }
}

impl Iterator for InstructionPage {
    type Item=(usize, HashSet<(usize, usize)>);

    // new_h = h - (h - f) = 7
    // fold (f = 7) -> 14 - f = 7 = offset
    // height = 7 -> h - offset = 0 (new_y)
    // new_h = h - (h - f) = 13
    // fold (f = 13) -> 14 - f = 1
    // height = 14 -> h - offset = 12 (new_y)

    // new_w = w - (w - f) = 5
    // fold (f = 5) -> 6 - f = 1 = offset
    // width = 5 -> w - offset = 4
    fn next(&mut self) -> Option<Self::Item> {
        let fold = self.folds.pop_front();
        let height = self.height;
        let width = self.width;
        // f_val = fold_val, s_val = size (height/width), pt = x or y
        let get_new_axis = |f_val: usize, s_val: usize, pt: usize| {
            if pt > f_val {
                let offset = pt - f_val;
                let new_pt = s_val - offset;
                new_pt
            } else {
                pt
            }
        };
        if let Some(Fold::Up(fold_amt)) = fold {
            let new_height = height - (height - fold_amt);
            let new_marks: HashSet<(usize, usize)> = self.marks.iter().map(|(x, y)| {
                let new_y = get_new_axis(fold_amt, new_height, *y);
                (*x, new_y)
            }).collect();
            let visible = new_marks.len();
            self.height = new_height;
            self.marks = new_marks.clone();
            Some((visible, new_marks))
        } else if let Some(Fold::Left(fold_amt)) = fold {
            let new_width = width - (width - fold_amt);
            let new_marks: HashSet<(usize, usize)> = self.marks.iter().map(|(x, y)| {
                let new_x = get_new_axis(fold_amt, new_width, *x);
                (new_x, *y)
            }).collect();
            let visible = new_marks.len();
            self.width = new_width;
            self.marks = new_marks.clone();
            Some((visible, new_marks))
        } else {
            None
        }
    }
}

pub struct Day13Handler {}
impl<'a> Day13Handler {
    pub fn new() -> DayHandler<'a, &'a str> { DayHandler::new(Day13Handler {}) }
    pub fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day13Error> {
        let instructions_page = InstructionPage::new(input_lines);
        let mut instructions_iter = instructions_page.into_iter();
        let (val, _) = instructions_iter.next().unwrap();

        Ok(format!("{}", val))
    }
    
    pub fn solve_2(&self, input_lines: Split<&str>) -> Result<String, Day13Error> {
        let instructions_page = InstructionPage::new(input_lines);
        let (_, final_marks) = instructions_page
            .into_iter()
            .fold(None, |_, marks| Some(marks))
            .unwrap();
        let (width, height) = final_marks.clone().iter().fold((0usize, 0usize), |(x,  y), (x_c, y_c)| {
            let new_x = if x_c > &x { *x_c } else { x };
            let new_y = if y_c > &y { *y_c } else { y };

            (new_x, new_y)
        });
        let page = vec![vec!["."; width + 1]; height + 1];
        let filled_page = final_marks.iter().fold(page, |mut acc, (x, y)| {
            acc[*y][*x] = "#";
            acc
        });
        let formatted = filled_page.iter().fold(String::from(""), |acc, line| {
            format!("{}\n{}", acc, line.join(""))
        });

        Ok(formatted)
    }
}

impl<'a> AdventSolution<&str> for Day13Handler {
    fn get_day(&self) -> String { String::from("13") }
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
    use super::Day13Handler;

    fn get_input<'a>() -> &'a str {
"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"
    }

    #[test]
    fn get_day() {
        let handler = Day13Handler::new();
        assert!(&handler.get_day() == "13");
    }

    async fn solution(sol: &str) -> String {
        let handler = Day13Handler::new();
        handler.solve(sol, get_input()).unwrap()
    }

    #[tokio::test]
    async fn solution_1() {
        let solution = solution("1").await;
        assert!(solution == String::from("17"));
    }

    #[tokio::test]
    async fn solution_2() {
        let solution = solution("2").await;
        assert!(solution == String::from("\n#####\n#...#\n#...#\n#...#\n#####"));
    }
}
