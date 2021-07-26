
use actix_web::client::Client;
use juniper::{FieldError, IntoFieldError, ScalarValue, graphql_value};
use serde::{Deserialize, de::DeserializeOwned};
use serde_derive::{Serialize};
use thiserror::Error;

#[derive(Deserialize)]
pub struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}

impl ErrorResponse {
	pub fn into_service_error(&self) -> ServiceError {
		match self.code {
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

#[derive(Clone, Serialize, Deserialize)]
pub struct UserVerifyResult {
	pub user_email: Option<String>
}

#[derive(juniper::GraphQLObject, Clone, Serialize, Deserialize)]
pub struct PostResult {
	pub errno: i32
}

#[derive(Error, Debug)]
pub enum ServiceError {
	#[error("Invalid form content")]
	InvalidContent,
	#[error("Too many attempts")]
	TooManyAttempts,
	#[error("Forbidden")]
	Forbidden,
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
			errno: 0
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
