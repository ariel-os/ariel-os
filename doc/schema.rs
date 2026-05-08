use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Matrix {
    pub support_keys: Vec<SupportKeyInfo>,
    pub functionalities: Vec<FunctionalityInfo>,
    pub note_snippets: Vec<NoteSnippets>,
    pub chips: HashMap<String, ChipInfo>,
    pub builders: HashMap<String, BuilderInfo>,
    pub boards: Vec<BoardInfo>,
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
    pub support_criteria: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NoteSnippets {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChipInfo {
    pub name: String,
    pub manufacturer: String,
    pub description: Option<String>,
    pub support: HashMap<String, SupportInfo>,
    pub notes: Option<String>,
    pub note_snippets: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuilderInfo {
    pub chip: String,
    pub tier: String,
    pub support: HashMap<String, SupportInfo>,
    pub notes: Option<String>,
    pub note_snippets: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoardInfo {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub builders: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
