use juniper::FieldResult;
use juniper::GraphQLSubscriptionValue;
use juniper::RootNode;

use chrono::{DateTime, Utc};

use crate::common::PostResult;
use self::submit_handler::CPSubmitGQL;
use self::submit_handler::MusicSubmitGQL;
use self::submit_handler::PaperSubmitGQL;
use self::submit_handler::WorkSubmitGQL;

use super::context::Context;

#[path="submit_handler/mod.rs"]
mod submit_handler;
use submit_handler::{CharacterSubmitGQL};

#[path="result_query/mod.rs"]
mod result_query;
use result_query::{CharacterRankResult, Reasons, FilterConditions, SingleCharacterResult};

#[path="user_manager/mod.rs"]
mod user_manager;
use user_manager::{SendVoteTokenInputs, LoginInputs, LoginResults};

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

	/// 发送投票代码
	fn sendVoteCode(context: &Context, content: SendVoteTokenInputs) -> FieldResult<PostResult> {
		user_manager::sendVoteToken_impl(content)
	}

	/// 使用老帐号登录
	fn login(context: &Context, content: LoginInputs) -> FieldResult<LoginResults> {
		user_manager::login_impl(content)
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
