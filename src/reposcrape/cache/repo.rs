use localsavefile::{localsavefile, localsavefile_impl};
use savefile::prelude::Savefile;
use std::collections::{BTreeSet, HashMap};

use crate::{
    date::{Epoch, EpochType},
    reposcrape::Repo,
};

#[localsavefile]
#[derive(Eq, PartialEq)]
pub struct Cachable<T> {
    pub data: T,
    pub days_to_update: u32,
    pub last_update: EpochType,
}

impl<T> Cachable<T> {
    pub fn is_outdated(&self) -> bool {
        let local = Epoch::get_local();
        let millis = (self.days_to_update * 24 * 60 * 60 * 100).into();
        (self.last_update < local) && (local - self.last_update > millis)
    }
}

pub trait Update<T> {
    fn update(&mut self, other: &T);
}

#[localsavefile_impl]
#[derive(Eq, PartialEq, Savefile)]
pub struct RepoScrapeCache {
    pub repos: Cachable<BTreeSet<Repo>>,
    pub colors: Cachable<HashMap<String, String>>,
}

impl Default for RepoScrapeCache {
    fn default() -> Self {
        let mut ret = Self {
            repos: Default::default(),
            colors: Default::default(),
        };
        // NOTE: TTL is currently hardcoded for cache, must be manually deleted if updated
        ret.repos.days_to_update = 14;
        ret.colors.days_to_update = 60;
        ret
    }
}

impl RepoScrapeCache {
    pub fn new(
        repos: Option<Cachable<BTreeSet<Repo>>>,
        colors: Option<Cachable<HashMap<String, String>>>,
    ) -> Self {
        Self {
            repos: repos.unwrap_or_default(),
            colors: colors.unwrap_or_default(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.repos.data.is_empty()
    }
}

impl Update<BTreeSet<Repo>> for Cachable<BTreeSet<Repo>> {
    fn update(&mut self, other: &BTreeSet<Repo>) {
        // TODO: Test if extending on incoming repos changes anything or if persistance depends on Ord impl
        let mut other = other.clone();
        other.extend(self.data.clone());
        self.data = other;
        self.last_update = Epoch::get_local();
    }
}

impl Update<HashMap<String, String>> for Cachable<HashMap<String, String>> {
    fn update(&mut self, other: &HashMap<String, String>) {
        let mut other = other.to_owned();
        other.extend(self.data.clone());
        self.data = other;
    }
}
