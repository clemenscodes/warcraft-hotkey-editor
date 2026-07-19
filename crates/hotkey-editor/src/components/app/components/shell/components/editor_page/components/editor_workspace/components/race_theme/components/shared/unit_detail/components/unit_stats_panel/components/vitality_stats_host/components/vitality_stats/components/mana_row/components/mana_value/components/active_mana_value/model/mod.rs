use super::view::ActiveManaValueView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ActiveManaValueModel {
    #[props(into)]
    pub text: String,
}

impl From<&ActiveManaValueView> for ActiveManaValueModel {
    fn from(view: &ActiveManaValueView) -> Self {
        let ActiveManaValueView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for ActiveManaValueModel {
    type View = ActiveManaValueView;
}
