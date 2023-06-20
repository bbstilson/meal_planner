use std::collections::{HashMap, HashSet};

use anyhow::Result;
use common::*;
use lambda_http::{
    run, service_fn, Body, Error, Request, RequestExt, Response,
};
use todoist::Todoist;

mod todoist;
mod util;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let api = Todoist::init("Automated Groceries").await?;
    let meals_param = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("meals"));

    if let Some(raw_meals_param) = meals_param {
        let requested_meals = raw_meals_param
            .split(",")
            .map(|s| parse_query_param(s).unwrap())
            .collect::<HashSet<_>>();

        tracing::info!("meals requested before filtering:");
        tracing::info!("{requested_meals:?}");

        let all_meals = get_meals(MealType::Dinner);
        let all_meal_names =
            all_meals.keys().cloned().collect::<HashSet<String>>();

        let valid_meal_names = all_meal_names
            .intersection(&requested_meals)
            .collect::<HashSet<_>>();

        let meals_to_plan = all_meals
            .into_iter()
            .filter(|(n, _)| valid_meal_names.contains(n))
            .collect::<HashMap<_, _>>();

        let aisle_config = get_aisle_conf()?;
        let categories = util::build_ingredients(meals_to_plan, &aisle_config)?;

        tracing::info!("adding ingredients for meals:");
        tracing::info!("{valid_meal_names:?}");
        api.create_ingredients(categories).await?;
        tracing::info!("done");
    } else {
        eprintln!("wahh");
    }

    let message = format!("OK");

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
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
