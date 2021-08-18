
#[cfg(debug_assertions)]
pub const USER_MANAGER: &'static str = "127.0.0.1:1100";
#[cfg(debug_assertions)]
pub const SUBMIT_HANDLER: &'static str = "127.0.0.1:1101";
#[cfg(debug_assertions)]
pub const RESULT_QUERY: &'static str = "127.0.0.1:1102";
#[cfg(debug_assertions)]
pub const SUBMIT_VALIDATOR: &'static str = "127.0.0.1:1103";

#[cfg(not(debug_assertions))]
pub const USER_MANAGER: &'static str = "http://user_manager";
#[cfg(not(debug_assertions))]
pub const SUBMIT_HANDLER: &'static str = "http://submit_handler";
#[cfg(not(debug_assertions))]
pub const RESULT_QUERY: &'static str = "http://result_query";
#[cfg(not(debug_assertions))]
pub const SUBMIT_VALIDATOR: &'static str = "http://user_manager";

