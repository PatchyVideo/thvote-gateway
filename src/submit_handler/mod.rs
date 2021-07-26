
use juniper::graphql_value;

use juniper::FieldResult;

use crate::common::ServiceError;
use crate::common::getJSON;
use crate::common::postJSON;
use crate::common::{Error, UserVerifyResult, PostResult};
use crate::context::Context;

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;

// ------------------------------------------------
// REST Schemas
// ------------------------------------------------

#[derive(Clone, Serialize, Deserialize)]
pub struct SubmitMetadata {
	/// 投票人油箱
	pub email: String,
	/// 提交时间
	pub created_at: bson::DateTime,
	/// 用户IP
	pub user_ip: String,
	/// 额外用户指纹信息
	pub additional_fingreprint: Option<String>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CharacterSubmitRest {
	pub characters: Vec<CharacterSubmit>,
	pub meta: SubmitMetadata
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MusicSubmitRest {
	pub music: Vec<MusicSubmit>,
	pub meta: SubmitMetadata
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WorkSubmitRest {
	pub works: Vec<WorkSubmit>,
	pub meta: SubmitMetadata
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CPSubmitRest {
	pub cps: Vec<CPSubmit>,
	pub meta: SubmitMetadata
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PaperSubmitRest {
	pub papers: serde_json::Map<String, serde_json::Value>,
	pub meta: SubmitMetadata
}

// ------------------------------------------------
// GQL Schemas
// ------------------------------------------------

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="Single character submit")]
pub struct CharacterSubmit {
	pub name: String,
	pub reason: Option<String>,
	pub rank: i32
}

#[derive(juniper::GraphQLInputObject, Clone)]
#[graphql(description="Character submit")]
pub struct CharacterSubmitGQL {
	pub vote_token: String,
	pub characters: Vec<CharacterSubmit>
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="Single cp submit")]
pub struct CPSubmit {
	pub name_a: String,
	pub name_b: String,
	pub name_c: Option<String>,
	pub active: Option<String>,
	pub reason: Option<String>,
	pub rank: i32
}

#[derive(juniper::GraphQLInputObject, Clone)]
#[graphql(description="CP submit")]
pub struct CPSubmitGQL {
	pub vote_token: String,
	pub cps: Vec<CPSubmit>
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="Single music submit")]
pub struct MusicSubmit {
	pub name: String,
	pub reason: Option<String>,
	pub rank: i32
}

#[derive(juniper::GraphQLInputObject, Clone)]
#[graphql(description="Music submit")]
pub struct MusicSubmitGQL {
	pub vote_token: String,
	pub musics: Vec<MusicSubmit>
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="Single work submit")]
pub struct WorkSubmit {
	pub name: String,
	pub reason: Option<String>,
	pub rank: i32
}

#[derive(juniper::GraphQLInputObject, Clone)]
#[graphql(description="Work submit")]
pub struct WorkSubmitGQL {
	pub vote_token: String,
	pub work: Vec<WorkSubmit>
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="Single paper submit")]
pub struct PaperSubmit {
	pub id: String,
	/// 答案
	pub answer: String
}

#[derive(juniper::GraphQLInputObject, Clone)]
#[graphql(description="Paper submit")]
pub struct PaperSubmitGQL {
	pub vote_token: String,
	pub paper_json: String
}

pub fn generate_submit_metadata(email: &str, context: &Context) -> SubmitMetadata {
	SubmitMetadata {
		email: email.to_string(),
		created_at: bson::DateTime(chrono::Utc::now()),
		user_ip: context.user_ip,
		additional_fingreprint: None, // TODO
	}
}

// ------------------------------------------------
// Root Quries
// ------------------------------------------------

use crate::services::*;

pub async fn submitCharacterVote_impl(context: &Context, content: &CharacterSubmitGQL) -> FieldResult<PostResult> {
	let verify_result = getJSON::<UserVerifyResult>(&format!("http://{}/v1/verify/{}", USER_MANAGER, content.vote_token)).await?;
	if let Some(email) = verify_result.user_email {
		let submit_json = CharacterSubmitRest {
			meta: generate_submit_metadata(&email, context),
			characters: content.characters.clone(),
		};
		let post_result: PostResult = postJSON(&format!("http://{}/v1/character/", SUBMIT_HANDLER), submit_json).await?;
		Ok(PostResult::new())
	} else {
		return Err(ServiceError::Forbidden.into())
	}
}

pub async fn submitMusicVote_impl(context: &Context, content: &MusicSubmitGQL) -> FieldResult<PostResult> {
	let verify_result = getJSON::<UserVerifyResult>(&format!("http://{}/v1/verify/{}", USER_MANAGER, content.vote_token)).await?;
	if let Some(email) = verify_result.user_email {
		let submit_json = MusicSubmitRest {
			meta: generate_submit_metadata(&email, context),
			music: content.musics.clone(),
		};
		let post_result: PostResult = postJSON(&format!("http://{}/v1/music/", SUBMIT_HANDLER), submit_json).await?;
		Ok(PostResult::new())
	} else {
		return Err(ServiceError::Forbidden.into())
	}
}

pub async fn submitCPVote_impl(context: &Context, content: &CPSubmitGQL) -> FieldResult<PostResult> {
	let verify_result = getJSON::<UserVerifyResult>(&format!("http://{}/v1/verify/{}", USER_MANAGER, content.vote_token)).await?;
	if let Some(email) = verify_result.user_email {
		let submit_json = CPSubmitRest {
			meta: generate_submit_metadata(&email, context),
			cps: content.cps.clone(),
		};
		let post_result: PostResult = postJSON(&format!("http://{}/v1/cp/", SUBMIT_HANDLER), submit_json).await?;
		Ok(PostResult::new())
	} else {
		return Err(ServiceError::Forbidden.into())
	}
}

pub async fn submitWorkVote_impl(context: &Context, content: &WorkSubmitGQL) -> FieldResult<PostResult> {
	let verify_result = getJSON::<UserVerifyResult>(&format!("http://{}/v1/verify/{}", USER_MANAGER, content.vote_token)).await?;
	if let Some(email) = verify_result.user_email {
		let submit_json = WorkSubmitRest {
			meta: generate_submit_metadata(&email, context),
			works: content.work.clone(),
		};
		let post_result: PostResult = postJSON(&format!("http://{}/v1/work/", SUBMIT_HANDLER), submit_json).await?;
		Ok(PostResult::new())
	} else {
		return Err(ServiceError::Forbidden.into())
	}
}

pub async fn submitPaperVote_impl(context: &Context, content: &PaperSubmitGQL) -> FieldResult<PostResult> {
	let verify_result = getJSON::<UserVerifyResult>(&format!("http://{}/v1/verify/{}", USER_MANAGER, content.vote_token)).await?;
	if let Some(email) = verify_result.user_email {
		let submit_json = PaperSubmitRest {
			meta: generate_submit_metadata(&email, context),
			papers: {
				let parsed: serde_json::Map<String, serde_json::Value> = match serde_json::from_str(&content.paper_json) {
					Ok(a) => a,
					Err(_) => {
						return Err(ServiceError::InvalidContent.into());
					}
				};
				parsed
			}
		};
		let post_result: PostResult = postJSON(&format!("http://{}/v1/work/", SUBMIT_HANDLER), submit_json).await?;
		Ok(PostResult::new())
	} else {
		return Err(ServiceError::Forbidden.into())
	}
}
