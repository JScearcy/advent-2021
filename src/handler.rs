use std::{fmt::Display, collections::HashMap};

#[derive(Debug)]
pub struct SolveError(pub String);

impl Display for SolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SolveError: {}", self.0)
    }
}

pub struct DayHandler<'a, InputType>(Box<dyn AdventSolution<InputType> + 'a>);

impl<'a, InputType> DayHandler<'a, InputType> {
    pub fn new(handler: impl AdventSolution<InputType> + 'a) -> DayHandler<'a, InputType> {
        DayHandler(Box::new(handler))
    }
}

impl<'a, InputType> AdventSolution<InputType> for DayHandler<'a, InputType> {
    fn get_day(&self) -> String { self.0.get_day()}
    fn solve(&self, problem: &str, input: InputType) -> Result<String, SolveError> { self.0.solve(problem, input) }
}

pub trait AdventSolution<InputType> {
    fn get_day(&self) -> String;
    fn solve(&self, problem: &str, input: InputType) -> Result<String, SolveError>;
}


pub struct SolutionHandler<'a, InputType> {
    handlers: HashMap<String, DayHandler<'a, InputType>>,
}

impl<'a, InputType> SolutionHandler<'a, InputType> {
    pub fn new() -> SolutionHandler<'a, InputType> { SolutionHandler {handlers: HashMap::new()} }

    pub fn register(&mut self, handlers: Vec<DayHandler<'a, InputType>>) {
        for handler in handlers {
            let handler_day = handler.get_day();
            self.handlers.insert(handler_day, handler);
        }
    }

    pub fn solve(&self, day: &str, problem: &str, input: InputType) -> Result<String, SolveError> {
        let handler_opt = self.handlers.get(day);
        if let Some(handler) = handler_opt {
            return handler.solve(problem, input);
        }
        Err(SolveError(format!("NotFound: day {}, problem {}", day, problem)))
    }
}


