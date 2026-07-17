use super::view::UnitCardButtonView;
use crate::components::app::components::shell::components::shared::icons::IconUrl;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct UnitCardButtonModel {
    pub icon_path: Option<IconUrl>,
    #[props(into)]
    pub display_name: String,
    pub unit_id: WarcraftObjectId,
    pub is_selected: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&UnitCardButtonView> for UnitCardButtonModel {
    fn from(view: &UnitCardButtonView) -> Self {
        let UnitCardButtonView {
            icon_path,
            display_name,
            unit_id,
            is_selected,
            onclick,
            onkeydown,
        } = view.clone();
        Self {
            icon_path,
            display_name,
            unit_id,
            is_selected,
            onclick,
            onkeydown,
        }
    }
}

impl ddd::Model for UnitCardButtonModel {
    type View = UnitCardButtonView;
}
