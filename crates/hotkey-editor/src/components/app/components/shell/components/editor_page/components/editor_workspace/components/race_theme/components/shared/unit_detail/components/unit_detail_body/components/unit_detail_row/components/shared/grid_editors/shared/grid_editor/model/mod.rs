use super::view::GridEditorView;
use dioxus::prelude::*;
use warcraft_keybinds::GridBehavior;

/// The `GridEditor` component's internal model (a `ddd::Model`): a [`GridEditorView`]
/// bound to a concrete [`GridBehavior`]. The behavior is a zero-sized marker; the
/// three variant wrappers each instantiate it with their own type. It exists so the
/// generic `GridEditor<B>` carries `B`, letting the `From` impls cascade and mutate
/// tiles. It is built from the published `GridEditorView` at the variant boundary.
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
