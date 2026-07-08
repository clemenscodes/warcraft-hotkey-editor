mod props;
mod style;

use dioxus::prelude::*;
pub use props::DetailProps;
use style::CLASS;
use tw_macro::assert_component;

assert_component!(Detail);

/// The base detail pane: the bordered section shell. Its parent fills it with the
/// empty prompt, or the loaded header and conflict grid; the base owns only the shell.
#[component]
pub fn Detail(props: DetailProps) -> Element {
    let is_empty = props.is_empty;
    let children = props.children;
    rsx! {
        section {
            class: CLASS,
            "data-empty": is_empty,
            {children}
        }
    }
}
