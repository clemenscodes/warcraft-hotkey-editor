use crate::components::app::components::shell::components::header::components::toolbar::components::shared::toolbar_button_surface::components::shared::toolbar_button_icon::ToolbarButtonIconProps;
use dioxus::prelude::*;

/// The attention resting look of a toolbar surface: a persistently gold surface used
/// when the button is surfacing a condition that needs the user's eye. Carries every
/// attribute needed to render the `<button>`; the glyph rides along as its
/// already-shaped child props (data, not `Element`). Built by the dispatcher from
/// `ToolbarButtonSurfaceProps`.
#[derive(Props, Clone, PartialEq)]
pub struct AttentionSurfaceProps {
    pub glyph: ToolbarButtonIconProps,
    pub aria_label: &'static str,
    pub aria_haspopup: Option<&'static str>,
    pub aria_expanded: Option<bool>,
    pub aria_pressed: Option<bool>,
    pub disabled: bool,
    pub onclick: EventHandler<MouseEvent>,
}
