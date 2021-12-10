
use std::str::FromStr;

use actix_web::client::Client;
use chrono::{DateTime, Utc};
use juniper::{FieldError, IntoFieldError, ScalarValue, graphql_value};
use jwt_simple::prelude::UnixTimeStamp;
use serde::{Deserialize, de::DeserializeOwned};
use serde_derive::{Serialize};
use thiserror::Error;

// pub const VOTE_START: DateTime<Utc> = DateTime::from_str("2021-10-01 00:00:00GMT+8").unwrap();
// pub const VOTE_END: DateTime<Utc> = DateTime::from_str("2021-10-15 00:00:00GMT+8").unwrap();

#[derive(Clone, Serialize, Deserialize)]
pub struct EmptyJSON {
	
}
impl EmptyJSON {
	pub fn new() -> EmptyJSON {
		EmptyJSON {  }
	}
}

#[derive(Deserialize)]
pub struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
	detail: Option<String>,
	sid: Option<String>,
	nickname: Option<String>
}

impl ErrorResponse {
	pub fn into_service_error(self) -> ServiceError {
		if let Some(sid) = self.sid {
			return ServiceError::RedirectToSignup { sid: sid, nickname: self.nickname };
		}
		match self.code {
			401 => {
				ServiceError::IncorrectLogin
			},
			422 => {
				ServiceError::InvalidContent
			},
			429 => {
				ServiceError::TooManyAttempts
			},
			403 => {
				ServiceError::Forbidden
			},
			_ => {
				ServiceError::Unknown
			}
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Error {
	pub msg: String,
	pub aux: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VoteTokenClaim {
	pub vote_id: Option<String>
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct PostResult {
	code: i32,
	message: String,
}

#[derive(Error, Debug)]
pub enum ServiceError {
	#[error("Invalid form content")]
	InvalidContent,
	#[error("Too many attempts")]
	TooManyAttempts,
	#[error("Forbidden")]
	Forbidden,
	#[error("Incorrect login")]
	IncorrectLogin,
	#[error("Redirect to signup")]
	RedirectToSignup { sid: String, nickname: Option<String> },
	#[error("Unknown Internal Error")]
	Unknown
}


impl<S: ScalarValue> IntoFieldError<S> for ServiceError {
	fn into_field_error(self) -> FieldError<S> {
		match self {
			ServiceError::InvalidContent => {
				FieldError::new(
					"InvalidContent",
					graphql_value!({
						"type": "InvalidContent"
					}),
				)
			},
			ServiceError::TooManyAttempts => {
				FieldError::new(
					"TooManyAttempts",
					graphql_value!({
						"type": "TooManyAttempts"
					}),
				)
			},
			ServiceError::Unknown => {
				FieldError::new(
					"Internal Server Error",
					graphql_value!({
						"type": "Unknown"
					}),
				)
			},
			ServiceError::Forbidden => {
				FieldError::new(
					"Operation Forbideen",
					graphql_value!({
						"type": "Forbidden"
					}),
				)
			},
			ServiceError::IncorrectLogin => FieldError::new(
				"Incorrect login",
				graphql_value!({
					"type": "IncorrectLogin"
				}),
			),
			ServiceError::RedirectToSignup { sid, nickname } => FieldError::new(
				"RedirectToSignup",
				graphql_value!({
					"type": "RedirectToSignup",
					"sid": sid,
					"nickname": nickname
				}),
			),
		}
	}
}

#[derive(juniper::GraphQLEnum, Clone, Serialize, Deserialize)]
pub enum VoteSection {
	Characters,
	Musics,
	CPs,
	Works,
	Papers
}

#[derive(juniper::GraphQLEnum, Clone, Serialize, Deserialize)]
pub enum FilterConditionOp {
	Equ,
	Neq,
	Gt,
	Gte,
	Lt,
	Lte,
	Contains
}

impl PostResult {
	pub fn new() -> PostResult {
		PostResult {
			code: 0,
			message: "none".to_string()
		}
	}
}

pub async fn getJSON<T: DeserializeOwned>(url: &str) -> Result<T, ServiceError> where T: DeserializeOwned {
	let client = reqwest::Client::new();
	let response = client.get(url)
		.send()
		.await;
	let response = match response {
		Ok(r) => r,
		Err(_) => { return Err(ServiceError::Unknown); }
	};
	if response.status().is_success() {
		let ret: T = match response.json().await {
			Ok(a) => a,
			Err(_) => { return Err(ServiceError::Unknown); }
		};
		Ok(ret)
	} else {
		let err: ErrorResponse = match response.json().await {
			Ok(a) => a,
			Err(_) => { return Err(ServiceError::Unknown); }
		};
		Err(err.into_service_error())
	}
}

pub async fn postJSON<T: DeserializeOwned, J: serde::ser::Serialize>(url: &str, obj: J) -> Result<T, ServiceError> {
	let client = reqwest::Client::new();
	let response = client.post(url)
		.json(&obj)
		.send()
		.await;
	let response = match response {
		Ok(r) => r,
		Err(e) => { println!("response error: {:?}", e); return Err(ServiceError::Unknown); }
	};
	let status = response.status();
	if status.is_success() {
		let ret: T = match response.json().await {
			Ok(a) => a,
			Err(e) => { println!("response status {}: {:?}", status, e); return Err(ServiceError::Unknown); }
		};
		Ok(ret)
	} else {
		let err: ErrorResponse = match response.json().await {
			Ok(a) => a,
			Err(e) => { println!("response error 2: {:?} {:?}", status, e); return Err(ServiceError::Unknown); }
		};
		Err(err.into_service_error())
	}
}
