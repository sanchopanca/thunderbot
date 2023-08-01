use std::time::{Duration, Instant};

use dashmap::DashMap;
use lazy_static::lazy_static;
use uuid::Uuid;

lazy_static! {
    pub static ref TOKENS: DashMap<String, (u64, Instant)> = DashMap::new();
}

pub fn generate_token(user_id: u64) -> String {
    let token = Uuid::new_v4().to_string();
    TOKENS.insert(token.clone(), (user_id, Instant::now()));
    token
}

#[allow(dead_code)]
pub fn validate_token(token: &str) -> bool {
    match TOKENS.get(token) {
        Some(entry) => {
            let delta = Instant::now() - entry.value().1;
            delta < Duration::from_secs(900)
        }
        None => false,
    }
}
