use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ListAvailableModels {
    pub(crate) category: Option<String>,
}
