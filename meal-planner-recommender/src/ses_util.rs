use anyhow::{anyhow, Result};
use serde::Serialize;
use tera::Context;

pub struct EmailRenderer {
    renderer: tera::Tera,
}

impl EmailRenderer {
    pub fn init() -> Result<Self> {
        let email = include_str!("templates/email.tera");

        let mut renderer = tera::Tera::default();
        renderer.add_raw_template("email", email)?;
        renderer.autoescape_on(vec![".tera", ".sql"]);

        Ok(Self { renderer })
    }
    pub fn mk_email_body<T: Serialize>(self, t: T) -> Result<String> {
        let context = Context::from_serialize(t)?;
        self.renderer
            .render("email", &context)
            .map_err(|e| anyhow!(e))
    }
}
