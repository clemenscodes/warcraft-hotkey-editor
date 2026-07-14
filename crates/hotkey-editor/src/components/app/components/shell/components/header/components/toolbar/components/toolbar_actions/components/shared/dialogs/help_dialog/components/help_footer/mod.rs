pub mod components;
mod model;
mod style;
mod view;

pub use view::HelpFooterView;

use components::help_dismiss::HelpDismiss;
use dioxus::prelude::*;
use model::HelpFooterModel;
use style::CLASS;
use tw_macro::assert_component;

/// The pinned bar below the scrolling guide, separated from it by a gold rule, holding the
/// dismiss button that closes the guide and records that the player has seen it. Presentational:
/// the dialog that owns the open signal builds the dismiss handler and hands it in. A dialog
/// places it as its footer region, so it stays put while the guide scrolls above it.
#[component]
pub fn HelpFooter(props: HelpFooterModel) -> Element {
    let on_dismiss = props.on_dismiss;
    rsx! {
        footer {
            class: CLASS,
            HelpDismiss {
                on_dismiss,
            }
        }
    }
}

assert_component!(HelpFooter);
