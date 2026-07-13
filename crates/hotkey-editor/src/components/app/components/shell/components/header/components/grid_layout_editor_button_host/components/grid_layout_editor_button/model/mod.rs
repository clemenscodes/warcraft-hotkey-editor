use super::view::GridLayoutEditorButtonView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GridLayoutEditorButtonModel {
    pub is_open: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&GridLayoutEditorButtonView> for GridLayoutEditorButtonModel {
    fn from(view: &GridLayoutEditorButtonView) -> Self {
        let GridLayoutEditorButtonView { is_open, onclick } = view.clone();
        Self { is_open, onclick }
    }
}

impl ddd::Model for GridLayoutEditorButtonModel {
    type View = GridLayoutEditorButtonView;
}
