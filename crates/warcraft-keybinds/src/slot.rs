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
            Self::Ability(value) | Self::AbilityOff(value) | Self::Command(value) => value.as_str(),
        }
    }
}

impl AsRef<str> for GridSlotId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::ops::Deref for GridSlotId {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}
