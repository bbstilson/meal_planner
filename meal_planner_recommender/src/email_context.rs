use serde::Serialize;

use common::*;

#[derive(Debug)]
pub struct EmailBodyContext {
    pub meals: Vec<String>,
    pub lambda_url: String,
}

impl EmailBodyContext {
    pub fn prepare(self) -> PreparedEmailBodyContext {
        PreparedEmailBodyContext {
            meals: self.meals.clone(),
            responder_url: format!(
                "{}?meals={}",
                self.lambda_url,
                strs_to_query_params(self.meals)
            ),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PreparedEmailBodyContext {
    pub meals: Vec<String>,
    pub responder_url: String,
}
