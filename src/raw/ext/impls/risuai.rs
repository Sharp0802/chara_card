use serde_json::Value;
use serdev::{Deserialize, Serialize};

/// **Not Enough Observed**
///
/// Represents RisuAI-specific extension.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RisuAI {
    #[serde(default)]
    pub bias: Vec<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_screen: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub utility_bot: Option<bool>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub sd_data: Vec<Vec<String>>,

    /// Represents HTML snippet to be injected to UI.
    #[serde(rename = "backgroundHTML")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_html: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_text: Option<String>,

    #[serde(rename = "virtualscript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_script: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_portrait: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lore_plus: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inlay_view_screen: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_gen_data: Option<NewGenData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vits: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_level_access: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_variables: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prebuilt_asset_command: Option<String>,

    #[serde(default)]
    pub prebuilt_asset_exclude: Vec<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prebuilt_asset_style: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewGenData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emotion_instructions: Option<String>,
}
