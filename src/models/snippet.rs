use serde_derive::Deserialize;
use std::fmt::Formatter;

#[derive(Deserialize, Debug, Clone)]
pub struct Snippet {
    pub description: String,
    pub command: String,
    pub tag: Option<Vec<String>>
}

impl std::fmt::Display for Snippet {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {}", self.command, self.description)
    }
}


