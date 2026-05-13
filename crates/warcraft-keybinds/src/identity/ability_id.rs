use warcraft_api::WarcraftObjectId;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AbilityId {
    object_id: WarcraftObjectId,
}

impl AbilityId {
    pub const fn new(id: &'static str) -> Self {
        Self {
            object_id: WarcraftObjectId::new(id),
        }
    }

    pub fn value(&self) -> &'static str {
        self.object_id.value()
    }

    pub fn object_id(&self) -> WarcraftObjectId {
        self.object_id
    }
}

impl From<WarcraftObjectId> for AbilityId {
    fn from(object_id: WarcraftObjectId) -> Self {
        Self { object_id }
    }
}

impl From<AbilityId> for WarcraftObjectId {
    fn from(ability_id: AbilityId) -> Self {
        ability_id.object_id
    }
}

impl From<&'static str> for AbilityId {
    fn from(id: &'static str) -> Self {
        Self {
            object_id: WarcraftObjectId::from(id),
        }
    }
}
