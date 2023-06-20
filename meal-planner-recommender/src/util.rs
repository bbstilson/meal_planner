use std::collections::HashMap;

use anyhow::Result;

use crate::{env, s3};

pub async fn get_suggestion_history(
    env: &env::Env,
    s3_client: &s3::S3,
) -> Result<HashMap<String, u32>> {
    let result = s3_client
        .get_object(&env.suggestion_history_bucket, &env.suggestion_history_key)
        .await?;

    let map = serde_json::from_str::<HashMap<String, u32>>(&result)?;

    Ok(map)
}
