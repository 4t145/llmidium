use mcp_client::{McpClient, McpService, StdioTransport, Transport};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path, time::Duration};
#[derive(Debug, Serialize, Deserialize)]
pub struct Local {
    cmd: String,
    args: Vec<String>,
    envs: HashMap<String, String>,
    nix_shell: String,
}

// impl Local {
//     pub async fn run(self) -> Result<(), std::io::Error> {
//         use tokio::*;
//         let process = process::Command::new("nix-shell")
//             .arg("-p")
//             .arg(self.nix_shell)
//             .arg("--run")
//             .arg(self.cmd)
//             .spawn()?;
//         let args = vec!["-p".to_string(), self.nix_shell];
//         let transport = StdioTransport::new(self.cmd, self.args, self.envs);
//         let transport_handle = transport.start().await.map_err(std::io::Error::other)?;
//         let service = McpService::with_timeout(transport_handle, Duration::from_secs(10));
//         let client = McpClient::new(service);
//         Ok(())
//     }
// }
