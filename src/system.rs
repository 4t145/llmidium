use std::sync::Arc;

use mcp_core::{
    Content, Resource, Tool, ToolError,
    handler::{PromptError, ResourceError},
    prompt::Prompt,
    protocol::{PromptsCapability, ResourcesCapability, ServerCapabilities, ToolsCapability},
    toolset::ToolSet,
};
use mcp_server::Router;
use serde_json::Value;
pub mod process;
pub mod fs;
#[derive(Debug, Clone)]
pub struct SystemRouter {
    tool_set: Arc<ToolSet>,
}

impl SystemRouter {
    pub fn enable_all() -> Self {
        let mut tool_set = ToolSet::default();
        tool_set.add_tool(process::Process::default());
        tool_set.extend(fs::toolset());
        SystemRouter { tool_set: tool_set.into() }
    }
}

impl Router for SystemRouter {
    fn name(&self) -> String {
        "system".to_string()
    }

    fn instructions(&self) -> String {
        "System tools".to_string()
    }

    fn capabilities(&self) -> ServerCapabilities {
        ServerCapabilities {
            prompts: Some(PromptsCapability {
                list_changed: Some(true),
            }),
            resources: Some(ResourcesCapability {
                subscribe: Some(true),
                list_changed: Some(true),
            }),
            tools: Some(ToolsCapability {
                list_changed: Some(true),
            }),
        }
    }

    fn list_tools(&self) -> Vec<Tool> {
        self.tool_set.list_all()
    }

    async fn call_tool(
        &self,
        tool_name: &str,
        arguments: Value,
    ) -> Result<Vec<Content>, ToolError> {
        self.tool_set.call(tool_name, arguments).await
    }

    fn list_resources(&self) -> Vec<Resource> {
        vec![]
    }

    async fn read_resource(&self, uri: &str) -> Result<String, ResourceError> {
        Err(ResourceError::NotFound("no resource".into()))
    }

    fn list_prompts(&self) -> Vec<Prompt> {
        vec![]
    }

    async fn get_prompt(&self, prompt_name: &str) -> Result<String, PromptError> {
        Err(PromptError::NotFound("no prompt".into()))
    }
}
