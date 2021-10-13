
use juniper::graphql_value;

use juniper::FieldResult;

use crate::common::postJSON;
use crate::common::{Error, PostResult};
use crate::context::Context;

use chrono::{DateTime, Utc};
use serde_derive::{Serialize, Deserialize};
use bson::oid::ObjectId;

// ------------------------------------------------
// REST Schemas
// ------------------------------------------------

#[derive(Clone, Serialize, Deserialize)]
pub struct UserEventMeta {
	pub user_ip: String,
	pub additional_fingureprint: Option<String>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SendPhoneVerifyCodeRequest {
	pub phone: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SendEmailVerifyCodeRequest {
	pub email: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EmailLoginInputsForExistingVoters {
	pub email: String,
	pub password: String,
	pub meta: UserEventMeta
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EmailLoginInputs {
	pub email: String,
	pub nickname: Option<String>,
	pub verify_code: String,
	pub meta: UserEventMeta
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PhoneLoginInputs {
	pub phone: String,
	pub nickname: Option<String>,
	pub verify_code: String,
	pub meta: UserEventMeta
}

// ------------------------------------------------
// GQL Schemas
// ------------------------------------------------

// #[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
// #[graphql(description="Email login inputs for existing voters")]
// pub struct EmailLoginInputsForExistingVotersGQL {
//     pub email: String,
//     pub password: String,
//     pub meta: UserEventMeta
// }

// #[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
// #[graphql(description="Email login up inputs")]
// pub struct EmailLoginInputsGQL {
//     pub email: String,
//     pub nickname: Option<String>,
//     pub verify_code: String,
//     pub meta: UserEventMeta
// }

// #[derive(juniper::GraphQLInputObject, Clone, Serialize, Deserialize)]
// #[graphql(description="Phone login inputs")]
// pub struct PhoneLoginInputsGQL {
//     pub phone: String,
//     pub nickname: Option<String>,
//     pub verify_code: String,
//     pub meta: UserEventMeta
// }

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
#[graphql(description="Login results")]
pub struct LoginResults {
	/// 投票token，登陆失败了就是错误返回，不会得到这个结构体
	pub vote_token: String
}

// ------------------------------------------------
// Root Quries
// ------------------------------------------------

use crate::services::*;

/// 老用户使用email帐号登录
pub async fn login_email_password(context: &Context, email: String, password: String) -> FieldResult<LoginResults> {
	let submit_json = EmailLoginInputsForExistingVoters {
		email: email,
		password: password,
		meta: UserEventMeta {
			user_ip: context.user_ip.clone(),
			additional_fingureprint: context.additional_fingureprint.clone()
		}
	};
	Ok(postJSON(&format!("http://{}/v1/login-email-password", USER_MANAGER), submit_json).await?)
}

/// 新用户使用email帐号登录
pub async fn login_email(context: &Context,  email: String, nickname: Option<String>, verify_code: String) -> FieldResult<LoginResults> {
	let submit_json = EmailLoginInputs {
		email: email,
		verify_code: verify_code,
		nickname: nickname,
		meta: UserEventMeta {
			user_ip: context.user_ip.clone(),
			additional_fingureprint: context.additional_fingureprint.clone()
		}
	};
	Ok(postJSON(&format!("http://{}/v1/login-email", USER_MANAGER), submit_json).await?)
}
/// 向邮箱发送验证码
pub async fn request_email_code(context: &Context, email: String) -> FieldResult<bool> {
	let submit_json = SendEmailVerifyCodeRequest {
		email: email
	};
	postJSON(&format!("http://{}/v1/send-email-code", USER_MANAGER), submit_json).await?;
	Ok(true)
}

/// 使用手机帐号登录
pub async fn login_phone(context: &Context, phone: String, nickname: Option<String>, verify_code: String) -> FieldResult<LoginResults> {
	let submit_json = PhoneLoginInputs {
		phone: phone,
		verify_code: verify_code,
		nickname: nickname,
		meta: UserEventMeta {
			user_ip: context.user_ip.clone(),
			additional_fingureprint: context.additional_fingureprint.clone()
		}
	};
	Ok(postJSON(&format!("http://{}/v1/login-phone", USER_MANAGER), submit_json).await?)
}
/// 向手机发送验证码
pub async fn request_phone_code(context: &Context, phone: String) -> FieldResult<bool> {
	let submit_json = SendPhoneVerifyCodeRequest {
		phone: phone
	};
	postJSON(&format!("http://{}/v1/send-sms-code", USER_MANAGER), submit_json).await?;
	Ok(true)
}

