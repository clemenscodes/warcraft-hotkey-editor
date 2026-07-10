use super::props::NarrowKeySlotProps;
use crate::components::app::components::shell::components::shared::key_picker_board::components::key_picker_column::components::key_picker_row::components::key_picker_key::components::shared::color_key::ColorKeyProps;

impl From<&NarrowKeySlotProps> for ColorKeyProps {
    fn from(props: &NarrowKeySlotProps) -> Self {
        let state = props.state;
        let label = props.label.clone();
        let data_label = props.data_label.clone();
        let disabled = props.disabled;
        let onclick = props.onclick;
        let tooltip = props.tooltip.clone();
        Self {
            state,
            label,
            data_label,
            disabled,
            onclick,
            tooltip,
        }
    }
}
