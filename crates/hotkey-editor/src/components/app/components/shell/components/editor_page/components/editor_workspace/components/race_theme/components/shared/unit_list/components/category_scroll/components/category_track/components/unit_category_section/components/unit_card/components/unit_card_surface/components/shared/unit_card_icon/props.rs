use super::view::UnitCardIconView;
use crate::components::app::components::shell::components::shared::icons::IconUrl;
use dioxus::prelude::*;

/// The portrait source (absent for units without an icon) and the alt text.
#[derive(Props, Clone, PartialEq)]
pub struct UnitCardIconProps {
    pub icon_path: Option<IconUrl>,
    #[props(into)]
    pub display_name: String,
}

impl From<&UnitCardIconView> for UnitCardIconProps {
    fn from(view: &UnitCardIconView) -> Self {
        let UnitCardIconView {
            icon_path,
            display_name,
        } = view.clone();
        Self {
            icon_path,
            display_name,
        }
    }
}

impl ddd::Props for UnitCardIconProps {
    type View = UnitCardIconView;
}
