use ddd::ApplicationLayer;
use ddd::Layered;
use ddd::Query;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::GridSlotId;
use warcraft_keybinds::InspectorDetail;
use warcraft_keybinds::UnitSlotContainers;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnitOverrideTargetView {
    detail: Option<InspectorDetail>,
    active_container_slots: Rc<[GridSlotId]>,
}

impl UnitOverrideTargetView {
    pub(crate) fn detail(&self) -> Option<InspectorDetail> {
        self.detail.clone()
    }

    pub(crate) fn active_container_slots(&self) -> Rc<[GridSlotId]> {
        Rc::clone(&self.active_container_slots)
    }
}

impl Default for UnitOverrideTargetView {
    fn default() -> Self {
        let empty_slots: Vec<GridSlotId> = Vec::new();
        let active_container_slots: Rc<[GridSlotId]> = Rc::from(empty_slots);
        Self {
            detail: None,
            active_container_slots,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UnitOverrideTargetRequest {
    pub(crate) unit_id: WarcraftObjectId,
    pub(crate) selected_slot: Option<GridSlotId>,
    pub(crate) from_uprooted: bool,
    pub(crate) from_research: bool,
}

/// Answers "what does the hotkey override panel show for this unit?".
///
/// The selection it reads is app-wide, but a mobile pager mounts several unit
/// cards at once, so the query only reports a detail when the selected slot
/// actually belongs to the requested unit. That guard is what stops a card
/// building an override against a host unit that does not own the slot.
pub struct UnitOverrideTargetQuery {
    request: UnitOverrideTargetRequest,
}

impl UnitOverrideTargetQuery {
    pub fn new(request: UnitOverrideTargetRequest) -> Self {
        Self { request }
    }

    pub fn answer(&self, custom_keys: &Option<CustomKeys>) -> UnitOverrideTargetView {
        let unit_id = self.request.unit_id;
        let containers = UnitSlotContainers::resolve(unit_id);
        let owned_slot = self.owned_slot(&containers);
        let active_container_slots = self.active_container_slots(&containers, &owned_slot);
        let detail = self.detail(&containers, &owned_slot, custom_keys);
        UnitOverrideTargetView {
            detail,
            active_container_slots,
        }
    }

    fn owned_slot(&self, containers: &UnitSlotContainers) -> Option<GridSlotId> {
        let selected_slot = self.request.selected_slot?;
        let is_owned = Self::contains_slot(containers, &selected_slot);
        if is_owned {
            return Some(selected_slot);
        }
        None
    }

    fn contains_slot(containers: &UnitSlotContainers, slot: &GridSlotId) -> bool {
        let command_card_slots = containers.command_card();
        let in_command_card = command_card_slots.contains(slot);
        if in_command_card {
            return true;
        }
        let in_build_menu = containers.build_menu_contains(slot);
        if in_build_menu {
            return true;
        }
        let uprooted_menu_slots = containers.uprooted();
        let in_uprooted_menu = Self::optional_contains(uprooted_menu_slots, slot);
        if in_uprooted_menu {
            return true;
        }
        let research_menu_slots = containers.research();
        Self::optional_contains(research_menu_slots, slot)
    }

    fn optional_contains(slots: Option<Rc<[GridSlotId]>>, slot: &GridSlotId) -> bool {
        match slots {
            Some(present_slots) => present_slots.contains(slot),
            None => false,
        }
    }

    fn active_container_slots(
        &self,
        containers: &UnitSlotContainers,
        owned_slot: &Option<GridSlotId>,
    ) -> Rc<[GridSlotId]> {
        let empty_slots: Vec<GridSlotId> = Vec::new();
        let empty_slot_list: Rc<[GridSlotId]> = Rc::from(empty_slots);
        if self.request.from_uprooted {
            let uprooted_menu_slots = containers.uprooted();
            return uprooted_menu_slots.unwrap_or(empty_slot_list);
        }
        if self.request.from_research {
            let research_menu_slots = containers.research();
            return research_menu_slots.unwrap_or(empty_slot_list);
        }
        let in_build_menu = match owned_slot {
            Some(slot) => containers.build_menu_contains(slot),
            None => false,
        };
        if in_build_menu {
            let build_menu_slots = containers.build_menu();
            return build_menu_slots.unwrap_or(empty_slot_list);
        }
        containers.command_card()
    }

    fn detail(
        &self,
        containers: &UnitSlotContainers,
        owned_slot: &Option<GridSlotId>,
        custom_keys: &Option<CustomKeys>,
    ) -> Option<InspectorDetail> {
        let slot = owned_slot.as_ref()?;
        let upgrade_unit_id = Self::upgrade_unit_id(containers, slot);
        let host_unit_id = self.request.unit_id;
        let from_uprooted = self.request.from_uprooted;
        let from_research = self.request.from_research;
        let detail = InspectorDetail::build(
            slot,
            custom_keys,
            host_unit_id,
            from_uprooted,
            from_research,
            upgrade_unit_id,
        );
        Some(detail)
    }

    fn upgrade_unit_id(
        containers: &UnitSlotContainers,
        slot: &GridSlotId,
    ) -> Option<WarcraftObjectId> {
        let GridSlotId::Ability(ability_id) = slot else {
            return None;
        };
        let train_upgrades = containers.train_upgrades();
        let ability_object_id = ability_id.object_id();
        train_upgrades.get(&ability_object_id).copied()
    }
}

impl Layered for UnitOverrideTargetQuery {
    type Layer = ApplicationLayer;
}

impl Query for UnitOverrideTargetQuery {
    type Output = UnitOverrideTargetView;
}

#[cfg(test)]
mod tests {
    use super::UnitOverrideTargetQuery;
    use super::UnitOverrideTargetRequest;
    use crate::services::customkeys::queries::assert_query;
    use warcraft_api::WarcraftApi;
    use warcraft_keybinds::UnitSlotContainers;

    fn first_unit_with_command_card() -> warcraft_api::WarcraftObjectId {
        let api = WarcraftApi::default();
        api.unit()
            .all()
            .map(|unit| unit.id())
            .find(|unit_id| {
                let containers = UnitSlotContainers::resolve(*unit_id);
                let command_card = containers.command_card();
                !command_card.is_empty()
            })
            .expect("at least one unit has a command card")
    }

    #[test]
    fn unit_override_target_is_a_query() {
        assert_query::<UnitOverrideTargetQuery>();
    }

    #[test]
    fn reports_no_detail_when_nothing_is_selected() {
        let unit_id = first_unit_with_command_card();
        let request = UnitOverrideTargetRequest {
            unit_id,
            selected_slot: None,
            from_uprooted: false,
            from_research: false,
        };
        let query = UnitOverrideTargetQuery::new(request);
        let view = query.answer(&None);
        assert!(view.detail().is_none());
    }

    #[test]
    fn reports_a_detail_for_a_slot_the_unit_owns() {
        let unit_id = first_unit_with_command_card();
        let containers = UnitSlotContainers::resolve(unit_id);
        let command_card = containers.command_card();
        let owned_slot = *command_card.first().expect("command card is not empty");
        let request = UnitOverrideTargetRequest {
            unit_id,
            selected_slot: Some(owned_slot),
            from_uprooted: false,
            from_research: false,
        };
        let query = UnitOverrideTargetQuery::new(request);
        let view = query.answer(&None);
        assert!(view.detail().is_some());
    }

    #[test]
    fn reports_no_detail_for_a_slot_the_unit_does_not_own() {
        let api = WarcraftApi::default();
        let host_unit_id = first_unit_with_command_card();
        let host_containers = UnitSlotContainers::resolve(host_unit_id);
        let foreign_slot = api
            .unit()
            .all()
            .map(|unit| unit.id())
            .filter(|unit_id| *unit_id != host_unit_id)
            .filter_map(|unit_id| {
                let containers = UnitSlotContainers::resolve(unit_id);
                let command_card = containers.command_card();
                command_card.first().copied()
            })
            .find(|slot| !UnitOverrideTargetQuery::contains_slot(&host_containers, slot))
            .expect("some other unit owns a slot this host does not");
        let request = UnitOverrideTargetRequest {
            unit_id: host_unit_id,
            selected_slot: Some(foreign_slot),
            from_uprooted: false,
            from_research: false,
        };
        let query = UnitOverrideTargetQuery::new(request);
        let view = query.answer(&None);
        assert!(
            view.detail().is_none(),
            "a card must not build an override for a slot its unit does not own"
        );
    }
}
