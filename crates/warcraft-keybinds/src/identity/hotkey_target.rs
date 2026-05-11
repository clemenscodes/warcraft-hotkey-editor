use warcraft_api::WarcraftObjectId;

use crate::identity::ability_id::AbilityId;

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
