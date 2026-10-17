use warcraft_api::WarcraftObjectId;

/// The run of cards the pager has mounted, as consecutive slots on an endless
/// strip. Slot `s` shows roster unit `s mod roster_len`, so the strip loops in
/// both directions without repeating the roster in the DOM. Slots may run
/// negative once the pager has looped backward past the first unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(super) struct PagerWindow {
    first_slot: isize,
    slot_count: usize,
}

impl PagerWindow {
    /// The window of `2 * buffer + 1` slots centred on `center_slot`, never more
    /// slots than the roster has units, so no unit is mounted twice.
    pub(super) fn centered_on(center_slot: isize, buffer: usize, roster_len: usize) -> Self {
        let wanted_count = buffer.saturating_mul(2).saturating_add(1);
        let slot_count = wanted_count.min(roster_len);
        let half_count = isize::try_from(slot_count / 2).unwrap_or(0);
        let first_slot = center_slot.saturating_sub(half_count);
        Self {
            first_slot,
            slot_count,
        }
    }

    pub(super) fn first_slot(&self) -> isize {
        self.first_slot
    }

    pub(super) fn center_slot(&self) -> isize {
        let half_count = isize::try_from(self.slot_count / 2).unwrap_or(0);
        self.first_slot.saturating_add(half_count)
    }

    pub(super) fn slot_count(&self) -> usize {
        self.slot_count
    }

    /// The same window with `additional` more slots mounted after its last one,
    /// never more slots than the roster has units. Appending below the card on
    /// screen leaves every card above it where it is, so this is safe mid-swipe.
    pub(super) fn extended_forward(&self, additional: usize, roster_len: usize) -> Self {
        let slot_count = self.slot_count.saturating_add(additional).min(roster_len);
        Self {
            first_slot: self.first_slot,
            slot_count,
        }
    }

    pub(super) fn contains(&self, slot: isize) -> bool {
        let count = isize::try_from(self.slot_count).unwrap_or(0);
        let end_slot = self.first_slot.saturating_add(count);
        slot >= self.first_slot && slot < end_slot
    }

    /// The slot rendered by the `offset`th mounted card.
    pub(super) fn slot_at(&self, offset: usize) -> isize {
        let offset_slots = isize::try_from(offset).unwrap_or(0);
        self.first_slot.saturating_add(offset_slots)
    }

    /// The unit every mounted card shows, in DOM order.
    pub(super) fn unit_ids(&self, roster: &[WarcraftObjectId]) -> Vec<WarcraftObjectId> {
        (0..self.slot_count)
            .filter_map(|offset| {
                let slot = self.slot_at(offset);
                let unit_index = RosterSlot::unit_index(slot, roster.len())?;
                roster.get(unit_index).copied()
            })
            .collect()
    }

    /// The slot nearest this window's centre that shows roster unit
    /// `unit_index`, so a jump re-centres on the closest copy of the target and
    /// the slot numbers stay small however long the pager is used.
    pub(super) fn nearest_slot_of(&self, unit_index: usize, roster_len: usize) -> isize {
        let center_slot = self.center_slot();
        let Some(center_unit_index) = RosterSlot::unit_index(center_slot, roster_len) else {
            return center_slot;
        };
        let roster_span = isize::try_from(roster_len).unwrap_or(1).max(1);
        let target_index = isize::try_from(unit_index).unwrap_or(0);
        let center_index = isize::try_from(center_unit_index).unwrap_or(0);
        let forward_distance = (target_index - center_index).rem_euclid(roster_span);
        let shortest_distance = if forward_distance > roster_span / 2 {
            forward_distance - roster_span
        } else {
            forward_distance
        };
        center_slot.saturating_add(shortest_distance)
    }
}

/// Folds a slot on the endless strip back onto the single roster.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(super) struct RosterSlot;

impl RosterSlot {
    pub(super) fn unit_index(slot: isize, roster_len: usize) -> Option<usize> {
        let roster_span = isize::try_from(roster_len).ok().filter(|span| *span > 0)?;
        let folded = slot.rem_euclid(roster_span);
        usize::try_from(folded).ok()
    }
}

/// Where a slot's card must sit, as its top's distance from the pager's top,
/// once the next render has been committed. A jump anchors its target at 0 so
/// the card lands aligned; a re-centre anchors the card on screen at the offset
/// it already has, so adding and dropping cards around it never moves it.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub(super) struct PagerAnchor {
    slot: isize,
    offset_from_top: f64,
}

impl PagerAnchor {
    pub(super) fn new(slot: isize, offset_from_top: f64) -> Self {
        Self {
            slot,
            offset_from_top,
        }
    }

    pub(super) fn aligned(slot: isize) -> Self {
        Self {
            slot,
            offset_from_top: 0.0,
        }
    }

    pub(super) fn slot(&self) -> isize {
        self.slot
    }

    pub(super) fn offset_from_top(&self) -> f64 {
        self.offset_from_top
    }
}
