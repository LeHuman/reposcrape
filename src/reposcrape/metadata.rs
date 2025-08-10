use regex::Regex;
use reqwest::Client;
use std::collections::HashMap;
use tracing::{debug, warn};

pub fn extract_urls(input: &Vec<&str>) -> Vec<String> {
    let url_pattern = r"(http(s)?://.)?(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_\+.~#?&//= ]*)"; // FIXME: Include space character?
    let re = Regex::new(url_pattern).unwrap();
    let mut urls = Vec::new();

    for line in input {
        for mat in re.find_iter(line) {
            urls.push(mat.as_str().to_string());
        }
    }
    urls
}

pub async fn extract_resolved_urls(client: &Client, input: &Vec<&str>) -> Vec<String> {
    let urls = extract_urls(input);
    let mut result = Vec::new();

    for url in urls {
        if let Ok(resp) = client.get(url).send().await {
            let url = resp.url().as_str().to_string();
            if resp.error_for_status().is_ok() {
                result.push(url);
            }
        }
    }
    result
}

pub struct Metadata;

const URL_KEYWORDS: &[&str] = &["HIGHLIGHT", "LOGO", "DEMO"]; // IMPROVE: Generalize url captures

impl Metadata {
    pub async fn resolve_meta_urls(raw_url: &String, data: &mut HashMap<String, String>) {
        let client = reqwest::Client::new();
        let re = regex::Regex::new(r#"\[.*?\]\(<?(?P<path>.*?)>?(?:\s+\".\*?\")?\)"#).unwrap();

        for (k, v) in data {
            if !URL_KEYWORDS.contains(&k.to_uppercase().as_str()) {
                continue;
            }
            let val = match re.captures(v) {
                Some(m) => m.name("path").unwrap().as_str().to_string(), // TODO: unwrap
                None => v.to_string(),
            };
            // FIXME: Does this need more trimming?
            let x: &[_] = &['.', '/'];
            let new_path = raw_url.to_owned() + val.trim().trim_start_matches(x);
            let resolved =
                extract_resolved_urls(&client, &vec![val.as_str(), new_path.as_str()]).await;
            debug!("{:?}", resolved);
            if !resolved.is_empty() {
                *v = resolved.join("\n");
                // data.insert(k.to_string(), resolved[0].to_owned());
            }
        }
    }

    pub fn extract(text: &str) -> HashMap<String, String> {
        let re: Regex = Regex::new(r"(?i)^\s*<!--\s*((?P<key>\w*?):\s*(?P<val>.*?)|(?P<start>\w+\s*START)|(?P<end>\w+\s*END)|(?P<keyword>\w+?))\s*-\s*-\s*>").unwrap();
        let re_section: Regex = Regex::new(r"(?i)^(?P<name>.+?)\s*?(START|END)").unwrap();
        let mut map: HashMap<String, String> = HashMap::new();

        fn extract_metadata_section_name(re: &Regex, captured_line: &str) -> String {
            let full_line = captured_line.to_uppercase();
            let captures = re.captures(&full_line);

            if let Some(result) = captures {
                if result.name("name").is_some() {
                    let name = result
                        .name("name")
                        .expect("Failed to get section name")
                        .as_str()
                        .to_uppercase();
                    return name;
                }
            }

            String::new()
        }

        let mut section_accumulator = String::new();
        let mut section_name = String::new();
        let mut section_enable = false;
        let mut keyword_trigger = "";

        for line in text.lines() {
            if !keyword_trigger.is_empty() {
                let keyword = keyword_trigger.to_string();
                map.entry(keyword).or_insert_with(|| line.to_string());
                keyword_trigger = "";
            }
            let captures = re.captures(line);
            if let Some(result) = captures {
                if result.name("keyword").is_some() {
                    keyword_trigger = result
                        .name("keyword")
                        .expect("Failed to get result name")
                        .as_str();
                } else if result.name("start").is_some() {
                    if section_enable {
                        // TODO: warn on malformed section
                        // continue;
                    }
                    let name = result
                        .name("start")
                        .expect("Failed to get start name")
                        .as_str();

                    section_name = extract_metadata_section_name(&re_section, name);
                    section_accumulator = String::new();
                    section_enable = true;
                } else if result.name("end").is_some() {
                    let name = result.name("end").expect("Failed to get end name").as_str();
                    if (!section_enable)
                        || (extract_metadata_section_name(&re_section, name) != section_name)
                    {
                        warn!("Malformed section found {}", line);
                        continue;
                    }
                    section_accumulator.pop(); // Remove last ' '
                    map.insert(section_name.clone(), section_accumulator.clone());
                    section_enable = false;
                } else if result.name("key").is_some() && result.name("val").is_some() {
                    let key = result
                        .name("key")
                        .expect("Failed to get key")
                        .as_str()
                        .to_uppercase();
                    let val = result
                        .name("val")
                        .expect("Failed to get val")
                        .as_str()
                        .to_string(); // FIXME: Does this need to be uppercase?
                    map.insert(key, val);
                }
                // TODO: warn that capture detected but unable to parse
            } else if section_enable {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    section_accumulator.push_str(trimmed);
                    section_accumulator.push(' ');
                }
            }
        }

        if let Some(status) = map.get("STATUS") {
            let status = status.to_owned();
            let x: &[_] = &['*', '`'];
            let status = status.trim().trim_matches(x);
            map.insert("STATUS".to_string(), status.to_string());
        }

        map
    }
}
