use std::collections::HashMap;

use anyhow::{anyhow, Result};
use serde::Deserialize;

use crate::util::Category;

const API_BASE: &str = "https://api.todoist.com/rest/v2";

#[derive(Debug, Deserialize)]
struct Project {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Section {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Task {
    id: String,
}

pub struct Todoist {
    client: reqwest::Client,
    project_id: String,
}

impl Todoist {
    pub async fn init(project_name: &str) -> Result<Self> {
        let token: &'static str = env!("TODOIST_API_TOKEN");

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", token).parse().unwrap(),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        headers.insert(
            reqwest::header::ACCEPT,
            "application/json".parse().unwrap(),
        );

        // TODO: X-Request-Id: uuidv4
        // This is not required but can be useful for implementation of request retry
        // logic. This header value should not exceed 36 bytes. We will be generating
        // them with uuidgen in the shell code examples.

        let client = reqwest::Client::builder()
            .user_agent("github@bbstilson")
            .default_headers(headers)
            .build()?;

        let all_projects = Todoist::get_projects(&client).await?;

        let filtered_projects = all_projects
            .into_iter()
            .filter(|p| p.name == project_name)
            .collect::<Vec<_>>();

        match filtered_projects.first() {
            Some(project) => Ok(Self {
                client,
                project_id: project.id.clone(),
            }),
            None => {
                Err(anyhow!(format!("Project not found: {}", project_name)))
            }
        }
    }

    pub async fn create_ingredients<'a>(
        &self,
        categories: Vec<Category<'a>>,
    ) -> Result<()> {
        let sections = self.get_all_sections().await?;
        let section_to_id = sections
            .into_iter()
            .map(|s| (s.name, s.id))
            .collect::<HashMap<_, _>>();

        let all_categories = categories
            .iter()
            .map(|c| c.category.to_string())
            .collect::<Vec<_>>();

        let mut category_to_section_id = HashMap::<String, String>::new();

        // pair categories to section ids
        // if any categories do not have a section, create that section
        for category in &all_categories {
            let section_id = match section_to_id.get(category) {
                Some(id) => id.clone(),
                None => {
                    println!("creating section: {category:}");
                    self.create_section(&category).await?
                }
            };
            category_to_section_id.insert(category.clone(), section_id);
        }

        // insert a task for each ingredient in the correct section and project
        for category in categories {
            for item in category.items {
                // TODO: parallelize this
                let task_id = self
                    .create_task(
                        &category_to_section_id[category.category],
                        &format!("{item:}"),
                    )
                    .await?;
                println!("created task: {task_id}");
            }
        }

        Ok(())
    }

    async fn get_projects(client: &reqwest::Client) -> Result<Vec<Project>> {
        // https://developer.todoist.com/rest/v2/#get-all-projects

        let r = client
            .get(format!("{}/projects", API_BASE))
            .send()
            .await?
            .json::<Vec<Project>>()
            .await?;

        Ok(r)
    }

    async fn get_all_sections(&self) -> Result<Vec<Section>> {
        // https://developer.todoist.com/rest/v2/#get-all-sections

        let r = self
            .client
            .get(format!(
                "{}/sections?project_id={}",
                API_BASE, self.project_id
            ))
            .send()
            .await?
            .json::<Vec<Section>>()
            .await?;

        Ok(r)
    }

    async fn create_section(&self, section_name: &str) -> Result<String> {
        // https://developer.todoist.com/rest/v2/#create-a-new-section

        let section = self
            .client
            .post(format!("{}/sections", API_BASE))
            .json(&HashMap::from([
                ("project_id", self.project_id.to_string()),
                ("name", section_name.to_string()),
            ]))
            .send()
            .await?
            .json::<Section>()
            .await?;

        Ok(section.id)
    }

    async fn create_task(
        &self,
        section_id: &str,
        content: &str,
    ) -> Result<String> {
        // https://developer.todoist.com/rest/v2/#create-a-new-task
        let request: HashMap<&str, &str> = HashMap::from([
            ("project_id", self.project_id.as_str()),
            ("section_id", section_id),
            ("content", content),
        ]);
        let task = self
            .client
            .post(format!("{}/tasks", API_BASE))
            .json(&request)
            .send()
            .await?
            .json::<Task>()
            .await?;

        Ok(task.id)
    }
}
