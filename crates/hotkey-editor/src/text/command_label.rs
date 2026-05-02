pub(crate) struct CommandLabel;

impl CommandLabel {
    pub(crate) fn pretty(command_name: &str) -> String {
        let stripped = command_name.strip_prefix("Cmd").unwrap_or(command_name);
        if stripped.is_empty() {
            return command_name.to_string();
        }
        stripped.to_string()
    }
}
