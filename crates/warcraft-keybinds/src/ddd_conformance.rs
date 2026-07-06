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

#[cfg(test)]
mod tests {
    use super::assert_identifier;
    use super::assert_read_model;
    use super::assert_value_object;
    use ddd::DomainLayer;
    use ddd::Layered;
    use ddd::ReadModel;
    use ddd::ValueObject;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    struct ConformantMarker;

    impl Layered for ConformantMarker {
        type Layer = DomainLayer;
    }

    impl ValueObject for ConformantMarker {}

    impl ddd::Identifier for ConformantMarker {}

    impl ReadModel for ConformantMarker {}

    #[test]
    fn harness_accepts_a_conformant_type() {
        assert_value_object::<ConformantMarker>();
        assert_identifier::<ConformantMarker>();
        assert_read_model::<ConformantMarker>();
    }
}
