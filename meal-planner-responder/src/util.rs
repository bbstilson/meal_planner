use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
};

use anyhow::Result;
use cooklang::{
    self,
    aisle::AileConf,
    convert::Converter,
    model::IngredientListEntry,
    quantity::{GroupedQuantity, TotalQuantity},
};
use include_dir::File;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Ingredient {
    pub name: String,
    pub quantity: TotalQuantity,
}
impl From<(String, GroupedQuantity)> for Ingredient {
    fn from((name, qty): (String, GroupedQuantity)) -> Self {
        Ingredient {
            name,
            quantity: qty.total().into(),
        }
    }
}

impl Display for Ingredient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let amt = match &self.quantity {
            TotalQuantity::Single(q) => format!("{q} - "),
            TotalQuantity::None => "".to_string(),
            TotalQuantity::Many(qs) => {
                // TODO: figure out how to resolve this. we just take the first one
                eprintln!("{qs:?}");
                format!("{} - ", qs.first().unwrap())
            }
        };
        write!(f, "{}{}", amt, self.name)
    }
}

#[derive(Debug, Serialize)]
pub struct Category<'a> {
    pub category: &'a str,
    pub items: Vec<Ingredient>,
}

pub fn build_ingredients<'a>(
    recipes: HashMap<String, File<'a>>,
    aisle_conf: &'a AileConf<'a>,
) -> Result<Vec<Category<'a>>> {
    let mut all_ingredients: BTreeMap<String, GroupedQuantity> =
        BTreeMap::new();
    let converter = Converter::default();
    for (recipe_name, file) in recipes {
        let (raw_recipe, _) =
            cooklang::parse(file.contents_utf8().unwrap(), &recipe_name)
                .into_result()?;

        let recipe = raw_recipe.default_scale();
        let ingredients = recipe.ingredient_list(&converter);

        for ingredient in ingredients {
            let IngredientListEntry {
                index, quantity, ..
            } = ingredient;
            let ingredient = &recipe.ingredients[index];
            all_ingredients
                .entry(ingredient.display_name().into_owned())
                .or_default()
                .merge(&quantity, &converter);
        }
    }

    let split_ingredients = split_into_categories(all_ingredients, &aisle_conf)
        .into_iter()
        .map(|(category, items)| Category {
            category,
            items: items.into_iter().map(Ingredient::from).collect(),
        })
        .collect::<Vec<_>>();

    Ok(split_ingredients)
}

pub fn split_into_categories<'a>(
    all_ingredients: BTreeMap<String, GroupedQuantity>,
    aisle_conf: &'a AileConf<'a>,
) -> Vec<(&'a str, Vec<(String, GroupedQuantity)>)> {
    let aisle = aisle_conf.reverse();
    let mut m = BTreeMap::<&str, Vec<_>>::new();
    let mut other = Vec::new();
    for (igr, q) in all_ingredients {
        if let Some(cat) = aisle.get(igr.as_str()) {
            m.entry(cat).or_default().push((igr, q))
        } else {
            other.push((igr, q));
        }
    }

    m.into_iter()
        .map(|(cat, items)| (cat, items))
        .chain(std::iter::once(("other", other)))
        .collect()
}
