use dioxus::prelude::*;
use warcraft_api::DefenseType;

/// The defense type row's input: the unit's defense classification.
#[derive(Props, Clone, PartialEq)]
pub struct DefenseTypeRowProps {
    pub value: DefenseType,
}
