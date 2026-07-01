use dioxus::prelude::*;

/// A slot's caption (e.g. "SLOT 1", "HERO 2") and whether it is a compact
/// (control-group) cell, which tightens the caption on small viewports.
#[derive(Props, Clone, PartialEq)]
pub struct SystemSlotLabelProps {
    #[props(into)]
    pub text: String,
    pub compact: bool,
}
