use std::collections::BTreeSet;

use crate::{
    date::Epoch,
    reposcrape::{
        cache::{Cachable, ExpandedRepoCache, RepoScrapeCache},
        Repo, RepoDetails,
    },
};

#[test]
#[tracing_test::traced_test]
pub fn test_expand_cache() -> Result<(), Box<dyn std::error::Error>> {
    let mut repos: BTreeSet<Repo> = BTreeSet::new();
    repos.insert(Repo {
        uid: "github/Username/Repo0".into(),
        id: "Username/Repo0".into(),
        url: "".into(),
        name: "Repo0".into(),
        owner: "Username0".into(),
        origin: "github".into(),
        raw_url: "".into(),
        last_sync: Epoch::from_rfc3339("2019-05-14T19:19:26Z")?,
        last_update: Epoch::from_rfc3339("2018-05-14T19:19:26Z")?,
        details: Some(RepoDetails {
            project: Some("Project0".into()),
            title: Some("Repo0-0".into()),
            font: Some(Vec::from(["Arial".into(), "Helvetica".into()])),
            color: Some(Vec::new()),
            keywords: Some(Vec::from(["key0".into(), "yek1".into()])),
            description: Some("Description0".into()),
            why: Some("why0".into()),
            languages: Some(Vec::from(["rust".into(), "C++".into()])),
            technology: Some(Vec::from(["GH Actions".into(), "https".into()])),
            status: Some("Work In Progress".into()),
            main: Some("".into()),
            children: None,
            logo: None,
            demo: None,
            highlight: None,
        }),
    });
    repos.insert(Repo {
        uid: "github/Username/Repo1".into(),
        id: "Username/Repo1".into(),
        url: "".into(),
        name: "Repo1".into(),
        owner: "Username1".into(),
        origin: "github".into(),
        raw_url: "".into(),
        last_sync: Epoch::from_rfc3339("2021-06-14T08:19:26Z")?,
        last_update: Epoch::from_rfc3339("2020-05-14T08:19:26Z")?,
        details: Some(RepoDetails {
            project: Some("Project0".into()),
            title: Some("Repo1-0".into()),
            font: Some(Vec::from(["Times new roman".into()])),
            color: Some(Vec::new()),
            keywords: Some(Vec::from(["key2".into(), "yek3".into()])),
            description: Some("Description1".into()),
            why: Some("why1".into()),
            languages: Some(Vec::from(["C++".into()])),
            technology: Some(Vec::from(["https".into()])),
            status: Some("Archive".into()),
            main: Some("this".into()),
            children: None,
            logo: None,
            demo: None,
            highlight: None,
        }),
    });
    repos.insert(Repo {
        uid: "github/Username/Repo2".into(),
        id: "Username/Repo2".into(),
        url: "".into(),
        name: "Repo2".into(),
        owner: "Username0".into(),
        origin: "github".into(),
        raw_url: "".into(),
        last_sync: Epoch::from_rfc3339("2021-06-14T08:19:26Z")?,
        last_update: Epoch::from_rfc3339("2020-10-12T08:19:26Z")?,
        details: Some(RepoDetails {
            project: Some("Project1".into()),
            title: Some("Repo2-1".into()),
            font: Some(Vec::from(["Arial".into()])),
            color: Some(Vec::new()),
            keywords: Some(Vec::from(["key0".into(), "yek3".into()])),
            description: Some("Description2 😎".into()),
            why: Some("why2 🙄".into()),
            languages: Some(Vec::from(["C".into()])),
            technology: Some(Vec::from(["tech2".into()])),
            status: Some("Work In Progress".into()),
            main: None,
            children: None,
            logo: None,
            demo: None,
            highlight: None,
        }),
    });

    let dummy_cache = RepoScrapeCache::new(
        Some(Cachable {
            data: repos,
            days_to_update: 0,
            last_update: 0,
        }),
        None,
    );

    let rt = tokio::runtime::Runtime::new().unwrap();
    let expanded_cache = rt.block_on(ExpandedRepoCache::new(dummy_cache));

    assert!(expanded_cache.repos.len() == 1);
    assert!(expanded_cache.projects.len() == 1);

    Ok(())
}
