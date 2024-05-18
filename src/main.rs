// region:    --- Modules

mod ais;
mod buddy;
mod error;

use ais::{
	asst::{self, CreateConfig},
	new_oa_client,
};
use dotenv::dotenv;

use self::error::{Error, Result};

// endregion: --- Modules

#[tokio::main]
async fn main() {
	dotenv().ok();

	match start().await {
		Ok(()) => println!("Hellow World"),
		Err(err) => println!("Error: {err:?}"),
	}
}

async fn start() -> Result<()> {
	let oac = new_oa_client()?;
	let asst_config = CreateConfig {
		name: "buddy-01".to_string(),
		model: "gpt-3.5-turbo".to_string(),
	};

	let asst_id = asst::load_or_create(&oac, asst_config, false).await?;

	println!("->> asst id: {asst_id}");

	Ok(())
}
