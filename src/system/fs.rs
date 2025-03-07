//! File system operations module for interacting with the local filesystem.
//!
//! This module provides a structured interface for various file system operations
//! including reading, writing, creating, and manipulating files and directories.

use mcp_core::handler::TypedToolHandler;
use mcp_core::{Content, toolset::ToolSet};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::default;
use std::os::unix::fs::PermissionsExt;

pub fn toolset() -> ToolSet {
    let mut tool_set = ToolSet::default();
    tool_set.add_tool(FsRead);
    tool_set.add_tool(FsWrite);
    tool_set.add_tool(FsCreate);
    tool_set.add_tool(FsDelete);
    tool_set.add_tool(FsRename);
    tool_set.add_tool(FsMove);
    tool_set.add_tool(FsCopy);
    tool_set.add_tool(FsListDirectory);
    tool_set.add_tool(FsMakeDirectory);
    tool_set.add_tool(FsRemoveDirectory);
    tool_set.add_tool(FsGetFileInfo);
    tool_set.add_tool(FsSetPermissions);
    tool_set.add_tool(FsChangeOwnership);
    tool_set
}

/// Parameters for reading from a file.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ReadParams {
    /// Path to the file to be read.
    pub path: String,
    /// Starting position (in bytes) from which to read.
    pub offset: Option<u32>,
    /// Number of bytes to read.
    pub length: Option<u32>,
}

pub struct FsRead;

impl TypedToolHandler for FsRead {
    type Params = ReadParams;
    fn name(&self) -> &'static str {
        "fs.read"
    }

    fn description(&self) -> &'static str {
        "Read data from a file."
    }

    async fn call(&self, params: Self::Params) -> mcp_core::ToolResult<Vec<mcp_core::Content>> {
        let content = tokio::fs::read(&params.path)
            .await
            .map_err(mcp_core::ToolError::execution)?;
        let content = String::from_utf8_lossy(&content).to_string();
        Ok(vec![Content::text(content)])
    }
}

pub struct FsWrite;

impl TypedToolHandler for FsWrite {
    type Params = WriteParams;
    fn name(&self) -> &'static str {
        "fs.write"
    }

    fn description(&self) -> &'static str {
        "Write data to a file."
    }

    async fn call(&self, params: Self::Params) -> mcp_core::ToolResult<Vec<mcp_core::Content>> {
        use tokio::io::{AsyncSeekExt, AsyncWriteExt};

        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&params.path)
            .await
            .map_err(mcp_core::ToolError::execution)?;

        file.seek(tokio::io::SeekFrom::Start(params.offset))
            .await
            .map_err(mcp_core::ToolError::execution)?;

        file.write_all(params.data.as_bytes())
            .await
            .map_err(mcp_core::ToolError::execution)?;

        Ok(vec![])
    }
}

/// Parameters for writing to a file.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct WriteParams {
    /// Path to the file to be written to.
    pub path: String,
    /// Starting position (in bytes) at which to write.
    pub offset: u64,
    /// Data to be written to the file.
    pub data: String,
}

pub struct FsCreate;

impl TypedToolHandler for FsCreate {
    type Params = CreateParams;
    fn name(&self) -> &'static str {
        "fs.create"
    }

    fn description(&self) -> &'static str {
        "Create a new file with specified permissions."
    }

    async fn call(&self, params: Self::Params) -> mcp_core::ToolResult<Vec<mcp_core::Content>> {
        let file = tokio::fs::File::create(&params.path)
            .await
            .map_err(mcp_core::ToolError::execution)?;

        let mut perms = file
            .metadata()
            .await
            .map_err(mcp_core::ToolError::execution)?
            .permissions();

        perms.set_mode(params.permissions);
        tokio::fs::set_permissions(&params.path, perms)
            .await
            .map_err(mcp_core::ToolError::execution)?;

        Ok(vec![])
    }
}

/// Parameters for creating a new file.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CreateParams {
    /// Path where the new file should be created.
    pub path: String,
    /// Unix-style file permissions (e.g., 0o644).
    pub permissions: u32,
}

pub struct FsDelete;

impl TypedToolHandler for FsDelete {
    type Params = DeleteParams;
    fn name(&self) -> &'static str {
        "fs.delete"
    }

    fn description(&self) -> &'static str {
        "Delete a file."
    }

    async fn call(&self, params: Self::Params) -> mcp_core::ToolResult<Vec<mcp_core::Content>> {
        tokio::fs::remove_file(&params.path)
            .await
            .map_err(mcp_core::ToolError::execution)?;
        Ok(vec![])
    }
}

/// Parameters for deleting a file.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct DeleteParams {
    /// Path to the file to be deleted.
    pub path: String,
}

pub struct FsRename;

impl TypedToolHandler for FsRename {
    type Params = RenameParams;
    fn name(&self) -> &'static str {
        "fs.rename"
    }

    fn description(&self) -> &'static str {
        "Rename a file."
    }

    async fn call(&self, params: Self::Params) -> mcp_core::ToolResult<Vec<mcp_core::Content>> {
        tokio::fs::rename(&params.old_path, &params.new_path)
            .await
            .map_err(mcp_core::ToolError::execution)?;
        Ok(vec![])
    }
}

/// Parameters for renaming a file.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct RenameParams {
    /// Current path of the file.
    pub old_path: String,
    /// New path for the file.
    pub new_path: String,
}

pub struct FsMove;

impl TypedToolHandler for FsMove {
    type Params = MoveParams;
    fn name(&self) -> &'static str {
        "fs.move"
    }

    fn description(&self) -> &'static str {
        "Move a file to a new location."
    }

    async fn call(&self, params: Self::Params) -> mcp_core::ToolResult<Vec<mcp_core::Content>> {
        tokio::fs::rename(&params.source_path, &params.destination_path)
            .await
            .map_err(mcp_core::ToolError::execution)?;
        Ok(vec![])
    }
}

/// Parameters for moving a file.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct MoveParams {
    /// Current path of the file to be moved.
    pub source_path: String,
    /// Destination path where the file should be moved to.
    pub destination_path: String,
}

pub struct FsCopy;

impl TypedToolHandler for FsCopy {
    type Params = CopyParams;
    fn name(&self) -> &'static str {
        "fs.copy"
    }

    fn description(&self) -> &'static str {
        "Copy a file to a new location."
    }

    async fn call(&self, params: Self::Params) -> mcp_core::ToolResult<Vec<mcp_core::Content>> {
        tokio::fs::copy(&params.source_path, &params.destination_path)
            .await
            .map_err(mcp_core::ToolError::execution)?;
        Ok(vec![])
    }
}

/// Parameters for copying a file.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CopyParams {
    /// Path of the file to be copied.
    pub source_path: String,
    /// Destination path where the file should be copied to.
    pub destination_path: String,
}

pub struct FsListDirectory;

impl TypedToolHandler for FsListDirectory {
    type Params = ListDirectoryParams;
    fn name(&self) -> &'static str {
        "fs.list_directory"
    }

    fn description(&self) -> &'static str {
        "List contents of a directory."
    }

    async fn call(&self, params: Self::Params) -> mcp_core::ToolResult<Vec<mcp_core::Content>> {
        let mut entries = tokio::fs::read_dir(&params.path)
            .await
            .map_err(mcp_core::ToolError::execution)?;

        let mut contents = Vec::new();
        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(mcp_core::ToolError::execution)?
        {
            let path = entry.path();
            contents.push(Content::text(path.to_string_lossy().into_owned()));
        }

        Ok(contents)
    }
}

/// Parameters for listing the contents of a directory.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ListDirectoryParams {
    /// Path to the directory to be listed.
    pub path: String,
}

pub struct FsMakeDirectory;

impl TypedToolHandler for FsMakeDirectory {
    type Params = MakeDirectoryParams;
    fn name(&self) -> &'static str {
        "fs.make_directory"
    }

    fn description(&self) -> &'static str {
        "Create a new directory with specified permissions."
    }

    async fn call(&self, params: Self::Params) -> mcp_core::ToolResult<Vec<mcp_core::Content>> {
        tokio::fs::create_dir(&params.path)
            .await
            .map_err(mcp_core::ToolError::execution)?;

        let perms = std::fs::Permissions::from_mode(params.permissions);
        tokio::fs::set_permissions(&params.path, perms)
            .await
            .map_err(mcp_core::ToolError::execution)?;

        Ok(vec![])
    }
}

/// Parameters for creating a new directory.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct MakeDirectoryParams {
    /// Path where the new directory should be created.
    pub path: String,
    /// Unix-style directory permissions (e.g., 0o755).
    pub permissions: u32,
}

pub struct FsRemoveDirectory;

impl TypedToolHandler for FsRemoveDirectory {
    type Params = RemoveDirectoryParams;
    fn name(&self) -> &'static str {
        "fs.remove_directory"
    }

    fn description(&self) -> &'static str {
        "Remove a directory and optionally its contents."
    }

    async fn call(&self, params: Self::Params) -> mcp_core::ToolResult<Vec<mcp_core::Content>> {
        if params.recursive {
            tokio::fs::remove_dir_all(&params.path)
                .await
                .map_err(mcp_core::ToolError::execution)?;
        } else {
            tokio::fs::remove_dir(&params.path)
                .await
                .map_err(mcp_core::ToolError::execution)?;
        }
        Ok(vec![])
    }
}

/// Parameters for removing a directory.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct RemoveDirectoryParams {
    /// Path to the directory to be removed.
    pub path: String,
    /// Whether to recursively remove all contents of the directory.
    pub recursive: bool,
}

pub struct FsGetFileInfo;

impl TypedToolHandler for FsGetFileInfo {
    type Params = GetFileInfoParams;
    fn name(&self) -> &'static str {
        "fs.get_file_info"
    }

    fn description(&self) -> &'static str {
        "Get metadata information about a file."
    }

    async fn call(&self, params: Self::Params) -> mcp_core::ToolResult<Vec<mcp_core::Content>> {
        let metadata = tokio::fs::metadata(&params.path)
            .await
            .map_err(mcp_core::ToolError::execution)?;

        let info = serde_json::json!({
            "path": params.path,
            "size": metadata.len(),
            "modified": metadata.modified()
                .map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs())
                .ok(),
            "is_dir": metadata.is_dir(),
            "is_file": metadata.is_file(),
            "permissions": metadata.permissions().mode(),
        });

        Ok(vec![Content::text(
            serde_json::to_string(&info).expect("invalid json"),
        )])
    }
}

/// Parameters for retrieving file information.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct GetFileInfoParams {
    /// Path to the file for which to get information.
    pub path: String,
}

pub struct FsSetPermissions;

impl TypedToolHandler for FsSetPermissions {
    type Params = SetFilePermissionsParams;
    fn name(&self) -> &'static str {
        "fs.set_permissions"
    }

    fn description(&self) -> &'static str {
        "Set file permissions."
    }

    async fn call(&self, params: Self::Params) -> mcp_core::ToolResult<Vec<mcp_core::Content>> {
        let perms = std::fs::Permissions::from_mode(params.permissions);
        tokio::fs::set_permissions(&params.path, perms)
            .await
            .map_err(mcp_core::ToolError::execution)?;
        Ok(vec![])
    }
}

/// Parameters for setting file permissions.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct SetFilePermissionsParams {
    /// Path to the file whose permissions should be changed.
    pub path: String,
    /// New Unix-style permissions to set (e.g., 0o644).
    pub permissions: u32,
}

pub struct FsChangeOwnership;

impl TypedToolHandler for FsChangeOwnership {
    type Params = ChangeOwnershipParams;
    fn name(&self) -> &'static str {
        "fs.change_ownership"
    }

    fn description(&self) -> &'static str {
        "Change file ownership (Unix only)."
    }

    async fn call(&self, params: Self::Params) -> mcp_core::ToolResult<Vec<mcp_core::Content>> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::chown;
            chown(&params.path, Some(params.user_id), Some(params.group_id))
                .map_err(mcp_core::ToolError::execution)?;
            Ok(vec![])
        }

        #[cfg(not(unix))]
        Err(mcp_core::ToolError::validation(
            "Changing ownership is only supported on Unix systems",
        ))
    }
}

/// Parameters for changing file ownership.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ChangeOwnershipParams {
    /// Path to the file whose ownership should be changed.
    pub path: String,
    /// User ID to set as the file's owner.
    pub user_id: u32,
    /// Group ID to set as the file's group.
    pub group_id: u32,
}
