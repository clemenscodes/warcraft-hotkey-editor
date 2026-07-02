mod props;
mod state;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::StatColumnProps;
pub use state::StatColumnKind;
use style::CLASS;
assert_component!(StatColumn);

/// One stat category column, placed by its named grid area.
#[component]
pub fn StatColumn(props: StatColumnProps) -> Element {
    let column = props.kind.data_attr();
    let with_icon = props.with_icon;
    let children = props.children;
    rsx! {
        div {
            class: CLASS,
            "data-column": column,
            "data-with-icon": with_icon,
            {children}
        }
    }
}
