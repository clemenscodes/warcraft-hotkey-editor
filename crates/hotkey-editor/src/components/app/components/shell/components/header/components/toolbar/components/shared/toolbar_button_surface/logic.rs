use super::components::attention_surface::AttentionSurfaceProps;
use super::components::clear_surface::ClearSurfaceProps;
use super::components::interactive_surface::InteractiveSurfaceProps;
use super::components::shared::toolbar_button_icon::ToolbarButtonIconProps;
use super::props::ToolbarButtonSurfaceProps;

impl From<&ToolbarButtonSurfaceProps> for ToolbarButtonIconProps {
    fn from(props: &ToolbarButtonSurfaceProps) -> Self {
        let icon = props.icon;
        Self { icon }
    }
}

impl From<&ToolbarButtonSurfaceProps> for InteractiveSurfaceProps {
    fn from(props: &ToolbarButtonSurfaceProps) -> Self {
        let glyph = ToolbarButtonIconProps::from(props);
        let aria_label = props.aria_label;
        let aria_haspopup = props.aria_haspopup;
        let aria_expanded = props.aria_expanded;
        let aria_pressed = props.aria_pressed;
        let data_action = props.data_action;
        let disabled = props.disabled;
        let onclick = props.onclick;
        Self {
            glyph,
            aria_label,
            aria_haspopup,
            aria_expanded,
            aria_pressed,
            data_action,
            disabled,
            onclick,
        }
    }
}

impl From<&ToolbarButtonSurfaceProps> for AttentionSurfaceProps {
    fn from(props: &ToolbarButtonSurfaceProps) -> Self {
        let glyph = ToolbarButtonIconProps::from(props);
        let aria_label = props.aria_label;
        let aria_haspopup = props.aria_haspopup;
        let aria_expanded = props.aria_expanded;
        let aria_pressed = props.aria_pressed;
        let data_action = props.data_action;
        let disabled = props.disabled;
        let onclick = props.onclick;
        Self {
            glyph,
            aria_label,
            aria_haspopup,
            aria_expanded,
            aria_pressed,
            data_action,
            disabled,
            onclick,
        }
    }
}

impl From<&ToolbarButtonSurfaceProps> for ClearSurfaceProps {
    fn from(props: &ToolbarButtonSurfaceProps) -> Self {
        let glyph = ToolbarButtonIconProps::from(props);
        let aria_label = props.aria_label;
        let aria_haspopup = props.aria_haspopup;
        let aria_expanded = props.aria_expanded;
        let aria_pressed = props.aria_pressed;
        let data_action = props.data_action;
        let disabled = props.disabled;
        let onclick = props.onclick;
        Self {
            glyph,
            aria_label,
            aria_haspopup,
            aria_expanded,
            aria_pressed,
            data_action,
            disabled,
            onclick,
        }
    }
}
