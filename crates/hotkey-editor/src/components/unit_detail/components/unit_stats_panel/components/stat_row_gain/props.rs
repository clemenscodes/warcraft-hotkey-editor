use dioxus::prelude::*;

/// A stat row's per-level or regen gain (e.g. "+2.5"). `is_zero` mutes it.
#[derive(Props, Clone, PartialEq)]
pub struct StatRowGainProps {
    #[props(into)]
    pub text: String,
    #[props(default)]
    pub is_zero: bool,
}
