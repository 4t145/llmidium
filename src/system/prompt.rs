use mcp_core::prompt::Prompt;

use crate::embed;

pub fn get(prompt: &str) -> Option<String> {
    if prompt == "llmidium" {
        Some(embed!("prompts/operate-llmidium").into())
    } else {
        None
    }
}

pub fn prompts() -> Vec<Prompt> {
    vec![Prompt {
        name: "llmidium".to_string(),
        description: Some(embed!("prompts/operate-llmidium.desc").to_string()),
        arguments: None,
    }]
}