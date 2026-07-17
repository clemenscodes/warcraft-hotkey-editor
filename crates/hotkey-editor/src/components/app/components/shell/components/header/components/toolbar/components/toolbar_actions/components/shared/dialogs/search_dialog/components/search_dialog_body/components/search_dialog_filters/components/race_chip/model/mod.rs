use super::view::RaceChipView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct RaceChipModel {
    pub label: &'static str,
    pub active: bool,
    pub on_pick: EventHandler<MouseEvent>,
}

impl From<&RaceChipView> for RaceChipModel {
    fn from(view: &RaceChipView) -> Self {
        let RaceChipView {
            label,
            active,
            on_pick,
        } = view.clone();
        Self {
            label,
            active,
            on_pick,
        }
    }
}

impl ddd::Model for RaceChipModel {
    type View = RaceChipView;
}
