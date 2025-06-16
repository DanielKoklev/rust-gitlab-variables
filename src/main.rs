use reqwest::header;
use serde::{Deserialize, Serialize};
use std::error::Error;
use clap::Parser;
use futures::future::BoxFuture;

#[derive(Parser)]
struct Options {
    #[clap(short = 't', long = "token")]
    gitlab_token: String,

    #[clap(short = 'v', long = "variable")]
    pipeline_variable: String,

    #[clap(short, long)]
    new_value: String,
}

#[derive(Debug, Deserialize)]
struct Project {
    id: u64,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Group {
    id: u64,
    name: String,
    full_path: String,
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
    let passed_variable = &options.pipeline_variable;
    let passed_new_value = &options.new_value;

    let gitlab_url = "https://gitlab.com/api/v4";

    let client = reqwest::Client::new();

    // Fetch groups
    let groups_response = client
        .get(format!("{}/groups?membership=true", gitlab_url))
        .bearer_auth(token)
        .send()
        .await?;

    let groups: Vec<Group> = groups_response.json().await?;
    println!("Groups accessible with the provided token:");
    for group in &groups {
        println!("ID: {}, Name: {}, Full Path: {}", group.id, group.name, group.full_path);

        let group_projects = fetch_all_projects_in_group_and_subgroups(&client, gitlab_url, token, group.id).await?;


        for project in group_projects {
            println!("Group Project - ID: {}, Name: {}", project.id, project.name);
            if let Err(e) = update_pipeline_variable(&client, gitlab_url, token, project.id, passed_variable, passed_new_value).await {
                println!("Error updating variable for project {}: {}", project.name, e);
            }
        }
    }

    Ok(())
}

async fn update_pipeline_variable(
    client: &reqwest::Client,
    gitlab_url: &str,
    token: &str,
    project_id: u64,
    variable_key: &str,
    variable_value: &str,
) -> Result<(), Box<dyn Error>> {
    let variable = PipelineVariable {
        key: variable_key.to_string(),
        value: variable_value.to_string(),
        variable_type: "env_var".to_string(),
    };

    // Check if the variable exists
    let check_response = client
        .get(format!("{}/projects/{}/variables/{}", gitlab_url, project_id, variable_key))
        .bearer_auth(token)
        .send()
        .await;

    match check_response {
        Ok(response) => {
            if response.status().is_success() {
                // Variable exists, attempt to update
                let update_response = client
                    .put(format!("{}/projects/{}/variables/{}", gitlab_url, project_id, variable_key))
                    .bearer_auth(token)
                    .header(header::CONTENT_TYPE, "application/json")
                    .json(&variable)
                    .send()
                    .await?;

                if update_response.status().is_success() {
                    println!("Updated variable {} for project ID: {}", variable_key, project_id);
                } else {
                    let error_details = update_response.text().await?;
                    println!("Failed to update variable {} for project ID: {}, \n{}", variable_key, project_id, error_details);
                }
            }
        }
        Err(e) => {
            println!("Error checking variable {} for project ID: {}, \n{}", variable_key, project_id, e);
        }
    }

    Ok(())
}

fn fetch_all_projects_in_group_and_subgroups<'a>(
    client: &'a reqwest::Client,
    gitlab_url: &'a str,
    token: &'a str,
    group_id: u64,
) -> BoxFuture<'a, Result<Vec<Project>, Box<dyn Error>>> {
    Box::pin(async move {
        let mut all_projects = Vec::new();

        // Fetch projects in the current group
        let url = format!(
            "{}/groups/{}/projects?per_page=100&with_shared=false&archived=false",
            gitlab_url, group_id
        );

        let response = client.get(&url)
            .bearer_auth(token)
            .send()
            .await?;

        let mut projects: Vec<Project> = response.json().await?;
        all_projects.append(&mut projects);

        // Fetch subgroups
        let subgroups_url = format!("{}/groups/{}/subgroups?per_page=100", gitlab_url, group_id);
        let subgroups_response = client.get(&subgroups_url)
            .bearer_auth(token)
            .send()
            .await?;

        let subgroups: Vec<Group> = subgroups_response.json().await?;
        for subgroup in subgroups {
            let subgroup_projects = fetch_all_projects_in_group_and_subgroups(
                client, gitlab_url, token, subgroup.id,
            ).await?;
            all_projects.extend(subgroup_projects);
        }

        Ok(all_projects)
    })
}
