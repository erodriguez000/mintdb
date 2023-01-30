use crate::err::Error;
mod config;
mod start;

pub fn init() -> Result<(), Error> {
    start::init()?;
    Ok(())
}