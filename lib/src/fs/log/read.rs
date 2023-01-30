use std::io::BufRead;
use crate::prelude::*;

const PATH: &str = "mint.db/log/log.bin";
pub async fn fetch_logs() -> Result<Vec<String>> {
    let file = std::fs::File::open(PATH)?;
    let reader = std::io::BufReader::new(file);
    let mut res = vec![];

    for line in reader.lines() {
        let line = line.expect("failed to read");
        res.push(line);
    }
    Ok(res)
}

pub async fn fetch_merge_logs() -> Result<Vec<String>> {
    let file = std::fs::File::open(PATH)?;
    let reader = std::io::BufReader::new(file);
    let mut res = vec![];

    for line in reader.lines() {
        let line = line.expect("failed to read");
        if line.contains("MERGE") {
            res.push(line);
        }
    }
    Ok(res)
}

pub async fn fetch_create_logs() -> Result<Vec<String>> {
    let file = std::fs::File::open(PATH)?;
    let reader = std::io::BufReader::new(file);
    let mut res = vec![];

    for line in reader.lines() {
        let line = line.expect("failed to read");
        if line.contains("CREATE") {
            res.push(line);
        }
    }
    Ok(res)
}

pub async fn fetch_read_logs() -> Result<Vec<String>> {
    let file = std::fs::File::open(PATH)?;
    let reader = std::io::BufReader::new(file);
    let mut res = vec![];

    for line in reader.lines() {
        let line = line.expect("failed to read");
        if line.contains("SELECT") {
            res.push(line);
        }
    }
    Ok(res)
}
pub async fn fetch_delete_logs() -> Result<Vec<String>> {
    let file = std::fs::File::open(PATH)?;
    let reader = std::io::BufReader::new(file);
    let mut res = vec![];

    for line in reader.lines() {
        let line = line.expect("failed to read");
        if line.contains("DELETE") {
            res.push(line);
        }
    }
    Ok(res)
}
pub async fn fetch_event_logs() -> Result<Vec<String>> {
    let file = std::fs::File::open(PATH)?;
    let reader = std::io::BufReader::new(file);
    let mut res = vec![];

    for line in reader.lines() {
        let line = line.expect("failed to read");
        if line.contains("EVENT") {
            res.push(line);
        }
    }
    Ok(res)
}

pub async fn fetch_warning_logs() -> Result<Vec<String>> {
    let file = std::fs::File::open(PATH)?;
    let reader = std::io::BufReader::new(file);
    let mut res = vec![];

    for line in reader.lines() {
        let line = line.expect("failed to read");
        if line.contains("WARNING") {
            res.push(line);
        }
    }
    Ok(res)
}

pub async fn fetch_error_logs() -> Result<Vec<String>> {
    let file = std::fs::File::open(PATH)?;
    let reader = std::io::BufReader::new(file);
    let mut res = vec![];

    for line in reader.lines() {
        let line = line.expect("failed to read");
        if line.contains("ERROR") {
            res.push(line);
        }
    }
    Ok(res)
}