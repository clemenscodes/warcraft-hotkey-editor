use dioxus::prelude::*;
use warcraft_api::AttackType;

/// The attack type row's input: the unit's attack classification.
#[derive(Props, Clone, PartialEq)]
pub struct AttackTypeRowProps {
    pub value: AttackType,
}
