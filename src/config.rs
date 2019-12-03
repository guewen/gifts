use serde::Deserialize;

use email::{EmailServer, EmailTemplate};
use pairs::Person;

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigFile {
    pub people: Vec<Person>,
    pub config: GeneralConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GeneralConfig {
    pub smtp: EmailServer,
    pub email: EmailTemplate,
}
