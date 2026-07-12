/// The published contract for the empty detail pane: the prompt it shows.
#[derive(Clone, PartialEq)]
pub struct EmptyHotkeyUnitDetailView {
    pub prompt: String,
}

impl ddd::View for EmptyHotkeyUnitDetailView {}
