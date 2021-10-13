use juniper::FieldResult;
use juniper::GraphQLSubscriptionValue;
use juniper::RootNode;

use chrono::{DateTime, Utc};

use crate::common::PostResult;
use crate::submit_handler::CPSubmitGQL;
use crate::submit_handler::CharacterSubmitGQL;
use crate::submit_handler::MusicSubmitGQL;
use crate::submit_handler::PaperSubmitGQL;
use crate::submit_handler::WorkSubmitGQL;
use crate::user_manager::EmailLoginInputs;
use crate::user_manager::EmailLoginInputsForExistingVoters;
use crate::user_manager::LoginResults;
use crate::user_manager::PhoneLoginInputs;

use crate::{user_manager, submit_handler, vote_data, result_query};

use super::context::Context;

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
	fn apiVersion() -> &str {
		"1.0"
	}

	fn serverDate() -> DateTime<Utc> {
		Utc::now()
	}

	// ------------------------------------------------
	//     result_query
	// ------------------------------------------------

	// /// 人物投票理由
	// fn character_reasons(name: String) -> FieldResult<Reasons> {
	// 	result_query::character_reasons_impl(name)
	// }

	// /// 人物投票结果
	// fn character_rank_result(filter: Option<FilterConditions>) -> FieldResult<CharacterRankResult> {
	// 	result_query::character_rank_result_impl(filter)
	// }

	// /// 人物投票理由
	// fn single_character_result(name: String, filter: Option<FilterConditions>) -> FieldResult<SingleCharacterResult> {
	// 	result_query::single_character_result_impl(name, filter)
	// }

	
	// ------------------------------------------------
	//     vote data
	// ------------------------------------------------
	async fn listVotableCharacters(context: &Context) -> FieldResult<vote_data::VotableCharacters> {
		vote_data::listVotableCharacters_impl(context).await
	}
	async fn listVotableWorks(context: &Context) -> FieldResult<vote_data::VotableWorks> {
		vote_data::listVotableWorks_impl(context).await
	}
	async fn listVotableMusics(context: &Context) -> FieldResult<vote_data::VotableMusics> {
		vote_data::listVotableMusics_impl(context).await

	}
}


pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
	
	fn apiVersion() -> &str {
		"1.0"
	}

	fn serverDate() -> DateTime<Utc> {
		Utc::now()
	}

	// ------------------------------------------------
	//     user_manager
	// ------------------------------------------------

	/// 老用户使用email帐号登录
	async fn login_email_password(context: &Context, email: String, password: String) -> FieldResult<LoginResults> {
		user_manager::login_email_password(context, email, password).await
	}

	/// 新用户使用email帐号登录
	async fn login_email(context: &Context,  email: String, nickname: Option<String>, verify_code: String) -> FieldResult<LoginResults> {
		user_manager::login_email(context, email, nickname, verify_code).await
	}
	/// 向邮箱发送验证码
	async fn request_email_code(context: &Context, email: String) -> FieldResult<bool> {
		user_manager::request_email_code(context, email).await
	}

	/// 使用手机帐号登录
	async fn login_phone(context: &Context, phone: String, nickname: Option<String>, verify_code: String) -> FieldResult<LoginResults> {
		user_manager::login_phone(context, phone, nickname, verify_code).await
	}
	/// 向手机发送验证码
	async fn request_phone_code(context: &Context, phone: String) -> FieldResult<bool> {
		user_manager::request_phone_code(context, phone).await
	}

	// ------------------------------------------------
	//     submit_handler
	// ------------------------------------------------

	/// Character
	async fn submitCharacterVote(context: &Context, content: CharacterSubmitGQL) -> FieldResult<PostResult> {
		submit_handler::submitCharacterVote_impl(context, &content).await
	}

	/// music
	async fn submitMusicVote(context: &Context, content: MusicSubmitGQL) -> FieldResult<PostResult> {
	   submit_handler::submitMusicVote_impl(context, &content).await
	}

	/// work
	async fn submitWorkVote(context: &Context, content: WorkSubmitGQL) -> FieldResult<PostResult> {
		submit_handler::submitWorkVote_impl(context, &content).await
	}

	/// CP
	async fn submitCPVote(context: &Context, content: CPSubmitGQL) -> FieldResult<PostResult> {
		submit_handler::submitCPVote_impl(context, &content).await
	}

	/// paper
	async fn submitPaperVote(context: &Context, content: PaperSubmitGQL) -> FieldResult<PostResult> {
		submit_handler::submitPaperVote_impl(context, &content).await
	}
}

pub struct Subscription;

#[juniper::graphql_object(Context = Context)]
impl Subscription {
	
	fn apiVersion() -> &str {
		"1.0"
	}

	fn serverDate() -> DateTime<Utc> {
		Utc::now()
	}

}

impl GraphQLSubscriptionValue for Subscription {
	
}

pub type Schema = RootNode<'static, Query, Mutation, Subscription>;

pub fn create_schema() -> Schema {
	Schema::new(Query {}, Mutation {}, Subscription {})
}
