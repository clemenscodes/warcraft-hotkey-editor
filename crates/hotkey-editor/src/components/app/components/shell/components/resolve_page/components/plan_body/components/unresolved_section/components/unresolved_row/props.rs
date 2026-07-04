use crate::components::app::components::shell::components::resolve_page::logic::{
    CarriersDialogData, UnresolvedView,
};
use dioxus::prelude::*;

/// One ability the cascade could not place, with the carriers-dialog signal its
/// icon opens.
#[derive(Props, Clone, PartialEq)]
pub struct UnresolvedRowProps {
    pub unresolved_view: UnresolvedView,
    pub carriers_dialog: Signal<Option<CarriersDialogData>>,
}
