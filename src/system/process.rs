use std::collections::HashMap;

use mcp_core::{Content, handler::TypedToolHandler};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(Debug, Default)]
pub struct Process {}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ProcessCallParam {
    command: String,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    envs: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessCallOutput {
    status: Option<i32>,
    std_out: String,
    std_err: String,
}

impl ProcessCallOutput {
    pub fn from_std(output: std::process::Output) -> Self {
        Self {
            status: output.status.code(),
            std_out: String::from_utf8_lossy(&output.stdout).into_owned(),
            std_err: String::from_utf8_lossy(&output.stderr).into_owned(),
        }
    }
}

impl From<std::process::Output> for ProcessCallOutput {
    fn from(value: std::process::Output) -> Self {
        Self::from_std(value)
    }
}

impl TypedToolHandler for Process {
    type Params = ProcessCallParam;
    fn name(&self) -> &'static str {
        "process"
    }

    fn description(&self) -> &'static str {
        "execute a process, output[0]: status code, output[1]: stdout, output[2]: stderr"
    }

    async fn call(&self, params: Self::Params) -> mcp_core::ToolResult<Vec<Content>> {
        let output = tokio::process::Command::new(params.command)
            .args(params.args)
            .envs(params.envs)
            .output()
            .await
            .map_err(|e| mcp_core::ToolError::ExecutionError(e.to_string()))?;
        let output: ProcessCallOutput = output.into();
        Ok(vec![
            Content::text(
                output
                    .status
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or("".to_owned()),
            ),
            Content::text(output.std_out),
            Content::text(output.std_err),
        ])
    }
}
