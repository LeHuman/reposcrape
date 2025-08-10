use show_option::ShowOption;

use super::Repo;

use std::{collections::BTreeSet, fmt::Display};

#[derive(Eq, PartialEq, Debug)]
pub struct Project {
    pub name: String,
    pub description: Option<String>,
    pub repo_main: Option<Repo>,
    pub repo_sub: BTreeSet<Repo>,
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}\n {}",
            self.name,
            self.description.show_or("No Description")
        )?;

        writeln!(f, "- Main Repo: {}", self.repo_main.show_or("None"))?;

        for repo in &self.repo_sub {
            writeln!(f, "- SubRepo: {}", repo)?;
        }
        Ok(())
    }
}

impl Project {
    pub fn get_single(mut self) -> Option<Repo> {
        let has_main = self.repo_main.is_some();
        if (has_main as usize + self.repo_sub.len()) == 1 {
            if has_main {
                self.repo_main
            } else {
                self.repo_sub.pop_last()
            }
        } else {
            None
        }
    }
    pub fn is_single(&self) -> bool {
        ((self.repo_main.is_some() as usize) + self.repo_sub.len()) == 1
    }
}
