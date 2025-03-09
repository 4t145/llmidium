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

use crate::embed;
pub mod fs;
pub mod process;
pub mod prompt;
pub mod broker;

#[derive(Debug, Clone)]
pub struct SystemRouter {
    tool_set: Arc<ToolSet>,
    resource: Arc<[Resource]>,
    prompt: Arc<[Prompt]>,
}

impl SystemRouter {
    pub fn enable_all() -> Self {
        let mut tool_set = ToolSet::default();
        tool_set.add_tool(process::Process::default());
        tool_set.extend(fs::toolset());
        SystemRouter {
            tool_set: tool_set.into(),
            resource: Arc::from(fs::resource_set()),
            prompt: prompt::prompts().into(),
        }
    }
}

impl Router for SystemRouter {
    fn name(&self) -> String {
        "llmidium".to_string()
    }

    fn instructions(&self) -> String {
        embed!("/instructions").to_owned()
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

    async fn list_tools(&self) -> Vec<Tool> {
        self.tool_set.list_all()
    }

    async fn call_tool(
        &self,
        tool_name: &str,
        arguments: Value,
    ) -> Result<Vec<Content>, ToolError> {
        self.tool_set.call(tool_name, arguments).await
    }

    async fn list_resources(&self) -> Vec<Resource> {
        self.resource.to_vec()
    }

    async fn read_resource(&self, uri: &str) -> Result<String, ResourceError> {
        if let Some(path) = uri.strip_prefix(fs::FS_RESOURCE) {
            return tokio::fs::read_to_string(path)
                .await
                .map_err(ResourceError::execution);
        }
        Err(ResourceError::NotFound("no such resource".into()))
    }

    async fn list_prompts(&self) -> Vec<Prompt> {
        self.prompt.to_vec()
    }

    async fn get_prompt(&self, prompt_name: &str) -> Result<String, PromptError> {
        prompt::get(prompt_name).ok_or(
            PromptError::NotFound(format!("{prompt_name} not found"))
        )
    }
}
