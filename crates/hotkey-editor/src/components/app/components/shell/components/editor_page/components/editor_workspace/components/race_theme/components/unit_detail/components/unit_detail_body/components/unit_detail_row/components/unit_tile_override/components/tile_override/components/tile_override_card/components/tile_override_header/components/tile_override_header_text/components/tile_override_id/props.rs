use dioxus::prelude::*;

/// The database object id shown under the name.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideIdProps {
    #[props(into)]
    pub text: String,
}
