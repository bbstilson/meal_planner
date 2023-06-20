use std::collections::HashMap;

use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use cooklang::aisle::AileConf;
use include_dir::{include_dir, Dir, File};

#[derive(Debug)]
pub enum MealType {
    Dinner,
}

impl MealType {
    pub fn to_string(&self) -> String {
        match self {
            MealType::Dinner => "Dinner".to_string(),
        }
    }
}

static MEALS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../meals");

pub fn get_aisle_conf<'a>() -> Result<AileConf<'a>> {
    // Assumed an aisle conf actually exists.
    let aisle_config_file = MEALS_DIR
        .find("config/aisle.conf")
        .unwrap()
        .filter_map(|f| f.as_file())
        .filter_map(|f| f.contents_utf8());

    if let Some(file_contents) = aisle_config_file.collect::<Vec<_>>().first() {
        match cooklang::aisle::parse(file_contents) {
            Ok(parsed) => Ok(parsed),
            Err(e) => {
                eprintln!("{e:}");
                Err(anyhow!("issue parsing aisle conf"))
            }
        }
    } else {
        Err(anyhow!("aisle conf file not found"))
    }
}
pub fn get_meals<'a>(meal_type: MealType) -> HashMap<String, File<'a>> {
    let meal_type_path = meal_type.to_string();
    MEALS_DIR
        .find("**/*.cook")
        .unwrap()
        .filter_map(|e| e.as_file())
        .filter(|f| f.path().starts_with(&meal_type_path))
        .map(|p| p.to_owned())
        .map(|f| (get_name_of_file(&f), f))
        .collect()
}

fn get_name_of_file<'a>(f: &File<'a>) -> String {
    f.path().file_stem().unwrap().to_str().unwrap().to_string()
}

pub fn strs_to_query_params(strs: Vec<String>) -> String {
    strs.iter()
        .map(|meal| str_to_query_param(meal))
        .collect::<Vec<_>>()
        .join(",")
}

pub fn str_to_query_param(s: &str) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(s)
}

pub fn parse_query_param(qp: &str) -> Result<String> {
    let decoded = general_purpose::URL_SAFE_NO_PAD.decode(qp)?;
    Ok(String::from_utf8(decoded)?)
}

#[test]
fn test_get_meals() {
    assert!(get_meals(MealType::Dinner).len() > 0)
}

#[test]
fn test_strs_to_query_params() {
    assert_eq!(
        "YQ,Yg,Y2QgZQ",
        strs_to_query_params(vec![
            "a".to_string(),
            "b".to_string(),
            "cd e".to_string()
        ])
    )
}

#[test]
fn test_roundtrip() {
    assert_eq!(
        "hey hi& hello",
        parse_query_param(&str_to_query_param("hey hi& hello")).unwrap()
    )
}
