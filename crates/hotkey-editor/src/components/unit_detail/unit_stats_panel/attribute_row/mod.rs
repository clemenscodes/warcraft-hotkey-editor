mod props;

use super::stat_row::StatRow;
use super::stat_row_gain::StatRowGain;
use super::stat_row_label::StatRowLabel;
use super::stat_row_value::StatRowValue;
use dioxus::prelude::*;
pub use props::AttributeRowProps;

/// One hero attribute row (name, value, per-level gain). A `StatRow` marked primary
/// for the hero's main attribute.
#[component]
pub fn AttributeRow(props: AttributeRowProps) -> Element {
    let label = props.label;
    let is_primary = props.is_primary;
    let value_text = props.value.to_string();
    let per_level_text = format!("+{:.1}", props.per_level);
    rsx! {
        StatRow {
            is_primary,
            StatRowLabel { text: label }
            StatRowValue { text: value_text }
            StatRowGain { text: per_level_text }
        }
    }
}
