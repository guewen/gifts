use serde::{Deserialize, Serialize};

use email::{EmailServer, EmailTemplate};
use pairs::Person;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFile {
    pub people: Vec<Person>,
    pub config: GeneralConfig,
}

impl ConfigFile {
    pub fn new(people: Vec<Person>, config: GeneralConfig) -> ConfigFile {
        ConfigFile { people, config }
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
