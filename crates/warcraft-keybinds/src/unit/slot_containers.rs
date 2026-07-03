//! The four command-card containers a unit can present (command card, build
//! menu, uprooted menu, research menu) plus its train-unit upgrade map, resolved
//! once from the database. The renderer used to orchestrate these five
//! [`UnitCommandSlots`] queries and shape the result itself at render time; that
//! is domain work (ARCHITECTURE R3), so it lives here behind one call.

use crate::identity::slot::GridSlotId;
use crate::unit::slots::UnitCommandSlots;
use std::collections::HashMap;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_database::WARCRAFT_DATABASE;

/// A unit's resolved command containers and its train-unit upgrade map. Cheap to
/// clone (the slot lists are reference-counted), so a renderer can memoise it on
/// the selected unit id.
#[derive(Clone, Debug, PartialEq)]
pub struct UnitSlotContainers {
    command_card: Rc<[GridSlotId]>,
    build_menu: Option<Rc<[GridSlotId]>>,
    uprooted: Option<Rc<[GridSlotId]>>,
    research: Option<Rc<[GridSlotId]>>,
    train_upgrades: HashMap<WarcraftObjectId, WarcraftObjectId>,
}

impl UnitSlotContainers {
    /// Resolve every container and the upgrade map for the unit with the given id.
    /// An unknown id resolves to the default object, whose command card is empty.
    pub fn resolve(unit_id: &str) -> Self {
        let unit_object_id = WARCRAFT_DATABASE
            .by_id_and_key(unit_id)
            .map(|(object_id, _key)| object_id)
            .unwrap_or_default();
        let command_card: Rc<[GridSlotId]> = WARCRAFT_DATABASE
            .command_card(unit_object_id)
            .filled_slots()
            .collect();
        let build_menu: Option<Rc<[GridSlotId]>> = WARCRAFT_DATABASE
            .build_menu(unit_object_id)
            .map(|card| card.filled_slots().collect());
        let uprooted: Option<Rc<[GridSlotId]>> = WARCRAFT_DATABASE
            .uprooted_menu(unit_object_id)
            .map(|card| card.filled_slots().collect());
        let research: Option<Rc<[GridSlotId]>> = WARCRAFT_DATABASE
            .research_menu(unit_object_id)
            .map(|card| card.filled_slots().collect());
        let train_upgrades = WARCRAFT_DATABASE.train_unit_upgrades(unit_object_id);
        Self {
            command_card,
            build_menu,
            uprooted,
            research,
            train_upgrades,
        }
    }

    pub fn command_card(&self) -> Rc<[GridSlotId]> {
        self.command_card.clone()
    }

    pub fn build_menu(&self) -> Option<Rc<[GridSlotId]>> {
        self.build_menu.clone()
    }

    pub fn uprooted(&self) -> Option<Rc<[GridSlotId]>> {
        self.uprooted.clone()
    }

    pub fn research(&self) -> Option<Rc<[GridSlotId]>> {
        self.research.clone()
    }

    pub fn train_upgrades(&self) -> &HashMap<WarcraftObjectId, WarcraftObjectId> {
        &self.train_upgrades
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn footman_has_a_non_empty_command_card() {
        let containers = UnitSlotContainers::resolve("hfoo");
        assert!(
            !containers.command_card().is_empty(),
            "the footman (hfoo) should have a populated command card"
        );
    }

    #[test]
    fn unknown_unit_resolves_to_empty_command_card() {
        let containers = UnitSlotContainers::resolve("this-is-not-a-real-unit-id");
        assert!(containers.command_card().is_empty());
        assert!(containers.build_menu().is_none());
    }
}
