use std::fs;
use std::path::PathBuf;
use reqwest::StatusCode;

#[derive(Debug)]
pub enum LoadError {
    HttpError(String),
    ReadError(String),
    WriteError(String),
}

impl From<reqwest::Error> for LoadError {
    fn from(e: reqwest::Error) -> Self {
        LoadError::HttpError(e.to_string())
    }
}


pub async fn load<'a>(day_num: &str, year: &str, session: &str, allow_remote: bool, base_path_opt: Option<&str>) -> Result<String, Vec<LoadError>> {
    let base_path = base_path_opt.unwrap_or("./src");
    let path: PathBuf = [base_path, &format!("year{}", year), &format!("day{}", day_num), "input"].iter().collect();
    let local_read = load_local(&path).await;
    if let Ok(text) = local_read {
        return Ok(text);
    }
    println!("WARN: Local load failure: {:?}", local_read.as_ref().unwrap_err());

    if allow_remote {
        let remote_read = load_remote(day_num, &path, session).await;
        if let Ok(text) = remote_read {
            return Ok(text);
        }
        
        return Err(vec![local_read.unwrap_err(), remote_read.unwrap_err()]);
    }

    Err(vec![local_read.unwrap_err()])
}

pub async fn load_local(path: &PathBuf) -> Result<String, LoadError> {
    fs::read_to_string(path)
        .map_err(|e| LoadError::ReadError(format!("{} - {}", e.to_string(), path.display())))
}

async fn load_remote(day_num: &str, path: &PathBuf, session: &str) -> Result<String, LoadError> {
    let url = format!("https://adventofcode.com/2023/day/{}/input", day_num);
    let client = reqwest::Client::new();
    let request = client.get(url).header("Cookie", session).build()?;
    let response = client.execute(request).await
        .map_err(|e| LoadError::HttpError(e.status().unwrap_or(StatusCode::default()).to_string()))?;
    let text = response.text().await.map_err(|e| LoadError::HttpError(e.to_string()))?;
    fs::write(&path, &text).map_err(|e| LoadError::WriteError(e.to_string()))?;

    Ok(text)
}
