use reqwest::header;
use serde::{Deserialize, Serialize};
use std::error::Error;
use clap::Parser;

#[derive(Parser)]
struct Options {
   #[clap(short, long)]
   gitlab_token: String, 
}

#[derive(Debug, Deserialize)]
struct Project {
    id: u64,
    name: String,
    permissions: ProjectPermissions,
}

#[derive(Debug, Deserialize)]
struct ProjectPermissions {
    project_access: Option<ProjectAccess>,
}

#[derive(Debug, Deserialize)]
struct ProjectAccess {
    access_level: u8,
}

#[derive(Debug, Serialize)]
struct PipelineVariable {
    key: String,
    value: String,
    variable_type: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::parse(); 
    let token = &options.gitlab_token;
    let gitlab_url = "https://gitlab.com/api/v4";

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/projects?membership=true", gitlab_url))
        .bearer_auth(token)
        .send()
        .await?;

    let projects: Vec<Project> = response.json().await?;

    for project in projects {
        if let Some(access) = project.permissions.project_access {
            if access.access_level >= 40 { // Maintainer access level
                let variable = PipelineVariable {
                    key: "GITLAB_ACCESS_TOKEN".to_string(),
                    value: token.to_string(),
                    variable_type: "env_var".to_string(),
                };

                let update_response = client
                    .put(format!("{}/projects/{}/variables/GITLAB_ACCESS_TOKEN", gitlab_url, project.id))
                    .bearer_auth(token)
                    .header(header::CONTENT_TYPE, "application/json")
                    .json(&variable)
                    .send()
                    .await?;

                if update_response.status().is_success() {
                    println!("Updated GITLAB_ACCESS_TOKEN for project: {}", project.name);
                } else {
                    let error_details = update_response.text().await?;
                    println!("Failed to update GITLAB_ACCESS_TOKEN for project: {}, \n{}", project.name, error_details);
                }
                println!("Token: {}", &token);
            }
        }
    }

    Ok(())
}
