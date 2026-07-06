use crate::collision::cross_unit::CrossUnitCollisionReport;
use crate::collision::unit_report::UnitCollisionReport;
use crate::custom_keys::CustomKeys;
use crate::grid::layout::GridLayout;

/// The headline collision counts, one per collision class, aggregated across the
/// whole config. This is the number the toolbar badge shows and the figure e2e
/// asserts on. Aggregating the two collision reports is a domain decision, so it
/// lives here and never in the renderer — the UI reads these counts through a
/// hook and only displays them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct CollisionSummary {
    cross_unit: usize,
    per_unit_position: usize,
    per_unit_hotkey: usize,
}

impl CollisionSummary {
    /// Count every collision the reports surface, grouped by class:
    /// cross-unit position islands, per-unit position collisions, and per-unit
    /// hotkey collisions.
    pub fn compute(custom_keys: &CustomKeys, layout: GridLayout) -> Self {
        let cross_unit_report = CrossUnitCollisionReport::compute(custom_keys);
        let cross_unit = cross_unit_report.position_groups().len();
        let unit_report = UnitCollisionReport::compute(custom_keys, layout);
        let mut per_unit_position: usize = 0;
        let mut per_unit_hotkey: usize = 0;
        for entry in unit_report.entries() {
            for card in entry.position_cards() {
                per_unit_position += card.into_iter().count();
            }
            for card in entry.hotkey_cards() {
                per_unit_hotkey += card.into_iter().count();
            }
        }
        Self {
            cross_unit,
            per_unit_position,
            per_unit_hotkey,
        }
    }

    /// Cross-unit position groups — cells where two or more units share an
    /// ability and at least one has a multi-button collision there.
    pub fn cross_unit(&self) -> usize {
        self.cross_unit
    }

    /// Per-unit position collisions — cells on a single unit's command card
    /// where two or more of its abilities land at the same slot.
    pub fn per_unit_position(&self) -> usize {
        self.per_unit_position
    }

    /// Per-unit hotkey collisions — letters on a single unit's command card
    /// claimed by two or more buttons.
    pub fn per_unit_hotkey(&self) -> usize {
        self.per_unit_hotkey
    }

    /// Every collision, regardless of class.
    pub fn total(&self) -> usize {
        self.cross_unit + self.per_unit_position + self.per_unit_hotkey
    }

    /// True when the config has no collisions of any class.
    pub fn is_clean(&self) -> bool {
        self.total() == 0
    }
}

impl ddd::ReadModel for CollisionSummary {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{AbilityBinding, ColumnIndex, GridCoordinate, Hotkey, RowIndex};

    fn run_on_large_stack<F>(check: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let join_handle = std::thread::Builder::new()
            .stack_size(32 * 1024 * 1024)
            .spawn(check)
            .unwrap();
        join_handle.join().unwrap();
    }

    #[test]
    fn shared_position_counts_position_collisions() {
        run_on_large_stack(|| {
            let shared_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
            let holy_light_binding = AbilityBinding::builder()
                .button_position(shared_position)
                .build();
            let divine_shield_binding = AbilityBinding::builder()
                .button_position(shared_position)
                .build();
            let mut custom_keys = CustomKeys::from_text("");
            custom_keys.put_ability("AHhb", holy_light_binding);
            custom_keys.put_ability("AHds", divine_shield_binding);
            let layout = GridLayout::qwerty_grid();
            let summary = CollisionSummary::compute(&custom_keys, layout);
            assert!(
                !summary.is_clean(),
                "two Paladin abilities at (0,0) must register as collisions",
            );
            assert!(
                summary.cross_unit() >= 1,
                "the shared position must appear as a cross-unit island",
            );
            assert!(
                summary.per_unit_position() >= 1,
                "the shared position must appear as a per-unit position collision",
            );
            assert_eq!(
                summary.total(),
                summary.cross_unit() + summary.per_unit_position() + summary.per_unit_hotkey(),
                "total must be the sum of every class",
            );
        });
    }

    #[test]
    fn shared_hotkey_counts_hotkey_collisions() {
        run_on_large_stack(|| {
            let hotkey_q = Hotkey::Letter('Q');
            let first_cell = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
            let second_cell = GridCoordinate::new(ColumnIndex::One, RowIndex::Zero);
            let holy_light_binding = AbilityBinding::builder()
                .button_position(first_cell)
                .hotkey(hotkey_q)
                .build();
            let divine_shield_binding = AbilityBinding::builder()
                .button_position(second_cell)
                .hotkey(hotkey_q)
                .build();
            let mut custom_keys = CustomKeys::from_text("");
            custom_keys.put_ability("AHhb", holy_light_binding);
            custom_keys.put_ability("AHds", divine_shield_binding);
            let layout = GridLayout::qwerty_grid();
            let summary = CollisionSummary::compute(&custom_keys, layout);
            assert!(
                summary.per_unit_hotkey() >= 1,
                "two Paladin abilities on Q must register as a hotkey collision",
            );
        });
    }
}
