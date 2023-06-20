use std::env;

pub struct Env {
    pub responder_url: String,
    pub suggestion_history_bucket: String,
    pub suggestion_history_key: String,
}

impl Env {
    pub fn from_env() -> Self {
        Self {
            responder_url: env::var("RESPONDER_URL")
                .expect("RESPONDER_URL not set"),
            suggestion_history_bucket: env::var("SUGGESTION_HISTORY_BUCKET")
                .expect("SUGGESTION_HISTORY_BUCKET not set"),
            suggestion_history_key: env::var("SUGGESTION_HISTORY_KEY")
                .expect("SUGGESTION_HISTORY_KEY not set"),
        }
    }
}
