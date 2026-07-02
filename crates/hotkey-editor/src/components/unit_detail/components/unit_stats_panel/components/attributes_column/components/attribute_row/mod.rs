mod props;

use crate::components::unit_detail::components::unit_stats_panel::components::shared::stat_row::StatRow;
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
    let value = Some(value_text);
    let gain = Some(per_level_text);
    rsx! {
        StatRow {
            is_primary,
            label,
            value,
            gain,
        }
    }
}
