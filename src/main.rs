use chrono::Utc;
use handler::SolutionHandler;

mod cli;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod generate;
mod handler;
mod load_input;

use crate::day1::Day1Handler;
use crate::day2::Day2Handler;
use crate::day3::Day3Handler;
use crate::day4::Day4Handler;
use crate::day5::Day5Handler;
use crate::day6::Day6Handler;
use crate::day7::Day7Handler;
use crate::day8::Day8Handler;

#[tokio::main]
async fn main() {
    let matches = cli::init();
    let day = matches.value_of("day").unwrap();
    if matches.is_present("generate") {
        match generate::generate_day(day) {
            Ok(()) => {},
            Err(e) => println!("{:?}", e)
        }
        return;
    }
    let challenge = matches.value_of("challenge").unwrap();
    let allow_remote = matches.is_present("remote");
    let session = matches.value_of("session").unwrap_or("");
    let raw_input = load_input::load(day, session, allow_remote, None).await.unwrap();

    let mut solution_handler = SolutionHandler::new();
    let handlers = vec![
        Day1Handler::new(),
        Day2Handler::new(),
        Day3Handler::new(),
        Day4Handler::new(),
        Day5Handler::new(),
        Day6Handler::new(),
        Day7Handler::new(),
        Day8Handler::new(),
    ];
    
    solution_handler.register(handlers);

    let perf_start_time = Utc::now().time();
    let res = solution_handler.solve(day, challenge, &raw_input);
    let perf_end_time = Utc::now().time();
    let diff = perf_end_time - perf_start_time;

    match res {
        Ok(text) => println!("Day {} challenge {} result: {}", day, challenge, text),
        Err(e) => println!("Error running solution: {:?}", e),
    };

    let display_time = if diff.num_milliseconds() > 0 { diff.num_milliseconds() as f64 } else { diff.num_microseconds().unwrap() as f64 / 1000. };
    println!("Took {} ms to solve", display_time);
}
