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

pub fn generate_day(day_num: &str) -> Result<(), GenerateError> {
    let tera = Tera::new("src/templates/**/*.rst")?;
    let mut tera_context = Context::new();
    tera_context.insert("day_num", day_num);
    let day = tera.render("day_handler.rst", &tera_context)?;
    
    let dir_path: PathBuf = ["./src", &format!("day{}", day_num)].iter().collect();
    let file_path: PathBuf = [&format!("{}", dir_path.display()), "mod.rs"].iter().collect();
    fs::create_dir(dir_path)?;
    fs::write(file_path, day)?;
    
    println!("Generated day {}", day_num);
    Ok(())
}
