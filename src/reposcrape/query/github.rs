use std::collections::BTreeSet;

use octocrab::Octocrab;
use regex::Regex;
use secrecy::SecretString;
use tracing::debug;

use crate::{
    date::{Epoch, EpochType},
    reposcrape::{Metadata, Repo},
};

use super::query_trait::{QueryInterface, QueryResult, QueryResultSingle};

pub struct GHQuery {
    pub octocrab: Octocrab,
}

pub const ORIGIN: &str = "GitHub";
pub const RAW_URL: &str = "https://raw.githubusercontent.com/{user}/{repo}/{branch}/";

impl GHQuery {
    pub fn new(instance: Octocrab) -> Self {
        Self { octocrab: instance }
    }
    pub fn from_user_access_token<S: Into<SecretString>>(token: S) -> Self {
        let octocrab = Octocrab::builder().user_access_token(token);
        GHQuery {
            octocrab: octocrab.build().unwrap_or_default(),
        }
    }
    pub fn from_personal_token<S: Into<SecretString>>(token: S) -> Self {
        let octocrab = Octocrab::builder().personal_token(token);
        GHQuery {
            octocrab: octocrab.build().unwrap_or_default(),
        }
    }
    pub fn from_basic_auth(username: String, password: String) -> Self {
        let octocrab = Octocrab::builder().basic_auth(username, password);
        GHQuery {
            octocrab: octocrab.build().unwrap_or_default(),
        }
    }

    async fn process_repository_node(
        repo_val: &serde_json::Value,
        today_epoch: EpochType,
    ) -> Option<Repo> {
        let id = repo_val["id"].as_str()?.to_owned();
        let url = repo_val["url"].as_str()?.to_owned();
        let name: String = repo_val["name"].as_str()?.to_owned();
        let updated_at = repo_val["updatedAt"].as_str()?.to_owned();
        let updated_at = match Epoch::from_rfc3339(&updated_at) {
            Ok(e) => e,
            Err(_) => {
                println!("Failed to parse repo update time");
                0
            }
        };
        let owner = repo_val["owner"].as_object()?["login"].as_str()?.to_owned();
        let branch = repo_val["defaultBranchRef"].as_object()?["name"]
            .as_str()?
            .to_owned();
        let readme_text = repo_val["object"].as_object()?["text"].as_str()?; // NOTE: fn ignores repositories with no README.md
        let mut metadata = Metadata::extract(readme_text);

        let mut raw_url = RAW_URL.to_owned();
        raw_url = raw_url.replace("{user}", &owner);
        raw_url = raw_url.replace("{repo}", &name);
        raw_url = raw_url.replace("{branch}", &branch);

        Metadata::resolve_meta_urls(&raw_url, &mut metadata).await;

        Some(Repo::new(
            id,
            url,
            name,
            owner,
            ORIGIN.to_owned(),
            raw_url,
            today_epoch,
            updated_at,
            &metadata,
        ))
    }

    async fn call_single_query(&self, query: &serde_json::Value) -> QueryResultSingle {
        let response: serde_json::Value = self.octocrab.graphql(query).await?;

        let Some(repository) = drill_response(&response, String::from("repository")) else {
            return Err(Box::from("Failed to obtain single repo"));
        };
        let repository = Self::process_repository_node(repository, Epoch::get_local()).await;

        match repository {
            Some(r) => Ok(r),
            None => Err(Box::from("Failed to parse single repo")),
        }
    }

    async fn call_node_query(&self, query: &serde_json::Value) -> QueryResult {
        let response: serde_json::Value = self.octocrab.graphql(query).await?;

        let response_nodes = match match drill_response(&response, String::from("nodes")) {
            Some(val) => val.as_array(),
            None => return Err(Box::from("Failed to find nodes from query response")),
        } {
            Some(arr) => arr,
            None => {
                return Err(Box::from(
                    "Failed to access nodes as array from query response",
                ))
            }
        };

        let now_epoch = Epoch::get_local();

        let mut node_process = Vec::new();

        for repo_node in response_nodes {
            node_process.push(Self::process_repository_node(repo_node, now_epoch));
        }

        let mut result: BTreeSet<Repo> = BTreeSet::new();

        for node in node_process {
            match node.await {
                Some(repo) => result.insert(repo),
                None => continue,
            };
        }

        Ok(result)
    }
}

fn form_qstr(raw_query: String) -> serde_json::Value {
    serde_json::json!({ "query": raw_query })
}

fn qstr_single(username: &str, repository: &str) -> String {
    format!(
        r#"query {{
            repository(owner: "{username}", name: "{repository}") {{
                id
                url
                name
                updatedAt
                owner{{login}}
                defaultBranchRef {{
                    name
                }}
                object(expression: "HEAD:README.md") {{
                    ... on Blob {{
                        text
                    }}
                }}
            }}
        }}"#
    )
}

fn qstr_latest(username: &str, max_count: u32) -> String {
    format!(
        r#"query {{
    user(login: "{username}") {{
        repositories(first: {max_count}, orderBy: {{ field: UPDATED_AT, direction: DESC }}) {{
            nodes {{
                id
                url
                name
                updatedAt
                owner{{login}}
                defaultBranchRef {{
                    name
                }}
                object(expression: "HEAD:README.md") {{
                    ... on Blob {{
                        text
                    }}
                }}
            }}
        }}
    }}
}}"#
    )
}

fn qstr_dated(username: &str, max_count: u32, after_epoch: EpochType) -> String {
    let local_date = match Epoch::to_rfc3339(after_epoch) {
        Some(s) => s,
        None => {
            println!("Failed to parse rfc3339 from epoch, defaulting to zero");
            "1970-01-01T00:00:00Z".to_owned()
        }
    };
    format!(
        r#"query {{
search(type: REPOSITORY, first: {max_count}, query: "user:{username} pushed:>{local_date}") {{
    nodes {{
    ... on Repository {{
        id
        url
        name
        updatedAt
        owner{{login}}
        defaultBranchRef {{
            name
        }}
        object(expression: "HEAD:README.md") {{
        ... on Blob {{
            text
                }}
            }}
        }}
    }}
}}
}}"#
    )
}

fn drill_response(json_value: &serde_json::Value, key: String) -> Option<&serde_json::Value> {
    match json_value {
        serde_json::Value::Object(map) => match map.get(&key) {
            Some(nodes) => return Some(nodes),
            None => {
                for (_, value) in map.iter() {
                    if let Some(result) = drill_response(value, key.to_owned()) {
                        return Some(result);
                    }
                }
            }
        },
        serde_json::Value::Array(array) => {
            // Traverse each element in the array recursively
            for element in array.iter() {
                if let Some(result) = drill_response(element, key.to_owned()) {
                    return Some(result);
                }
            }
        }
        _ => return None,
    }
    None
}

async fn resolve_url(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let resp = client.get(url).send().await?;
    let final_url = resp.url().as_str().to_string();
    Ok(final_url)
}

fn extract_user_repo(url: &str) -> Option<(String, String)> {
    let re = Regex::new(r"^(?:https://)?(?:www\.)?github\.com/([^/]+)/([^/]+)").ok()?;

    if let Some(caps) = re.captures(url) {
        let user = caps.get(1)?.as_str().to_string();
        let repo = caps.get(2)?.as_str().to_string();
        return Some((user, repo));
    }
    None
}

impl QueryInterface for GHQuery {
    async fn fetch_latest(&self, user: &str, max_count: u32) -> QueryResult {
        self.call_node_query(&form_qstr(qstr_latest(user, max_count)))
            .await
    }

    async fn fetch_after(&self, user: &str, max_count: u32, after_epoch: EpochType) -> QueryResult {
        self.call_node_query(&form_qstr(qstr_dated(user, max_count, after_epoch)))
            .await
    }

    async fn fetch_single(&self, url: &str) -> QueryResultSingle {
        match resolve_url(url).await {
            Ok(resolved_url) => match extract_user_repo(&resolved_url) {
                Some((user, repo)) => {
                    debug!(
                        "Resolved URL: {}, User: {}, Repo: {}",
                        resolved_url, user, repo
                    );
                    self.call_single_query(&form_qstr(qstr_single(&user, &repo)))
                        .await
                }
                None => Err(Box::from(format!(
                    "Failed to extract user and repo from URL: {resolved_url}"
                ))),
            },
            Err(err) => Err(Box::from(format!("Failed to resolve URL {url}: {err}"))),
        }
    }
}
