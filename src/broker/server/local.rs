use mcp_client::{client::DynMcpClient, ClientCapabilities, ClientInfo, McpClient, McpClientTrait, McpService, StdioTransport, Transport};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path, time::Duration};
#[derive(Debug, Serialize, Deserialize)]
pub struct Local {
    cmd: String,
    args: Vec<String>,
    envs: HashMap<String, String>,
    timeout: Duration,
    // nix_shell: String,
}
impl Local {
    pub async fn run(self, client_info: &ClientInfo) -> Result<DynMcpClient, std::io::Error> {
        // let args = ["-p", &self.nix_shell];
        let transport = StdioTransport::new(self.cmd, self.args, self.envs);
        let transport_handle = transport.start().await.map_err(std::io::Error::other)?;
        let service = McpService::with_timeout(transport_handle, self.timeout);
        let mut client = McpClient::new(service);
        let x = client.initialize(client_info.clone(), ClientCapabilities::default()).await.map_err(std::io::Error::other)?;
        Ok(DynMcpClient::new(client))
    }
}
