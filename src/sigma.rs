use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SigmaRule {
    header: Header,
    // todo: detection and condition
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Header {
    pub title: String,
    pub description: Option<String>,
    pub id: Option<String>,
    pub status: Option<Status>,
    pub license: Option<String>,
    pub author: Option<String>,
    pub references: Option<Vec<String>>,
    pub date: Option<String>,
    pub modified: Option<String>,
    pub logsource: Option<LogSource>,
    pub falsepositives: Option<Vec<String>>,
    pub level: Option<Level>,
    pub fields: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Stable,
    Test,
    Experimental,
    Deprecated,
    Unsupported,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Level {
    Informational,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Related {
    relation: Option<Vec<RelatedItem>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Type {
    Derived,
    Obsoletes,
    Merged,
    Renamed,
    Similar,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RelatedItem {
    pub id: String,
    pub types: Type,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LogSource {
    pub category: Option<String>,
    pub product: Option<String>,
    pub service: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Detection {}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Condition {}
