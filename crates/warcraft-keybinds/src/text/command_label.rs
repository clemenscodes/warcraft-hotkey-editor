#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CommandLabel;

impl CommandLabel {
    pub fn pretty(command_name: &str) -> String {
        let stripped = command_name.strip_prefix("Cmd").unwrap_or(command_name);
        if stripped.is_empty() {
            return command_name.to_string();
        }
        stripped.to_string()
    }
}
