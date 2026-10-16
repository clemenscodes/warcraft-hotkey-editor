use std::collections::BTreeSet;

pub type TalentVariantDatabase = BTreeSet<TalentVariantEntry>;

/// One hero talent (patch 3.0.0) paired with the hero ability it replaces on
/// the command card, e.g. `AHq1` (Guiding Hand, talent 1) with its base `AHgh`.
/// Ordered by `variant_id` first so the emitted table is deterministic across
/// regenerations.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TalentVariantEntry {
    variant_id: String,
    base_id: String,
}

impl TalentVariantEntry {
    pub fn new(variant_id: String, base_id: String) -> Self {
        Self {
            variant_id,
            base_id,
        }
    }

    pub fn variant_id(&self) -> &str {
        &self.variant_id
    }

    pub fn base_id(&self) -> &str {
        &self.base_id
    }
}
