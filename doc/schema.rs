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

pub enum Tier {
    Tier1,
    Tier2,
    Tier3,
}

impl argh::FromArgValue for Tier {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        match value {
            "tier-1" | "1" => Ok(Self::Tier1),
            "tier-2" | "2" => Ok(Self::Tier2),
            "tier-3" | "3" => Ok(Self::Tier3),
            _ => Err("invalid board support tier".to_string()),
        }
    }
}

impl ToString for Tier {
    fn to_string(&self) -> String {
        match self {
            Tier::Tier1 => "1".to_string(),
            Tier::Tier2 => "2".to_string(),
            Tier::Tier3 => "3".to_string(),
        }
    }
}
