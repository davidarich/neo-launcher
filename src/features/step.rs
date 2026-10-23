use crate::config;
use serde::Deserialize;
use std::fs;
use std::process::Command;
// see doc for generic nested structs so multiple step types can exist within a single step vector to provide a friendlier config
// https://stackoverflow.com/questions/74882738/how-do-add-generic-types-in-nested-structs-in-rust

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Step {
    #[serde(rename = "type")]
    #[allow(dead_code)] // deserialized from config; not needed at runtime yet
    pub step_type: String,

    #[serde(flatten)]
    pub specific_step: SpecificStep,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum SpecificStep {
    ExecStep {
        cmd: String,
        #[serde(default = "default_dir")]
        dir: String,
    },
    #[serde(rename_all = "camelCase", untagged)]
    ScriptStep { script_name: String },
    //PluginStep for handling external plugins
}

impl SpecificStep {
    fn execute(&self, config: config::AppConfig) -> bool {
        match self {
            SpecificStep::ExecStep { cmd, dir } => return exec_step(cmd, dir),
            SpecificStep::ScriptStep { script_name } => return script_step(config, script_name),
        };
    }
}

// sets default value when dir is not present in config.yaml
fn default_dir() -> String {
    return String::from("./");
}

fn exec_step(cmd: &String, dir: &String) -> bool {
    let _ = Command::new("cmd")
        .current_dir(dir)
        .args(["/C", cmd])
        .spawn()
        .expect("failed to start");
    return true;
}

fn script_step(config: config::AppConfig, script_name: &String) -> bool {
    // make sure the scripts directory exists before writing into it
    let dir = crate::data::scripts_dir().unwrap();
    if fs::create_dir_all(&dir).is_err() {
        return false;
    }

    // find script and write to filesystem
    for script in config.scripts {
        if script.name == script_name.to_string() {
            let script_path = crate::data::script_path(script_name.to_string());
            if crate::data::write_script(script_path.unwrap(), script.value).is_err() {
                return false;
            }
        }
    }

    // execute the script
    if Command::new("cmd")
        .current_dir(crate::data::scripts_dir().unwrap())
        .args(["/C", script_name])
        .spawn()
        .is_err()
    {
        return false;
    }

    return true;
}

pub fn perform_action(steps: Vec<Step>, config: config::AppConfig) {
    for step in steps {
        if !step.specific_step.execute(config.to_owned()) {
            // TODO: log why the step opted to return early
            // display a message to the user so they know the step failed
            return;
        }
    }
}
