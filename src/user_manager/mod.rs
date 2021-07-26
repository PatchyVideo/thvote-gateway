
use juniper::graphql_value;

use juniper::FieldResult;

use crate::common::{Error, UserVerifyResult, PostResult};

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;

// ------------------------------------------------
// REST Schemas
// ------------------------------------------------

#[derive(Clone, Serialize, Deserialize)]
pub struct UserCreateRest {
    pub email: String
}

// ------------------------------------------------
// GQL Schemas
// ------------------------------------------------

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="SendVoteCodeInputs")]
pub struct SendVoteTokenInputs {
    pub email: String
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="Login inputs")]
pub struct LoginInputs {
    pub email: String,
    pub password: String
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
#[graphql(description="Login results")]
pub struct LoginResults {
    pub succeed: bool,
    pub vote_token: String
}

// ------------------------------------------------
// Root Quries
// ------------------------------------------------

use crate::services::*;

pub fn sendVoteToken_impl(content: SendVoteTokenInputs) -> FieldResult<PostResult> {
	Ok(PostResult::new())
}

pub fn login_impl(content: LoginInputs) -> FieldResult<LoginResults> {
	Ok(LoginResults {
        succeed: true,
        vote_token: "thvote-2020-81180d4a1c77A7c3FDE".into()
    })
}

