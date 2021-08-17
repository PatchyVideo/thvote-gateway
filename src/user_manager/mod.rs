
use juniper::graphql_value;

use juniper::FieldResult;

use crate::common::{Error, PostResult};

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
#[graphql(description="Email login inputs for existing voters")]
pub struct EmailLoginInputsForExistingVoters {
    pub email: String,
    pub password: String
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="Email login up inputs")]
pub struct EmailLoginInputs {
    pub email: String,
    pub verify_code: String
}

#[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
#[graphql(description="Phone login inputs")]
pub struct PhoneLoginInputs {
    pub phone: String,
    pub verify_code: String
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
#[graphql(description="Login results")]
pub struct LoginResults {
    /// 投票token，登陆失败了就是错误返回，不会得到这个结构体
    pub vote_token: Option<String>
}

// ------------------------------------------------
// Root Quries
// ------------------------------------------------

use crate::services::*;


