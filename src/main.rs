#![allow(non_snake_case)]
extern crate juniper;

use std::io::{self, Read};
use std::sync::Arc;


use actix_web::{App, Error, HttpMessage, HttpResponse, HttpServer, client::ClientBuilder, cookie, middleware, web};
use context::Context;
use juniper_actix::{
	graphiql_handler as gqli_handler, graphql_handler, playground_handler as play_handler,
};
use jwt_simple::prelude::{ES256kKeyPair, ES256kPublicKey};

#[macro_use]
mod common;
mod schema;
mod services;
mod context;

pub mod user_manager;
pub mod result_query;
pub mod submit_handler;
pub mod vote_data;

use crate::schema::{create_schema, Schema};

fn read_a_file(filename: &str) -> std::io::Result<Vec<u8>> {
	let mut file = std::fs::File::open(filename)?;

	let mut data = Vec::new();
	file.read_to_end(&mut data)?;

	return Ok(data);
}

async fn graphiql_handler() -> Result<HttpResponse, Error> {
	gqli_handler("/graphql", None).await
}
async fn playground_handler() -> Result<HttpResponse, Error> {
	play_handler("/graphql", None).await
}
async fn graphql(
	req: actix_web::HttpRequest,
	payload: actix_web::web::Payload,
	schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
	//let vote_token = req.cookie("vote_token").map(|f| f.value().to_string());
	let ctx = Context {
		//vote_token: vote_token,
		additional_fingureprint: None,
		// TODO: additional fingerprint
		user_ip: req.connection_info().realip_remote_addr().unwrap_or("unknown").to_string(),
		public_key: ES256kPublicKey::from_pem(std::str::from_utf8(&read_a_file("../keys/key-pub.pem").unwrap()).unwrap()).unwrap()
	};
	graphql_handler(&schema, &ctx, req, payload).await
}

#[actix_web::main]
async fn main() -> io::Result<()> {
	std::env::set_var("RUST_LOG", "actix_web=info");
	env_logger::init();

	// Start http server
	HttpServer::new(move || {
		App::new()
			.data(create_schema())
			// .wrap(
			// 	Cors::new() // <- Construct CORS middleware builder
			// 		.allowed_origin("http://localhost:3000")
			// 		.allowed_origin("http://127.0.0.1:3000")
			// 		.allowed_origin("http://localhost:8080")
			// 		.allowed_origin("http://127.0.0.1:8080")
			// 		.max_age(3600)
			// 		.finish()
			// )
			.wrap(middleware::Logger::default())
			.service(web::resource("/graphql").route(web::post().to(graphql)))
			.service(web::resource("/graphiql").route(web::get().to(graphiql_handler)))
	})
	.bind("0.0.0.0:80")?
	.run()
	.await
}