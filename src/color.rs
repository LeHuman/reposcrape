use reqwest::get;
use serde::Deserialize;
use std::collections::HashMap;
use tracing::debug; // IMPROVE: Serialized size of HashMap vs BTreeMap vs patricia_tree

// IMPROVE: Fuzzy search of strings at usage
fn add_lang(map: &mut HashMap<String, String>, lang: String, color: String, is_alias: bool) {
    let lowercase = lang.to_lowercase();

    if is_alias && (map.contains_key(&lang) || map.contains_key(&lowercase)) {
        debug!("Ignore language alias, actual names take priority {}", lang);
        return;
    }

    if let Some(old_color) = map.insert(lang.to_owned(), color.clone()) {
        if color != old_color {
            debug!("Got new color for {}: {} -> {}", lang, old_color, color);
        }
    }

    if lowercase == lang {
        return;
    }

    if let Some(old_color) = map.insert(lowercase.to_owned(), color.clone()) {
        if color != old_color {
            debug!(
                "Got new color for {}: {} -> {}",
                lowercase, old_color, color
            );
        }
    }
}

pub async fn fetch_language_colors() -> Result<HashMap<String, String>, Box<dyn std::error::Error>>
{
    let url = "https://github.com/github-linguist/linguist/raw/main/lib/linguist/languages.yml";
    let response = get(url).await?.text().await?;

    #[derive(Debug, Deserialize)]
    struct Language {
        color: Option<String>,
        aliases: Option<Vec<String>>,
        extensions: Option<Vec<String>>,
    }

    // Parse the YAML content
    let languages: HashMap<String, Language> = serde_yaml::from_str(&response)?;

    // Create a map of language to color code, filtering out languages without a color
    let mut language_colors = HashMap::new();

    for (lang, info) in languages.iter() {
        let Some(color) = &info.color else {
            continue;
        };

        add_lang(&mut language_colors, lang.clone(), color.clone(), false);

        if let Some(extensions) = &info.extensions {
            for ext in extensions {
                add_lang(&mut language_colors, ext.clone(), color.clone(), true);
            }
        }

        if let Some(aliases) = &info.aliases {
            for alias in aliases {
                add_lang(&mut language_colors, alias.clone(), color.clone(), true);
            }
        }
    }

    Ok(language_colors)
}
