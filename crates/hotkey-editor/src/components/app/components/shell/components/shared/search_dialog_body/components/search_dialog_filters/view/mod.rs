#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct SearchDialogFiltersView {
    pub leaving: bool,
}

impl ddd::Layered for SearchDialogFiltersView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SearchDialogFiltersView {}
