use jwt_simple::prelude::ES256kPublicKey;


#[derive(Debug, Clone)]
pub struct Context {
    pub user_ip: String,
    pub additional_fingureprint: Option<String>,
    pub public_key: ES256kPublicKey
}

impl juniper::Context for Context {}
