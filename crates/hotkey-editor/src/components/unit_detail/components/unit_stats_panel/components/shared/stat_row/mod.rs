mod props;
mod state;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::StatRowProps;
pub use state::StatRowVariant;
use style::CLASS;
assert_component!(StatRow);

/// A stat row: the `group` whose `data-variant`/`data-regen`/`data-primary` drive
/// its children's colours. It owns the row shape only; the semantic row that wraps
/// it supplies the label and the domain-typed value.
#[component]
pub fn StatRow(props: StatRowProps) -> Element {
    let variant = props.variant.data_attr();
    let is_regen = props.is_regen;
    let is_primary = props.is_primary;
    let children = props.children;
    rsx! {
        div {
            class: CLASS,
            "data-variant": variant,
            "data-regen": is_regen,
            "data-primary": is_primary,
            {children}
        }
    }
}
