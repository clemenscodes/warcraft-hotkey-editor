mod props;

use super::super::kinds::EvasionKind;
use crate::components::unit_detail::components::unit_stats_panel::components::shared::stat_row::{
    StatRow, StatRowProps,
};
use dioxus::prelude::*;
pub use props::EvasionRowProps;

/// The evasion row, shown only when the unit can dodge. A unit with no evasion source
/// has no row at all — the base row would otherwise print a muted "0%". A guarded
/// leaf that early-returns, asking the domain whether there is any evasion.
#[component]
pub fn EvasionRow(props: EvasionRowProps) -> Element {
    let evasion = props.evasion;
    if evasion.is_zero() {
        return rsx! {};
    }
    let row = StatRowProps::<EvasionKind> { value: evasion };
    rsx! {
        StatRow { ..row }
    }
}
