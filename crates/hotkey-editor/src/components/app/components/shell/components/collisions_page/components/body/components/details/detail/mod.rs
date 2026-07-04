pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::conflict_grid::ConflictGrid;
use components::detail_header::DetailHeader;
use dioxus::prelude::*;
pub use props::{DetailBody, DetailContent, DetailProps};
use style::CLASS;
assert_component!(Detail);

/// The base detail pane: the bordered section shell. It shows the empty prompt or
/// the loaded header + conflict grid — the kind extension decides which and fills
/// the content; the base owns only the shell.
#[component]
pub fn Detail(props: DetailProps) -> Element {
    match props.content {
        DetailContent::Empty(prompt) => rsx! {
            section {
                class: CLASS,
                "data-empty": true,
                p { {prompt} }
            }
        },
        DetailContent::Loaded(body) => rsx! {
            section {
                class: CLASS,
                DetailHeader { {body.header} }
                ConflictGrid { {body.cards} }
            }
        },
    }
}
