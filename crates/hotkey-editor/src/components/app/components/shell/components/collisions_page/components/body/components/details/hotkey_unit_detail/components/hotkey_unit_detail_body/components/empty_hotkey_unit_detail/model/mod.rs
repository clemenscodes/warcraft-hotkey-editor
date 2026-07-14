use super::view::EmptyHotkeyUnitDetailView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct EmptyHotkeyUnitDetailModel {
    #[props(into)]
    pub prompt: String,
}

impl From<&EmptyHotkeyUnitDetailView> for EmptyHotkeyUnitDetailModel {
    fn from(view: &EmptyHotkeyUnitDetailView) -> Self {
        let EmptyHotkeyUnitDetailView { prompt } = view.clone();
        Self { prompt }
    }
}

impl ddd::Model for EmptyHotkeyUnitDetailModel {
    type View = EmptyHotkeyUnitDetailView;
}
