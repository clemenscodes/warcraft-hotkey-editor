use super::view::ResolveApplyBarView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ResolveApplyBarModel {
    #[props(into)]
    pub moves_text: String,
    pub unresolved_count: usize,
    pub running: bool,
    pub on_apply: EventHandler<MouseEvent>,
}

impl From<&ResolveApplyBarView> for ResolveApplyBarModel {
    fn from(view: &ResolveApplyBarView) -> Self {
        let ResolveApplyBarView {
            moves_text,
            unresolved_count,
            running,
            on_apply,
        } = view.clone();
        Self {
            moves_text,
            unresolved_count,
            running,
            on_apply,
        }
    }
}

impl ddd::Model for ResolveApplyBarModel {
    type View = ResolveApplyBarView;
}
