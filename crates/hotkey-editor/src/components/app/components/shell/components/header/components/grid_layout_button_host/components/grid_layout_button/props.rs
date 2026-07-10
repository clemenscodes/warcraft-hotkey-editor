use super::view::GridLayoutButtonView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GridLayoutButtonProps {
    pub is_open: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&GridLayoutButtonView> for GridLayoutButtonProps {
    fn from(view: &GridLayoutButtonView) -> Self {
        let GridLayoutButtonView { is_open, onclick } = view.clone();
        Self { is_open, onclick }
    }
}

impl ddd::Props for GridLayoutButtonProps {
    type View = GridLayoutButtonView;
}
