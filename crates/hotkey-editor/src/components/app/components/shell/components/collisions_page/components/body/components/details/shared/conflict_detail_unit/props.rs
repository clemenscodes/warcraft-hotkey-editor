use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictDetailUnitProps {
    pub onclick: EventHandler<MouseEvent>,
    pub icon_url: Option<String>,
    #[props(into)]
    pub name: String,
}
