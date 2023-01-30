use crate::{
    db,
    cnf::LOGO,
	net,
};
use crate::err::Error;

#[tokio::main]
pub async fn init() -> Result<(), Error> {
	// Set the default log level
	// Output MintDB logo
	println!("\x1b[38;5;50m{}\x1b[0m", LOGO);
	// Initialize DB
	db::init().await?;
	// Start the web server
	net::init().await?;
	// All ok
	Ok(())
}