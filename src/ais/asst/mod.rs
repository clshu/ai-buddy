use async_openai::types::{
	AssistantObject, AssistantToolsRetrieval, CreateAssistantRequest,
};
use derive_more::{Deref, Display, From};

use crate::Result;

use super::OaClient;

// region:    --- Constants
const DEFAULT_QUERY: &[(&str, &str)] = &[("limit", "100")];

// endregion: --- Constants

// region:    --- Types

pub struct CreateConfig {
	pub name: String,
	pub model: String,
}

#[derive(Debug, From, Deref, Display)]
pub struct AsstId(String);

#[derive(Debug, From, Deref, Display)]
pub struct ThreadId(String);

#[derive(Debug, From, Deref, Display)]
pub struct FileId(String);

// endregion: --- Types

// region:    --- Assistant CRUD

pub async fn create(oac: &OaClient, config: CreateConfig) -> Result<AsstId> {
	let oa_assts = oac.assistants();

	let asst_obj = oa_assts
		.create(CreateAssistantRequest {
			model: config.model,
			name: Some(config.name),
			tools: Some(vec![AssistantToolsRetrieval::default().into()]),
			..Default::default()
		})
		.await?;

	Ok(asst_obj.id.into())
}

pub async fn load_or_create(
	oac: &OaClient,
	config: CreateConfig,
	recreate: bool,
) -> Result<AsstId> {
	let asst_obj = first_by_name(oac, &config.name).await?;
	let mut asst_id = asst_obj.map(|a| AsstId::from(a.id));

	// -- Delete asst if recreate is true
	if let (true, Some(asst_id_ref)) = (recreate, asst_id.as_ref()) {
		delete(oac, asst_id_ref).await?;
		asst_id.take(); // asst_id becomes None
		println!("->> asst id deleted from {}", config.name);
	}

	// -- Create if needed
	if let Some(asst_id) = asst_id {
		println!("Assistant {} loaded", config.name);
		Ok(asst_id)
	} else {
		let asst_name = config.name.clone();
		let asst_id = create(oac, config).await?;
		Ok(asst_id)
	}
}

pub async fn first_by_name(
	oac: &OaClient,
	name: &str,
) -> Result<Option<AssistantObject>> {
	// Get assitants manager
	let oa_assts = oac.assistants();
	// Get assistants
	let assts = oa_assts.list(DEFAULT_QUERY).await?.data;
	//  find asst by name
	let asst_obj = assts
		.into_iter()
		.find(|a| a.name.as_ref().map(|n| n == name).unwrap_or(false));

	Ok(asst_obj)
}

pub async fn delete(oac: &OaClient, asst_id: &AsstId) -> Result<()> {
	let oa_assts = oac.assistants();

	//  TODO: delete files

	// -- Delete assitant
	oa_assts.delete(asst_id).await?;

	Ok(())
}

// endregion: --- Assistant CRUD
