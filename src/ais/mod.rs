// region:    --- Modules
pub mod asst;

use async_openai::{config::OpenAIConfig, Client};

use crate::Result;

// endregion: --- Modules

// region:    --- Cleint

// const ENV_OPENAI_API_KEY: &str = "OPENAP_API_KEY";

pub type OaClient = Client<OpenAIConfig>;

pub fn new_oa_client() -> Result<OaClient> {
    if std::env::var("OPENAI_API_KEY").is_ok() {
        Ok(Client::new())
    } else {
        println!("No OPENAI_API_KEY please set it.");
        Err("No openai api key in env".into())
    }
}

// endregion: --- Cleint
