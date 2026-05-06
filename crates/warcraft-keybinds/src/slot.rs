#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum GridSlotId {
    Ability(String),
    AbilityOff(String),
    Command(String),
}

impl GridSlotId {
    pub fn ability(value: impl Into<String>) -> Self {
        Self::Ability(value.into())
    }

    pub fn ability_off(value: impl Into<String>) -> Self {
        Self::AbilityOff(value.into())
    }

    pub fn command(value: impl Into<String>) -> Self {
        Self::Command(value.into())
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Ability(value) => value.as_str(),
            Self::AbilityOff(value) => value.as_str(),
            Self::Command(value) => value.as_str(),
        }
    }
}
