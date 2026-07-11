use super::view::GridLayoutButtonView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GridLayoutButtonModel {
    pub is_open: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&GridLayoutButtonView> for GridLayoutButtonModel {
    fn from(view: &GridLayoutButtonView) -> Self {
        let GridLayoutButtonView { is_open, onclick } = view.clone();
        Self { is_open, onclick }
    }
}

impl ddd::Model for GridLayoutButtonModel {
    type View = GridLayoutButtonView;
}
