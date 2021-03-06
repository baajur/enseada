use enseada::couchdb::repository::Entity;
use events::Event;
use enseada::guid::Guid;

use crate::entity::Repo;

#[derive(Debug, Event)]
pub struct RepoCreated {
    pub id: Guid,
    pub rev: Option<String>,
    pub group: String,
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

impl From<&Repo> for RepoCreated {
    fn from(repo: &Repo) -> Self {
        Self {
            id: repo.id().clone(),
            rev: repo.rev().map(str::to_string),
            group: repo.group().to_string(),
            name: repo.name().to_string(),
            description: repo.description().map(str::to_string),
            tags: repo.tags().clone(),
        }
    }
}

#[derive(Debug, Event)]
pub struct RepoUpdated {
    pub id: Guid,
    pub rev: Option<String>,
    pub group: String,
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

impl From<&Repo> for RepoUpdated {
    fn from(repo: &Repo) -> Self {
        Self {
            id: repo.id().clone(),
            rev: repo.rev().map(str::to_string),
            group: repo.group().to_string(),
            name: repo.name().to_string(),
            description: repo.description().map(str::to_string),
            tags: repo.tags().clone(),
        }
    }
}

#[derive(Debug, Event)]
pub struct RepoDeleted {
    pub id: Guid,
    pub rev: Option<String>,
    pub group: String,
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

impl From<&Repo> for RepoDeleted {
    fn from(repo: &Repo) -> Self {
        Self {
            id: repo.id().clone(),
            rev: repo.rev().map(str::to_string),
            group: repo.group().to_string(),
            name: repo.name().to_string(),
            description: repo.description().map(str::to_string),
            tags: repo.tags().clone(),
        }
    }
}
