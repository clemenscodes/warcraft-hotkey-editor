use super::view::StatLabelView;
use dioxus::prelude::*;

/// A stat label leaf's input: the row name it presents. This is the default label look
/// every plain row shares — gold, sitting at the row's start. A row whose label
/// carries its own identity (an attribute's primary glow, a regeneration's dimmer
/// gold) nests its own label leaf instead.
#[derive(Props, Clone, PartialEq)]
pub struct StatLabelModel {
    #[props(into)]
    pub text: String,
}

impl From<&StatLabelView> for StatLabelModel {
    fn from(view: &StatLabelView) -> Self {
        let StatLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for StatLabelModel {
    type View = StatLabelView;
}
