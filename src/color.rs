use reqwest::get;
use serde::Deserialize;
use std::collections::HashMap; // IMPROVE: Serialized size of HashMap vs BTreeMap vs patricia_tree

pub async fn fetch_language_colors() -> Result<HashMap<String, String>, Box<dyn std::error::Error>>
{
    let url = "https://github.com/github-linguist/linguist/raw/main/lib/linguist/languages.yml";
    let response = get(url).await?.text().await?;

    #[derive(Debug, Deserialize)]
    struct Language {
        color: Option<String>,
    }

    // Parse the YAML content
    let languages: HashMap<String, Language> = serde_yaml::from_str(&response)?;

    // Create a map of language to color code, filtering out languages without a color
    let mut language_colors = HashMap::new();
    for (lang, info) in languages.iter() {
        if let Some(color) = &info.color {
            language_colors.insert(lang.clone(), color.clone());
            language_colors.insert(lang.to_lowercase(), color.clone());
            language_colors.insert(lang.to_uppercase(), color.clone());
        }
    }

    Ok(language_colors)
}
