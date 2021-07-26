
#[derive(Debug, Clone)]
pub struct Context {
    pub vote_token: Option<String>,
    pub user_ip: String
}

impl juniper::Context for Context {}
