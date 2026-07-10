use super::components::available_key::AvailableKeyProps;
use super::components::conflict_key::ConflictKeyProps;
use super::components::current_key::CurrentKeyProps;
use super::props::ColorKeyProps;

/// The three colors share every button attribute and the conflict tooltip; they arrive
/// already shaped on `ColorKeyProps`, so each color's props is just the same data minus
/// the state the dispatcher already matched on.
impl From<&ColorKeyProps> for AvailableKeyProps {
    fn from(props: &ColorKeyProps) -> Self {
        let label = props.label.clone();
        let disabled = props.disabled;
        let onclick = props.onclick;
        let tooltip = props.tooltip.clone();
        Self {
            label,
            disabled,
            onclick,
            tooltip,
        }
    }
}

impl From<&ColorKeyProps> for CurrentKeyProps {
    fn from(props: &ColorKeyProps) -> Self {
        let label = props.label.clone();
        let disabled = props.disabled;
        let onclick = props.onclick;
        let tooltip = props.tooltip.clone();
        Self {
            label,
            disabled,
            onclick,
            tooltip,
        }
    }
}

impl From<&ColorKeyProps> for ConflictKeyProps {
    fn from(props: &ColorKeyProps) -> Self {
        let label = props.label.clone();
        let disabled = props.disabled;
        let onclick = props.onclick;
        let tooltip = props.tooltip.clone();
        Self {
            label,
            disabled,
            onclick,
            tooltip,
        }
    }
}
