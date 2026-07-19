use super::view::ActiveManaRegenGainView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ActiveManaRegenGainModel {
    #[props(into)]
    pub text: String,
}

impl From<&ActiveManaRegenGainView> for ActiveManaRegenGainModel {
    fn from(view: &ActiveManaRegenGainView) -> Self {
        let ActiveManaRegenGainView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for ActiveManaRegenGainModel {
    type View = ActiveManaRegenGainView;
}
