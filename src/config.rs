use super::features::step;
use serde::Deserialize;
use std::fs;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub name: String,
    pub steps: Vec<step::Step>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionGroup {
    pub name: String,
    pub actions: Vec<Action>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub icon_path: String,
    #[serde(default)]
    pub is_hidden: bool,
    #[serde(default)]
    pub actions: Vec<Action>,
    #[serde(default)]
    pub action_groups: Vec<ActionGroup>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Program {
    name: String,
    is_singleton: bool,
    bin_path: String,
    bin_name: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimePhase {
    #[allow(dead_code)]
    pub name: String,
    pub hour_start: u32,
    pub hour_end: u32,
    pub color: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClockConfig {
    pub default_color: String,
    pub phases: Vec<TimePhase>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    // version: String,
    #[serde(default)]
    pub is_forced_admin: bool,
    pub clock: ClockConfig,
    #[serde(default)]
    pub workspaces: Vec<Workspace>,
    #[serde(default)]
    pub scripts: Vec<Script>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Script {
    pub name: String,
    pub value: String,
}

pub fn load() -> Result<AppConfig, String> {
    // TODO: the file could also not exist, prompt for creation
    let config_string: String = fs::read_to_string("assets/config.yaml")
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    let config: AppConfig = serde_saphyr::from_str(&config_string[..])
        .map_err(|e| format!("Failed to parse config: {}", e))?;
    return Ok(config);
}
