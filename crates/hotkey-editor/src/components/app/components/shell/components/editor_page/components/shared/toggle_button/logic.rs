use super::components::active_toggle_button::ActiveToggleButtonProps;
use super::components::idle_toggle_button::IdleToggleButtonProps;
use super::props::ToggleButtonProps;

impl From<&ToggleButtonProps> for IdleToggleButtonProps {
    fn from(props: &ToggleButtonProps) -> Self {
        let label = props.label;
        let title = props.title;
        let onclick = props.onclick;
        let onkeydown = props.onkeydown;
        Self {
            label,
            title,
            onclick,
            onkeydown,
        }
    }
}

impl From<&ToggleButtonProps> for ActiveToggleButtonProps {
    fn from(props: &ToggleButtonProps) -> Self {
        let label = props.label;
        let title = props.title;
        let onclick = props.onclick;
        let onkeydown = props.onkeydown;
        Self {
            label,
            title,
            onclick,
            onkeydown,
        }
    }
}
