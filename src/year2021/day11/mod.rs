use std::{str::Split, fmt::Display, collections::VecDeque};

use crate::handler::{AdventSolution, SolveError, DayHandler};

#[derive(Debug)]
pub enum Day11Error {}

impl Into<SolveError> for Day11Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day11Error: {:?}", self))
    }
}

#[derive(Clone,Debug)]
enum FlashState {
    NoFlash,
    Flash
}


#[derive(Clone, Debug)]
struct Point(usize, usize);
impl Point {
    fn from(maybe: (isize, isize), width: isize, height: isize) -> Option<Point> {
        let (x, y) = maybe;
        if x >= 0 && y >= 0 && x < width && y < height {
            Some(Point(x as usize, y as usize))
        } else {
            None
        }
    }
}

#[derive(Clone,Debug)]
struct Octo(usize, FlashState);
impl Octo {
    fn from(serialized: char) -> Octo {
        let energy = serialized.to_digit(10).unwrap() as usize;
        Octo(energy, FlashState::NoFlash)
    }

    fn increase(&mut self, point: Point, width: isize, height: isize) -> Option<[Option<Point>; 8]> {
        self.0 = self.0 + 1;
        if self.0 > 9 {
            self.0 = 0;
            self.1 = FlashState::Flash;
            Some(OctoGrid::get_neighbor(point, width, height))
        } else {
            None
        }
    }

    fn reset_flash(&mut self) {
        self.1 = FlashState::NoFlash;
    }
}
#[derive(Clone,Debug)]
struct OctoGrid{
    grid: Vec<Vec<Octo>>,
    height: usize,
    width: usize,
}

impl OctoGrid {
    fn new(lines: Split<&str>) -> OctoGrid {
        let grid: Vec<Vec<Octo>> = lines.fold(vec![], |mut acc, oct_line| {
            let oct_parsed = oct_line.chars().fold(vec![], |mut octo_acc, octo| {
                octo_acc.push(Octo::from(octo));
                octo_acc
            });
            acc.push(oct_parsed);
            acc
        });
        OctoGrid {
            height: grid.len(),
            width: grid[0].len(),
            grid,
        }
    }

    fn get_neighbor(point: Point, width: isize, height: isize) -> [Option<Point>; 8] {
        let (x, y) = (point.0, point.1);

        let north = Point::from((x as isize, y as isize - 1), width, height);
        let south = Point::from((x as isize, y as isize + 1), width, height);
        let east = Point::from((x as isize + 1, y as isize), width, height);
        let west = Point::from((x as isize - 1, y as isize), width, height);

        let northeast = Point::from((x as isize + 1, y as isize - 1), width, height);
        let northwest = Point::from((x as isize - 1, y as isize - 1), width, height);
        let southeast = Point::from((x as isize + 1, y as isize + 1), width, height);
        let southwest = Point::from((x as isize - 1, y as isize + 1), width, height);

        [north, south, east, west, northeast, northwest, southeast, southwest]
    } 
}

impl Display for OctoGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt_line = |line: &Vec<Octo>| {
            line.iter().fold(Ok(()), |_, octo| {
                write!(f, "{}", octo.0)
            })?;
            write!(f, "\n")
        };

        for line in &self.grid {
            fmt_line(line)?;
        }

        Ok(())
    }
}

impl Iterator for OctoGrid {
    type Item = (usize, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let mut flashes = 0;
        let mut flash_queue = VecDeque::new();
        let width = self.width as isize;
        let height = self.height as isize;
        // increase energy for each octo, pushing neighbors affected by flashing into a queue
        for (y, octo_line) in self.grid.iter_mut().enumerate() {
            for (x, octo) in octo_line.iter_mut().enumerate() {
                let neighbors_opt = octo.increase(Point(x, y), width, height);
                if let Some(neighbors) = neighbors_opt {
                    neighbors.iter().for_each(|neighbor| {
                        if neighbor.is_some() {
                            flash_queue.push_back(neighbor.clone().unwrap())
                        }
                    });
                }
            }
        }
        // drain flash_queue, increasing and flashing as needed for affected octos (NoFlash octos still increase energy)
        while flash_queue.len() > 0 {
            let Point(x, y) = flash_queue.pop_front().unwrap();
            let octo =  &mut self.grid[y][x];
            match octo.1 {
                FlashState::NoFlash => {
                    let neighbors_opt = octo.increase(Point(x, y), width, height);
                    if let Some(neighbors) = neighbors_opt {
                        neighbors.iter().for_each(|neighbor| {
                            if neighbor.is_some() {
                                flash_queue.push_back(neighbor.clone().unwrap())
                            }
                        });
                    }
                }
                _ => {}
            }
        }
        // reset flash state for all octos, while resetting. check if any Octo is NoFlash, if all have flashed, we have synchronized
        let mut all_sync = true;
        for octo_line in self.grid.iter_mut() {
            for octo in octo_line.iter_mut() {
                match octo.1 {
                    FlashState::Flash => { flashes += 1},
                    FlashState::NoFlash => { all_sync = false },
                }
                octo.reset_flash();
            }
        }

        Some((flashes, all_sync))
    }
}

pub struct Day11Handler {}
impl<'a> Day11Handler {
    pub fn new() -> DayHandler<'a, &'a str> { DayHandler::new(Day11Handler {}) }
    pub fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day11Error> {
        let octo_grid = OctoGrid::new(input_lines);

        let mut total_flashes = 0;
        for (_step, (flashes, _all_sync)) in (0..100).zip(octo_grid.clone()) {
            total_flashes += flashes;
        }

        Ok(format!("{}", total_flashes))
    }
    
    pub fn solve_2(&self, input_lines: Split<&str>) -> Result<String, Day11Error> {
        let octo_grid = OctoGrid::new(input_lines);

        let mut total_steps = 0;
        let mut octo_iter = octo_grid.into_iter();
        loop {
            if let Some((_flashes, all_sync)) = octo_iter.next() {
                total_steps += 1;
                if all_sync {
                    break;
                }
            } else {
                panic!("octo_iter didn't return something");
            }
        }

        Ok(format!("{}", total_steps))
    }
}

impl<'a> AdventSolution<&str> for Day11Handler {
    fn get_day(&self) -> String { String::from("11") }
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
    use super::Day11Handler;

    fn get_input<'a>() -> &'a str {
"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
    }

    #[test]
    fn get_day() {
        let handler = Day11Handler::new();
        assert!(&handler.get_day() == "11");
    }

    async fn solution(sol: &str) -> String {
        let handler = Day11Handler::new();
        handler.solve(sol, get_input()).unwrap()
    }

    #[tokio::test]
    async fn solution_1() {
        let solution = solution("1").await;
        assert!(solution == String::from("1656"));
    }

    #[tokio::test]
    async fn solution_2() {
        let solution = solution("2").await;
        assert!(solution == String::from("195"));
    }
}
