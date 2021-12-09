mod cli;
mod generate;
mod handler;
mod load_input;
mod year2021;

use handler::SolutionHandler;
use year2021::prelude::*;


#[tokio::main]
async fn main() {
    let matches = cli::init();
    let day = matches.value_of("day").unwrap();
    let year = matches.value_of("year").unwrap_or("2021");
    if matches.is_present("generate") {
        match generate::generate_day(day, year) {
            Ok(()) => {},
            Err(e) => println!("{:?}", e)
        }
        return;
    }
    let challenge = matches.value_of("challenge").unwrap();
    let allow_remote = matches.is_present("remote");
    let session = matches.value_of("session").unwrap_or("");
    let raw_input = load_input::load(day, year, session, allow_remote, None).await.unwrap();

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
        Day9Handler::new(),
    ];
    
    solution_handler.register(handlers);

    let (duration, res) = solution_handler.solve(day, challenge, &raw_input);

    match res {
        Ok(text) => println!("Day {} challenge {} result: {}", day, challenge, text),
        Err(e) => println!("Error running solution: {:?}", e),
    };

    let display_time = if duration.num_milliseconds() > 0 {
        duration.num_milliseconds() as f64
    } else {
        duration.num_microseconds().unwrap() as f64 / 1000.
    };
    println!("Took {} ms to solve", display_time);
}
