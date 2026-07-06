//! Compile-time conformance assertions for the `ddd` role vocabulary.
//!
//! Each helper is a generic function whose `where` bound is the `ddd`
//! trait's own contract. Calling `assert_value_object::<Hotkey>()` from a
//! test fails to compile unless `Hotkey` genuinely implements
//! `ddd::ValueObject`. These are the domain-crate analogue of the existing
//! `assert_domain_aggregate` pattern, generalized to every role this crate
//! adopts. The whole module is compiled only under `#[cfg(test)]` (see the
//! declaration in `lib.rs`), so the assertions add no cost to the shipped
//! crate.

pub(crate) fn assert_value_object<Type>()
where
    Type: ddd::ValueObject,
{
}

pub(crate) fn assert_identifier<Type>()
where
    Type: ddd::Identifier,
{
}

pub(crate) fn assert_read_model<Type>()
where
    Type: ddd::ReadModel,
{
}

pub(crate) fn assert_entity<Type>()
where
    Type: ddd::Entity,
{
}

pub(crate) fn assert_domain_service<Type>()
where
    Type: ddd::DomainService,
{
}

pub(crate) fn assert_factory<Product, TheFactory>()
where
    TheFactory: ddd::Factory<Product>,
{
}

pub(crate) fn assert_specification<Candidate, TheSpecification>()
where
    TheSpecification: ddd::Specification<Candidate>,
{
}

#[cfg(test)]
mod tests {
    use super::assert_domain_service;
    use super::assert_entity;
    use super::assert_factory;
    use super::assert_identifier;
    use super::assert_read_model;
    use super::assert_specification;
    use super::assert_value_object;
    use ddd::DomainLayer;
    use ddd::DomainService;
    use ddd::Entity;
    use ddd::Factory;
    use ddd::Layered;
    use ddd::ReadModel;
    use ddd::Specification;
    use ddd::ValueObject;

    /// A single fixture that deliberately satisfies every domain role, purely
    /// to prove each assertion helper is wired to the right `ddd` bound. Real
    /// domain types carry only the one role that fits them.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    struct ConformantMarker;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    struct EmptyBlueprint;

    impl Layered for ConformantMarker {
        type Layer = DomainLayer;
    }

    impl ValueObject for ConformantMarker {}

    impl ddd::Identifier for ConformantMarker {}

    impl ReadModel for ConformantMarker {}

    impl DomainService for ConformantMarker {}

    impl Entity for ConformantMarker {
        type Identity = Self;

        fn identity(&self) -> &Self::Identity {
            self
        }
    }

    impl Factory<Self> for ConformantMarker {
        type Blueprint = EmptyBlueprint;

        fn create(&self, _blueprint: Self::Blueprint) -> Self {
            *self
        }
    }

    impl Specification<Self> for ConformantMarker {
        fn is_satisfied_by(&self, _candidate: &Self) -> bool {
            true
        }
    }

    #[test]
    fn harness_accepts_a_conformant_type() {
        assert_value_object::<ConformantMarker>();
        assert_identifier::<ConformantMarker>();
        assert_read_model::<ConformantMarker>();
        assert_entity::<ConformantMarker>();
        assert_domain_service::<ConformantMarker>();
        assert_factory::<ConformantMarker, ConformantMarker>();
        assert_specification::<ConformantMarker, ConformantMarker>();
    }
}
