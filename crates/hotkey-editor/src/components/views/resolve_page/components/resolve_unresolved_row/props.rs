use crate::components::views::resolve_page::logic::{CarriersDialogData, ResolveUnresolvedView};
use dioxus::prelude::*;

/// One ability the cascade could not place, with the carriers-dialog signal its
/// icon opens.
#[derive(Props, Clone, PartialEq)]
pub struct ResolveUnresolvedRowProps {
    pub unresolved_view: ResolveUnresolvedView,
    pub carriers_dialog: Signal<Option<CarriersDialogData>>,
}
