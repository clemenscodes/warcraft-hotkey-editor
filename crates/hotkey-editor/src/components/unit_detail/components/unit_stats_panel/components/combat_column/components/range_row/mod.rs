mod props;

use super::super::super::shared::stat_row::{StatRow, StatRowProps};
use super::super::kinds::RangeKind;
use dioxus::prelude::*;
pub use props::RangeRowProps;

/// The range row, shown only for a ranged attack. A melee attack has a reach of zero
/// and no row at all. A guarded leaf that early-returns.
#[component]
pub fn RangeRow(props: RangeRowProps) -> Element {
    let range = props.range;
    if range.is_zero() {
        return rsx! {};
    }
    let row = StatRowProps::<RangeKind> { value: range };
    rsx! {
        StatRow { ..row }
    }
}
