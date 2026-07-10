use super::components::normal_override_key::NormalOverrideKeyProps;
use super::components::special_override_key::SpecialOverrideKeyProps;
use super::props::OverrideKeyProps;
use crate::components::app::components::shell::components::shared::editable_keycap::EditableKeycapProps;

impl From<&OverrideKeyProps> for NormalOverrideKeyProps {
    fn from(props: &OverrideKeyProps) -> Self {
        let keycap = EditableKeycapProps::from(props);
        let title = props.title.clone();
        let is_focus_target = props.is_focus_target;
        let on_activate = props.on_activate;
        Self {
            keycap,
            title,
            is_focus_target,
            on_activate,
        }
    }
}

impl From<&OverrideKeyProps> for SpecialOverrideKeyProps {
    fn from(props: &OverrideKeyProps) -> Self {
        let keycap = EditableKeycapProps::from(props);
        let title = props.title.clone();
        let is_focus_target = props.is_focus_target;
        let on_activate = props.on_activate;
        Self {
            keycap,
            title,
            is_focus_target,
            on_activate,
        }
    }
}
