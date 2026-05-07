use warcraft_api::WarcraftObjectId;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GridSlotId {
    Ability(WarcraftObjectId),
    AbilityOff(WarcraftObjectId),
    Command(WarcraftObjectId),
}

impl GridSlotId {
    pub fn ability(id: impl Into<WarcraftObjectId>) -> Self {
        Self::Ability(id.into())
    }

    pub fn ability_off(id: impl Into<WarcraftObjectId>) -> Self {
        Self::AbilityOff(id.into())
    }

    pub fn command(id: impl Into<WarcraftObjectId>) -> Self {
        Self::Command(id.into())
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ability(id) | Self::AbilityOff(id) | Self::Command(id) => id.value(),
        }
    }

    pub fn id(&self) -> WarcraftObjectId {
        match self {
            Self::Ability(id) | Self::AbilityOff(id) | Self::Command(id) => *id,
        }
    }
}
