mod api;
mod cli;
mod cnf;
mod db;
mod err;
mod models;
mod net;

fn main() -> Result<(), err::Error> {
    cli::init()?;
    Ok(())
}