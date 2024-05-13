use zed_extension_api::{self as zed, Result};

struct MyExtension {}

impl zed::Extension for MyExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Err("language server is not defined".to_string())
    }
}

zed::register_extension!(MyExtension);
