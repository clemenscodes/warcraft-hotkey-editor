use crate::identity::ability_id::AbilityId;
use warcraft_api::WarcraftObjectId;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HotkeyTarget {
    Ability(AbilityId),
    AbilityResearch(AbilityId),
    AbilityOffState(AbilityId),
    Command(WarcraftObjectId),
}

impl HotkeyTarget {
    pub fn ability(id: impl Into<AbilityId>) -> Self {
        Self::Ability(id.into())
    }

    pub fn ability_research(id: impl Into<AbilityId>) -> Self {
        Self::AbilityResearch(id.into())
    }

    pub fn ability_off_state(id: impl Into<AbilityId>) -> Self {
        Self::AbilityOffState(id.into())
    }

    pub fn command(id: impl Into<WarcraftObjectId>) -> Self {
        Self::Command(id.into())
    }
}

impl ddd::Layered for HotkeyTarget {
    type Layer = ddd::DomainLayer;
}

impl ddd::ValueObject for HotkeyTarget {}

#[cfg(test)]
mod ddd_marker_tests {
    use super::HotkeyTarget;
    use crate::ddd_conformance::assert_value_object;

    #[test]
    fn hotkey_target_is_a_value_object() {
        assert_value_object::<HotkeyTarget>();
    }
}
