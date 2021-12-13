use std::collections::{HashMap, HashSet};
use std::str::Split;

use crate::handler::{AdventSolution, SolveError, DayHandler};

#[derive(Debug)]
pub enum Day12Error {}

impl Into<SolveError> for Day12Error {
    fn into(self) -> SolveError {
        SolveError(format!("Day12Error: {:?}", self))
    }
}

pub struct Day12Handler {}
impl<'a> Day12Handler {
    pub fn new() -> DayHandler<'a, &'a str> { DayHandler::new(Day12Handler {}) }
    pub fn solve_1(&self, input_lines: Split<&str>) -> Result<String, Day12Error> {
        let cave_map = input_lines.fold(HashMap::<&str, HashSet<&str>>::new(), |mut map, connect| {
            let caves: Vec<&str> = connect.split("-").collect();
            let cave_key = caves[0];
            let cave_connect = caves[1];
            if map.contains_key(cave_key) {
                map.get_mut(cave_key).map(|val| val.insert(cave_connect));
            } else {
                let mut connect_set = HashSet::new();
                connect_set.insert(cave_connect);
                map.insert(caves[0], connect_set);
            }

            if map.contains_key(cave_connect) {
                map.get_mut(cave_connect).map(|val| val.insert(cave_key));
            } else {
                let mut connect_set = HashSet::new();
                connect_set.insert(cave_key);
                map.insert(cave_connect, connect_set);
            }

            map
        });

        let mut paths: Vec<Vec<&str>> = vec![];
        let mut start_stack: Vec<&str> = cave_map.get("start").unwrap().clone().into_iter().collect();
        while start_stack.len() > 0 {
            let start_item = start_stack.pop().unwrap();
            let mut path_taken = vec!["start", start_item];
            let mut path_stack = vec![];
            cave_map.get(start_item).unwrap().iter().for_each(|val| {
                if *val != "start" { path_stack.push(*val); }
            });
            while path_stack.len() > 0 { 
                let element = path_stack.pop().unwrap();
                if element == "|" {
                    path_taken.pop().unwrap();
                    continue;
                }
                if element == "end" {
                    path_taken.push("end");
                    paths.push(path_taken.clone());
                    path_taken.pop().unwrap();
                    continue;
                } else if path_taken.contains(&element) && element.to_lowercase() == element {
                    continue;
                } else {
                    path_taken.push(element);
                    path_stack.push("|");
                    cave_map.get(element).unwrap().iter().for_each(|val| {
                        if "start" != *val && (val.to_lowercase() != *val || !path_taken.contains(val)) {
                            path_stack.push(*val);
                        }
                    });
                }
            }
        }
        
        Ok(format!("{}", paths.len()))
    }
    
    pub fn solve_2(&self, input_lines: Split<&str>) -> Result<String, Day12Error> {
        let cave_map = input_lines.fold(HashMap::<&str, HashSet<&str>>::new(), |mut map, connect| {
            let caves: Vec<&str> = connect.split("-").collect();
            let cave_key = caves[0];
            let cave_connect = caves[1];
            if map.contains_key(cave_key) {
                map.get_mut(cave_key).map(|val| val.insert(cave_connect));
            } else {
                let mut connect_set = HashSet::new();
                connect_set.insert(cave_connect);
                map.insert(caves[0], connect_set);
            }

            if map.contains_key(cave_connect) {
                map.get_mut(cave_connect).map(|val| val.insert(cave_key));
            } else {
                let mut connect_set = HashSet::new();
                connect_set.insert(cave_key);
                map.insert(cave_connect, connect_set);
            }

            map
        });

        let mut paths: Vec<Vec<&str>> = vec![];
        let mut start_stack: Vec<&str> = cave_map.get("start").unwrap().clone().into_iter().collect();
        while start_stack.len() > 0 {
            let start_item = start_stack.pop().unwrap();
            let mut path_taken = vec!["start", start_item];
            let mut path_stack = vec![];
            cave_map.get(start_item).unwrap().iter().for_each(|val| {
                if *val != "start" { path_stack.push(*val); }
            });
            while path_stack.len() > 0 {
                let element = path_stack.pop().unwrap();
                if element == "|" {
                    path_taken.pop().unwrap();
                    continue;
                }
                
                let (has_double_dup, _seen) = path_taken.iter().filter(|val| &&val.to_lowercase() == val)
                    .fold((None, HashSet::<&str>::new()), |dup, val| {
                        let (has_dup, mut seen) = dup;
                        if has_dup.is_some() {
                            (has_dup, seen)
                        } else {
                            if seen.contains(val) {
                                (Some(*val), seen)
                            } else {
                                seen.insert(val);
                                (has_dup, seen)
                            }
                        }
                    });

                if element == "end" {
                    path_taken.push("end");
                    paths.push(path_taken.clone());
                    path_taken.pop().unwrap();
                    continue;
                } else if element.to_lowercase() == element && (has_double_dup.is_some() && path_taken.contains(&element)) {
                    continue;
                } else {
                    path_taken.push(element);
                    path_stack.push("|");
                    cave_map.get(element).unwrap().iter().for_each(|val| {
                        if "start" != *val {
                            path_stack.push(*val);
                        }
                    });
                }
            }
        }
        
        Ok(format!("{}", paths.len()))
    }
}

impl<'a> AdventSolution<&str> for Day12Handler {
    fn get_day(&self) -> String { String::from("12") }
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
    use super::Day12Handler;

    fn get_input<'a>() -> &'a str {
"start-A
start-b
A-c
A-b
b-d
A-end
b-end"
    }

    #[test]
    fn get_day() {
        let handler = Day12Handler::new();
        assert!(&handler.get_day() == "12");
    }

    async fn solution(sol: &str) -> String {
        let handler = Day12Handler::new();
        handler.solve(sol, get_input()).unwrap()
    }

    #[tokio::test]
    async fn solution_1() {
        let solution = solution("1").await;
        assert!(solution == String::from("10"));
    }

    #[tokio::test]
    async fn solution_2() {
        let solution = solution("2").await;
        assert!(solution == String::from("36"));
    }
}
