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

impl ddd::Layered for AbilityId {
    type Layer = ddd::DomainLayer;
}

impl ddd::ValueObject for AbilityId {}

impl ddd::Identifier for AbilityId {}

#[cfg(test)]
mod ddd_marker_tests {
    use super::AbilityId;
    use crate::ddd_conformance::assert_identifier;

    #[test]
    fn ability_id_is_an_identifier() {
        assert_identifier::<AbilityId>();
    }
}
