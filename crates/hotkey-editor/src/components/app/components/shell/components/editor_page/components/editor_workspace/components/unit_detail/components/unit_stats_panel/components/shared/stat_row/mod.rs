pub mod components;
mod kind;
mod props;
mod stat_figure;
mod state;
mod style;

use components::stat_row_label::StatRowLabel;
use dioxus::prelude::*;
pub use kind::StatRowKind;
pub use props::StatRowProps;
pub use state::StatRowVariant;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(StatRow);

/// The base stat row: a pure renderer of a label plus a value-side, generic over the
/// [`StatRowKind`] that supplies both. It owns the row shape and the
/// `data-variant`/`data-regen`/`data-primary` group flags only; the bound kind
/// supplies the label and the variant and renders the value from its domain type, so
/// the shape is written once and every semantic row (hit points, mana, armor, …)
/// inherits it — exactly as `Grid` is written once for every tile kind.
#[component]
pub fn StatRow<B: StatRowKind>(props: StatRowProps<B>) -> Element {
    let variant = B::variant_attribute();
    let is_regen = B::is_regen();
    let is_primary = B::is_primary(&props.value);
    let label = B::label();
    let value = props.value;
    rsx! {
        div {
            class: CLASS,
            "data-variant": variant,
            "data-regen": is_regen,
            "data-primary": is_primary,
            StatRowLabel { text: label }
            {B::cells(value)}
        }
    }
}
