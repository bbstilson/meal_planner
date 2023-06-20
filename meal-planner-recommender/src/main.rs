use anyhow::{Ok, Result};
use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use rand::Rng;
use std::collections::HashSet;

use common::*;

mod email_context;
mod env;
mod s3;
mod ses;
mod ses_util;
mod util;

async fn function_handler(_: LambdaEvent<CloudWatchEvent>) -> Result<()> {
    let aws_config = aws_config::load_from_env().await;
    let s3_client = s3::S3::init(&aws_config).await;
    let ses_client = ses::Ses::init(&aws_config).await;
    let ses_util = ses_util::EmailRenderer::init()?;
    let env = env::Env::from_env();

    let mut suggestion_history =
        util::get_suggestion_history(&env, &s3_client).await?;
    tracing::info!("fetched suggestion history");
    tracing::debug!("{suggestion_history:?}");

    let meals = get_meals(MealType::Dinner);
    tracing::info!("fetched recipies");

    // Check that all meals are in the suggestion history. Add missing meals, and
    // remove meals no longer listed.
    let suggestion_meals = suggestion_history
        .keys()
        .cloned()
        .collect::<HashSet<String>>();
    let meals_set = meals.keys().cloned().collect::<HashSet<String>>();

    let missing_meals =
        meals_set.difference(&suggestion_meals).collect::<Vec<_>>();
    let removed_meals =
        suggestion_meals.difference(&meals_set).collect::<Vec<_>>();

    for recipe in missing_meals {
        suggestion_history.insert(recipe.clone(), 0);
    }

    for recipe in removed_meals {
        suggestion_history.remove(recipe);
    }

    // Sort meals with random likeliness rating and take the top N.
    let mut rng = rand::thread_rng();
    let all_meals = Vec::from_iter(suggestion_history.clone());
    let mut meals = all_meals
        .into_iter()
        .map(|(recipe, count)| (recipe, ((count as f32) * rng.gen::<f32>())))
        .collect::<Vec<_>>();

    meals.sort_by(|(_, c1), (_, c2)| c1.partial_cmp(c2).unwrap());
    let recommended_meals = meals
        .into_iter()
        .take(3)
        .map(|(meal, _)| meal)
        .collect::<Vec<_>>();

    // Increment the count for the meals selected
    for meal in &recommended_meals {
        suggestion_history
            .entry(meal.to_string())
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    // Send an email.
    let body_context = email_context::EmailBodyContext {
        meals: recommended_meals,
        lambda_url: env.responder_url,
    }
    .prepare();
    ses_client
        .send_email(ses_util.mk_email_body(&body_context)?)
        .await?;

    // Serialize suggestion history and upload to S3.
    let new_suggestion_history_json =
        serde_json::to_string(&suggestion_history)
            .expect("Could not convert hashmap to json");

    s3_client
        .upload_object(
            &env.suggestion_history_bucket,
            &env.suggestion_history_key,
            &new_suggestion_history_json,
        )
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
