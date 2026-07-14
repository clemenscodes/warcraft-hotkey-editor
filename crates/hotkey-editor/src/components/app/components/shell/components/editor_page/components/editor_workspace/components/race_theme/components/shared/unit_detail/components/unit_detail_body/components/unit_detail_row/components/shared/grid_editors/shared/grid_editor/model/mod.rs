use super::view::GridEditorView;
use dioxus::prelude::*;
use warcraft_keybinds::GridBehavior;

#[derive(Props, Clone, PartialEq)]
pub struct GridEditorModel<B: GridBehavior> {
    #[props(default)]
    pub(crate) behavior: B,
    pub(crate) config: GridEditorView,
}

impl<B: GridBehavior> From<&GridEditorView> for GridEditorModel<B> {
    fn from(config: &GridEditorView) -> Self {
        let behavior = B::default();
        let config = config.clone();
        Self { behavior, config }
    }
}

impl<B: GridBehavior> ddd::Model for GridEditorModel<B> {
    type View = GridEditorView;
}
