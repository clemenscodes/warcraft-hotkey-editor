use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ActiveRaceTabView {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl ddd::Layered for ActiveRaceTabView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ActiveRaceTabView {}
