use ndarray::{Array2, Axis};
use std::str::Split;

use crate::handler::{AdventSolution, SolveError, DayHandler};

#[derive(Debug)]
pub enum Day4Error {}

impl Into<SolveError> for Day4Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day4Error: {:?}", self))
    }
}

struct Boards {
    boards: Vec<(bool, Array2<(bool, u32)>)>
}

impl Boards {
    pub fn new(boards_vec: Vec<Vec<Vec<(bool, u32)>>>) -> Boards {
        let mut boards: Vec<(bool, Array2<(bool, u32)>)> = vec![];
        for board in boards_vec {
            let mut nd_board = Array2::<(bool, u32)>::default((5, 5));
            for (i, mut row) in nd_board.axis_iter_mut(Axis(0)).enumerate() {
                for (j, col) in row.iter_mut().enumerate() {
                    *col = board[i][j];
                }
            }
            boards.push((false, nd_board));
        }

        Boards { boards }
    }

    pub fn len(&self) -> usize { self.boards.len() }

    pub fn process_numbers<T>(&mut self, numbers: Vec<u32>, break_on_win: &mut dyn FnMut(u32, &Array2<(bool, u32)>) -> (bool, T)) -> Option<T> {
        let mut result = None;
        'number: for number in numbers {
            for board in self.boards.iter_mut().enumerate() {
                if !board.1.0 {
                    Boards::update_board(&mut board.1.1, number);
                    if Boards::board_win(&board.1.1) {
                        let win_cb = break_on_win(number, &board.1.1);
                        board.1.0 = true;
                        result = Some(win_cb.1);
                        if win_cb.0 {
                            break 'number;
                        }
                    }
                }
            }
        }

        result
    }

    fn update_board(board: &mut Array2<(bool, u32)>, number: u32) {
        for board_row in board.rows_mut() {
            for mut el in board_row {
                if el.1 == number {
                    el.0 = true;
                } 
            }
        }
    }

    fn board_win(board: &Array2<(bool, u32)>) -> bool {
        let mut win = false;
        ndarray::Zip::from(board.axis_iter(Axis(0))).and(board.axis_iter(Axis(1))).for_each(|axis_0, axis_1| {
            let axis_0_win = axis_0.fold(true,|acc, val| if acc { val.0 } else { acc });
            let axis_1_win = axis_1.fold(true,|acc, val| if acc { val.0 } else { acc });
            if !win {
                win = axis_0_win || axis_1_win;
            }
        });

        win
    }
}
pub struct Day4Handler {}
impl<'a> Day4Handler {
    pub fn new() -> DayHandler<'a, &'a str> { DayHandler::new(Day4Handler {}) }
    pub fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day4Error> {
        let (numbers, mut boards) = self.init(input_lines);

        let final_num_score_res = boards.process_numbers(numbers, &mut |number, board| {
            (true, (number, self.calculate_score(&board)))
        });
        let final_num_score = final_num_score_res.unwrap();

        Ok(format!("{}", final_num_score.0 * final_num_score.1))
    }
    
    pub fn solve_2(&self, input_lines: Split<&str>) -> Result<String, Day4Error> {
        let (numbers, mut boards) = self.init(input_lines);

        let wins_needed = boards.len();
        let mut wins = 0;
        let final_num_score_res = boards.process_numbers(numbers, &mut |number, board| {
            wins += 1;
            (wins == wins_needed, (number, self.calculate_score(&board)))
        });
        let final_num_score = final_num_score_res.unwrap();

        Ok(format!("{}", final_num_score.0 * final_num_score.1))
    }

    fn init(&self, mut input_lines: Split<&str>) -> (Vec<u32>, Boards) {
        let numbers: Vec<u32> = input_lines.next().unwrap().split(",")
            .map(|val| val.parse::<u32>().unwrap()).collect();
        let boards = self.get_boards(input_lines);
        
        (numbers, boards)
    }

    fn get_boards(&self, board_lines: Split<&str>) -> Boards {
        let mut boards_vec: Vec<Vec<Vec<(bool, u32)>>> = vec![];
        for line in board_lines {
            if line.len() > 0 {
                let board_line = self.process_bingo_line(line);
                let board_idx = boards_vec.len() - 1;
                let board = &mut boards_vec[board_idx];
                board.push(board_line);
            } else {
                boards_vec.push(vec![]);
            }
        }

        Boards::new(boards_vec)
    }

    fn process_bingo_line(&self, line: &str) -> Vec<(bool, u32)> {
        line.split(" ")
            .filter(|val| val.len() > 0)
            .map(|bingo_num| (false, bingo_num.parse::<u32>().unwrap())).collect()
    }

    fn calculate_score(&self, board: &Array2<(bool, u32)>) -> u32 {
        board.fold(0, |acc, val| if !val.0 { acc + val.1 } else { acc })
    }
}

impl<'a> AdventSolution<&str> for Day4Handler {
    fn get_day(&self) -> String { String::from("4") }
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
    use super::Day4Handler;
    fn get_input<'a>() -> &'a str {
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
    }

    #[tokio::test]
    async fn solution_1() {
        let handler = Day4Handler::new();
        let solution = handler.solve("1", get_input()).unwrap();
        assert!(solution == String::from("4512"), "Returned: {}, expected: {}", solution, "4512");
    }

    #[tokio::test]
    async fn solution_2() {
        let handler = Day4Handler::new();
        let solution = handler.solve("2", get_input()).unwrap();
        assert!(solution == String::from("1924"), "Returned: {}, expected: {}", solution, "1924");
    }
}
