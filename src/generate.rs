use std::{fs, io};
use std::path::PathBuf;
use tera::{Context,Tera};

#[derive(Debug)]
pub enum GenerateError {
    TeraError(tera::Error),
    IoError(io::Error)
}

impl From<tera::Error> for GenerateError {
    fn from(e: tera::Error) -> Self { GenerateError::TeraError(e) }
}
impl From<io::Error> for GenerateError {
    fn from(e: io::Error) -> Self { GenerateError::IoError(e) }
}

pub fn generate_day(day_num: &str, year: &str) -> Result<(), GenerateError> {
    let tera = Tera::new("src/templates/**/*.rst")?;
    let day = day_num.parse::<usize>().unwrap();
    let mut tera_context = Context::new();
    tera_context.insert("day_num", day_num);
    tera_context.insert("days", &(1..=day).collect::<Vec<usize>>());
    let day = tera.render("day_handler.rst", &tera_context)?;
    let year_module = tera.render("year_mod.rst", &tera_context)?;
    
    let year_dir_path: PathBuf = ["./src", &format!("year{}", year)].iter().collect();
    let dir_path: PathBuf = [&format!("{}", year_dir_path.display()), &format!("day{}", day_num)].iter().collect();
    let year_file_path: PathBuf = [&format!("{}", year_dir_path.display()), "mod.rs"].iter().collect();
    let day_file_path: PathBuf = [&format!("{}", dir_path.display()), "mod.rs"].iter().collect();
    fs::create_dir(dir_path)?;
    fs::write(day_file_path, day)?;
    fs::write(year_file_path, year_module)?;
    
    println!("Generated day {}", day_num);
    Ok(())
}
