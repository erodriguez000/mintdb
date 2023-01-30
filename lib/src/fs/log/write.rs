use std::{io::Write, fs::metadata};
use crate::prelude::*;
use crate::util::time;

const PATH: &str = "mint.db/log";

pub async fn log_event(event: &str) -> Result<()> {
    let ts = time::get_unix_time();
    let log = format!("[{}]: EVENT >> {}", ts, format!("{}", event));
    println!("\x1b[38;5;158m>>>>{}\x1b[0m",log);
    write_to_log(log).await?;
    Ok(())
}
pub async fn log_merge_event(event: &str) -> Result<()> {
    let ts = time::get_unix_time();
    let log = format!("[{}]: MERGE >> {}", ts, format!("{}", event));
    println!("\x1b[38;5;50m>>>>{}\x1b[0m",log);
    write_to_log(log).await?;
    Ok(())
}
pub async fn log_create_event(event: &str) -> Result<()> {
    let ts = time::get_unix_time();
    let log = format!("[{}]: CREATE >> {}", ts, format!("{}", event));
    println!("\x1b[38;5;152m>>>>{}\x1b[0m",log);
    write_to_log(log).await?;
    Ok(())
}
pub async fn log_read_event(event: &str) -> Result<()> {
    let ts = time::get_unix_time();
    let log = format!("[{}]: SELECT >> {}", ts, format!("{}", event));
    println!("\x1b[36m>>>>{}\x1b[0m",log);
    write_to_log(log).await?;
    Ok(())
}
pub async fn log_delete_event(event: &str) -> Result<()> {
    let ts = time::get_unix_time();
    let log = format!("[{}]: DELETE >> {}", ts, format!("{}", event));
    println!("\x1b[31m>>>>{}\x1b[0m",log);
    write_to_log(log).await?;
    Ok(())
}
pub async fn log_warning(event: &str) -> Result<()> {
    let ts = time::get_unix_time();
    let log = format!("[{}]: WARNING >> {}", ts, format!("{}", event));
    println!("\x1b[33m>>>>{}\x1b[0m",log);
    write_to_log(log).await?;
    Ok(())
}

pub async fn log_error(event: &str) -> Result<()> {
    let ts = time::get_unix_time();
    let log = format!("[{}]: ERROR >> {}", ts, format!("{}", event));
    println!("\x1b[31m>>>>{}\x1b[0m",log);
    write_to_log(log).await?;
    Ok(())
}

async fn write_to_log(log: String) -> Result<()> {
    match std::fs::metadata("mint.db") {
        Ok(_) => match metadata(PATH) {
            Ok(_) => (),
            Err(_) => std::fs::create_dir_all(PATH)?
        },
        Err(_) => {
            std::fs::create_dir_all(PATH)?;
        }
    };
    let path = f!("{}/log.bin", PATH);
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .create_new(false)
        .append(true)
        .write(true)
        .open(path)?;
    file.write_all(f!("{}\n", log).as_bytes())?;
    Ok(())
}
