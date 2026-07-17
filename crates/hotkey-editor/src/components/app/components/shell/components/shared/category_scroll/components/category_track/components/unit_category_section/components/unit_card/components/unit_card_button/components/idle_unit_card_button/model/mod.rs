use super::view::IdleUnitCardButtonView;
use crate::components::app::components::shell::components::shared::icons::IconUrl;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct IdleUnitCardButtonModel {
    pub icon_path: Option<IconUrl>,
    #[props(into)]
    pub display_name: String,
    pub unit_id: WarcraftObjectId,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&IdleUnitCardButtonView> for IdleUnitCardButtonModel {
    fn from(view: &IdleUnitCardButtonView) -> Self {
        let IdleUnitCardButtonView {
            icon_path,
            display_name,
            unit_id,
            onclick,
            onkeydown,
        } = view.clone();
        Self {
            icon_path,
            display_name,
            unit_id,
            onclick,
            onkeydown,
        }
    }
}

impl ddd::Model for IdleUnitCardButtonModel {
    type View = IdleUnitCardButtonView;
}
