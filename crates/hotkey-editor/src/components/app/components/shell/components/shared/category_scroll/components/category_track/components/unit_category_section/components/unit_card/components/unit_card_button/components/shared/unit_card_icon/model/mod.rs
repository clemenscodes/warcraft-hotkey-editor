use super::view::UnitCardIconView;
use crate::components::app::components::shell::components::shared::icons::IconUrl;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UnitCardIconModel {
    pub icon_path: Option<IconUrl>,
    #[props(into)]
    pub display_name: String,
}

impl From<&UnitCardIconView> for UnitCardIconModel {
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

impl ddd::Model for UnitCardIconModel {
    type View = UnitCardIconView;
}
