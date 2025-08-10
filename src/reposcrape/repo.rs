use localsavefile::localsavefile;
use set_field::SetField;
use show_option::ShowOption;
use std::{cmp::Ordering, collections::HashMap, fmt::Display};
use tracing::warn;

use crate::date::{Epoch, EpochType};

// TODO: map details to color codes if possible, look into phf crate for static maps

#[localsavefile]
#[derive(Eq, PartialEq, SetField, Clone, Debug, Hash)]
pub struct RepoDetails {
    pub project: Option<String>, // NOTE: Only attach project when the repository is part of a project
    pub main: Option<String>, // NOTE: Special option that defines this repo as the main for it's project, it's value does not matter but it should have a value
    pub title: Option<String>,
    pub font: Option<Vec<String>>,
    pub color: Option<Vec<u32>>,
    pub keywords: Option<Vec<String>>,
    pub languages: Option<Vec<String>>,
    pub technology: Option<Vec<String>>,
    pub children: Option<String>, // NOTE: Only relevant if this is a main repository for a project, ignored otherwise
    pub status: Option<String>,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub demo: Option<String>,
    pub highlight: Option<String>,
}

fn vec_str<T: Display>(vec: &Option<Vec<T>>) -> String {
    match vec {
        Some(vec) => vec
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<_>>()
            .join(", "),
        None => "None".to_string(),
    }
}

impl Display for RepoDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let project = self.project.show_or("None");
        let is_main = self.main.is_some();
        let title = self.title.show_or("None");
        let fonts = vec_str(&self.font);
        let colors = vec_str(&self.color);
        let keywords = vec_str(&self.keywords);
        let languages = vec_str(&self.languages);
        let technology = vec_str(&self.technology);
        let children = self.children.show_or("None");
        let status = self.status.show_or("None");
        let description = self.description.show_or("None");
        let logo = self.logo.show_or("None");
        let demo = self.demo.show_or("None");
        let highlight = self.highlight.show_or("None");

        write!(
            f,
            "
    project: {project}
    is_main: {is_main}
    title: {title}
    fonts: {fonts}
    colors: {colors}
    keywords: {keywords}
    languages: {languages}
    technology: {technology}
    children: {children}
    status: {status}
    description: {description}
    logo: {logo}
    demo: {demo}
    highlight: {highlight}"
        )
    }
}

impl RepoDetails {
    fn set(&mut self, key: &str, val: &str) -> bool {
        // IMPROVE: attempt to 'cast' directly to type, instead of just trial and error, will need custom macro or just use serde
        let nkey = key.to_lowercase();
        if self.set_field(&nkey, Some(String::from(val))) {
            return true;
        };

        let val_arr: Vec<String> = if val.contains(',') {
            val.split(',')
                .map(|s| s.trim()) // Trim whitespace from each substring
                .filter(|s| !s.is_empty()) // Filter out empty substrings
                .map(String::from)
                .collect()
        } else {
            vec![val.trim().to_string()]
        };
        if !val_arr.is_empty() && self.set_field(&nkey, Some(val_arr.clone())) {
            return true;
        };

        let val_ints: Vec<u32> = val_arr
            .into_iter()
            .filter_map(|s| match s.parse::<u32>() {
                Ok(val) => Some(val),
                Err(_) => {
                    // NOTE: Special Hex value case for Vec<u32>
                    u32::from_str_radix(
                        if let Some(stripped) = s.strip_prefix('#') {
                            stripped
                        } else {
                            &s
                        },
                        16,
                    )
                    .ok()
                }
            })
            .collect();
        if !val_ints.is_empty() && self.set_field(&nkey, Some(val_ints.clone())) {
            return true;
        };

        false
    }
}

#[localsavefile]
#[derive(Eq, Clone, Debug)]
pub struct Repo {
    pub uid: String,
    pub id: String,
    pub url: String,
    pub name: String,
    pub owner: String,
    pub origin: String,
    pub raw_url: String,
    pub last_sync: EpochType,
    pub last_update: EpochType,
    pub details: Option<RepoDetails>,
}

impl Display for Repo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} by {}\n  sync: {}\n  update: {}\n  URL: {}\n  origin: {}\n  uid: {}\n  details: {}",
            self.name,
            self.owner,
            Epoch::to_string(self.last_sync),
            Epoch::to_string(self.last_update),
            self.url,
            self.origin,
            self.uid,
            self.details.show_or("None")
        ))
    }
}

// TODO: Ensure comparing date strings works
impl Ord for Repo {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            self.last_sync.cmp(&other.last_sync)
        } else {
            self.last_update
                .cmp(&other.last_update)
                .then(self.uid.cmp(&other.uid))
        }
    }
}

impl PartialOrd for Repo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Repo {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
}

impl Repo {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: String,
        url: String,
        name: String,
        owner: String,
        origin: String,
        raw_url: String,
        last_sync: EpochType,
        last_update: EpochType,
        metadata: &HashMap<String, String>,
    ) -> Repo {
        let mut details = RepoDetails::default();
        let mut update: bool = false;
        for (key, val) in metadata {
            if !details.set(key, val) {
                warn!("Failed to set {} {}", key, val);
            } else {
                update = true;
            }
        }
        let mut uid = origin.to_owned();
        uid.push('/');
        uid.push_str(id.as_str());
        Repo {
            uid,
            id,
            url,
            name,
            owner,
            origin,
            raw_url,
            last_sync,
            last_update,
            details: if update { Some(details) } else { None },
        }
    }
}
