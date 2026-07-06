mod props;
mod style;

use dioxus::prelude::*;
pub use props::StatRowsProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(StatRows);

/// The stacked rows beside a stat column's icon.
#[component]
pub fn StatRows(props: StatRowsProps) -> Element {
    let children = props.children;
    rsx! {
        div {
            class: CLASS,
            {children}
        }
    }
}
