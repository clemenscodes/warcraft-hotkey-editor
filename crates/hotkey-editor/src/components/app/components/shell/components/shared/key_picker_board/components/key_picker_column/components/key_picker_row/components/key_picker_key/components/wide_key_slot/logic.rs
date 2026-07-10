use super::props::WideKeySlotProps;
use crate::components::app::components::shell::components::shared::key_picker_board::components::key_picker_column::components::key_picker_row::components::key_picker_key::components::shared::color_key::ColorKeyProps;

impl From<&WideKeySlotProps> for ColorKeyProps {
    fn from(props: &WideKeySlotProps) -> Self {
        let state = props.state;
        let label = props.label.clone();
        let disabled = props.disabled;
        let onclick = props.onclick;
        let tooltip = props.tooltip.clone();
        Self {
            state,
            label,
            disabled,
            onclick,
            tooltip,
        }
    }
}
