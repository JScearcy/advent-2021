use std::str::Split;

use crate::handler::{AdventSolution, DayHandler, SolveError};

#[derive(Debug)]
pub enum Day2Error {}

impl Into<SolveError> for Day2Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day2Error: {:?}", self))
    }
}

fn get_max_colors(results: &str) -> (usize, usize, usize) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for result in results.split(";") {
        for num_and_color_str in result.split(",") {
            let num_and_color: Vec<&str> = num_and_color_str.split(" ").filter(|str| str.len() > 0).collect();
            let num = usize::from_str_radix(num_and_color[0], 10).unwrap();
            let color = num_and_color[1];

            match color {
                "red" => {
                    if red < num {
                        red = num;
                    }
                }
                "green" => {
                    if green < num {
                        green = num;
                    }
                }
                "blue" => {
                    if blue < num {
                        blue = num
                    }
                }
                _ => todo!("unsupported color")
            }
        }
    }

    (red,green,blue)
}

fn game_to_usize(game: &str) -> usize {
    let game_split: Vec<&str> = game.split(" ").collect();

    usize::from_str_radix(game_split[1], 10).unwrap()
}

pub struct Day2Handler {}
impl<'a> Day2Handler {
    pub fn new() -> DayHandler<'a, &'a str> {
        DayHandler::new(Day2Handler {})
    }
    pub fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day2Error> {
        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;
        let mut total = 0;
        for game_line in input_lines {
            let line_split: Vec<&str> = game_line.split(":").take(2).collect();
            let game = line_split[0];
            let results = line_split[1];
            let (red, green, blue) = get_max_colors(results);
            if red <= max_red && green <= max_green && blue <= max_blue {
                let game_num = game_to_usize(game);
                total += game_num;
            }
        }

        Ok(total.to_string())
    }

    pub fn solve_2(&self, input_lines: Split<&str>) -> Result<String, Day2Error> {
        let mut total = 0;
        for game_line in input_lines {
            let line_split: Vec<&str> = game_line.split(":").take(2).collect();
            let results = line_split[1];
            let (red, green, blue) = get_max_colors(results);
            let power = red * green * blue;
            total += power;
        }

        Ok(total.to_string())
    }
}

impl<'a> AdventSolution<&str> for Day2Handler {
    fn get_day(&self) -> String {
        String::from("2")
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
    use super::Day2Handler;
    use crate::handler::AdventSolution;

    fn get_input<'a>() -> &'a str {
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    }

    #[test]
    fn get_day() {
        let handler = Day2Handler::new();
        assert!(&handler.get_day() == "2");
    }

    async fn solution(sol: &str) -> String {
        let handler = Day2Handler::new();
        handler.solve(sol, get_input()).unwrap()
    }

    #[tokio::test]
    async fn solution_1() {
        let solution = solution("1").await;
        assert!(solution == String::from("8"));
    }

    #[tokio::test]
    async fn solution_2() {
        let solution = solution("2").await;
        assert!(solution == String::from("2286"));
    }
}
