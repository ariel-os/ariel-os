use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Matrix {
    pub support_keys: Vec<SupportKeyInfo>,
    pub functionalities: Vec<FunctionalityInfo>,
    pub chips: HashMap<String, ChipInfo>,
    pub boards: HashMap<String, BoardInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SupportKeyInfo {
    pub name: String,
    pub icon: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FunctionalityInfo {
    pub name: String,
    pub title: String, // FIXME: rename this
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChipInfo {
    pub name: String,
    pub description: Option<String>,
    pub support: HashMap<String, SupportInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoardInfo {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub chip: String,
    pub tier: String,
    pub support: HashMap<String, SupportInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum SupportInfo {
    StatusOnly(String),
    Details {
        status: String,
        comments: Option<Vec<String>>,
        link: Option<String>,
    },
}

impl SupportInfo {
    pub fn status(&self) -> &str {
        match self {
            SupportInfo::StatusOnly(status) => status,
            SupportInfo::Details { status, .. } => status,
        }
    }
}
