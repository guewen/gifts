use serde::{Deserialize, Serialize};

use email::{EmailServer, EmailTemplate};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFile {
    pub groups: Vec<Group>,
    pub config: GeneralConfig,
}

// People in a group cannot exchange gifts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub people: Vec<Person>,
}

impl Group {
    pub fn new(people: Vec<Person>) -> Self {
        Self { people }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub email: String,
    pub name: String,
}

impl Person {
    pub fn new(email: &str, name: &str) -> Self {
        Self {
            email: email.to_string(),
            name: name.to_string(),
        }
    }
}

impl ConfigFile {
    pub fn new(groups: Vec<Group>, config: GeneralConfig) -> Self {
        Self { groups, config }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub smtp: EmailServer,
    pub email: EmailTemplate,
}

impl GeneralConfig {
    pub fn new(smtp: EmailServer, email: EmailTemplate) -> GeneralConfig {
        GeneralConfig { smtp, email }
    }
}
