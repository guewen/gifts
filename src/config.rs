use serde::{Deserialize, Serialize};

use email::{EmailServer, EmailTemplate};
use pairs::Group;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFile {
    pub groups: Vec<Group>,
    pub config: GeneralConfig,
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
