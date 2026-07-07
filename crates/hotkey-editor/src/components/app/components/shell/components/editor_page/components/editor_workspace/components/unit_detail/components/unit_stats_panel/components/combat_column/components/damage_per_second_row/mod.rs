mod props;

use super::super::super::shared::stat_row::{StatRow, StatRowProps};
use super::super::kinds::DamagePerSecondKind;
use dioxus::prelude::*;
pub use props::DamagePerSecondRowProps;

/// The damage-per-second row, shown only when the attack has a real cooldown. A
/// guarded leaf that early-returns when the rate is undefined.
use tw_macro::assert_component;
assert_component!(DamagePerSecondRow);
#[component]
pub fn DamagePerSecondRow(props: DamagePerSecondRowProps) -> Element {
    let Some(damage_per_second) = props.damage_per_second else {
        return rsx! {};
    };
    let row = StatRowProps::<DamagePerSecondKind> {
        value: damage_per_second,
    };
    rsx! {
        StatRow { ..row }
    }
}
