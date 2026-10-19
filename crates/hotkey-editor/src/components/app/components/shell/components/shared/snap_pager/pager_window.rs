/// The run of cards the pager has mounted, as consecutive slots on an endless
/// strip. Slot `s` shows roster unit `s mod roster_len`, so the strip loops in
/// both directions without repeating the roster in the DOM. Slots may run
/// negative once the pager has looped backward past the first unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(crate) struct PagerWindow {
    first_slot: isize,
    slot_count: usize,
}

impl PagerWindow {
    /// The window of `2 * buffer + 1` slots centred on `center_slot`, never more
    /// slots than the roster has units, so no unit is mounted twice.
    pub(crate) fn centered_on(center_slot: isize, buffer: usize, roster_len: usize) -> Self {
        let wanted_count = buffer.saturating_mul(2).saturating_add(1);
        let slot_count = wanted_count.min(roster_len);
        let half_count = isize::try_from(slot_count / 2).unwrap_or(0);
        let first_slot = center_slot.saturating_sub(half_count);
        Self {
            first_slot,
            slot_count,
        }
    }

    pub(crate) fn first_slot(&self) -> isize {
        self.first_slot
    }

    pub(crate) fn center_slot(&self) -> isize {
        let half_count = isize::try_from(self.slot_count / 2).unwrap_or(0);
        self.first_slot.saturating_add(half_count)
    }

    pub(crate) fn slot_count(&self) -> usize {
        self.slot_count
    }

    /// The same window with up to `additional` more slots mounted after its
    /// last one, never more slots than the roster has units. Appending below the
    /// cards on screen leaves them where they are, so this is safe mid-scroll.
    pub(crate) fn extended_forward(&self, additional: usize, roster_len: usize) -> Self {
        let room = roster_len.saturating_sub(self.slot_count);
        let added = additional.min(room);
        Self {
            first_slot: self.first_slot,
            slot_count: self.slot_count.saturating_add(added),
        }
    }

    /// The same window with up to `additional` more slots mounted before its
    /// first one, never more slots than the roster has units. The cards on
    /// screen move down by what was mounted above them, so the pager puts them
    /// back once the new cards are in.
    pub(crate) fn extended_backward(&self, additional: usize, roster_len: usize) -> Self {
        let room = roster_len.saturating_sub(self.slot_count);
        let added = additional.min(room);
        let added_slots = isize::try_from(added).unwrap_or(0);
        Self {
            first_slot: self.first_slot.saturating_sub(added_slots),
            slot_count: self.slot_count.saturating_add(added),
        }
    }

    /// The slot rendered by the `offset`th mounted card.
    pub(crate) fn slot_at(&self, offset: usize) -> isize {
        let offset_slots = isize::try_from(offset).unwrap_or(0);
        self.first_slot.saturating_add(offset_slots)
    }

    /// The slot nearest this window's centre that shows roster unit
    /// `unit_index`, so a jump re-centres on the closest copy of the target and
    /// the slot numbers stay small however long the pager is used.
    pub(crate) fn nearest_slot_of(&self, unit_index: usize, roster_len: usize) -> isize {
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
pub(crate) struct RosterSlot;

impl RosterSlot {
    pub(crate) fn unit_index(slot: isize, roster_len: usize) -> Option<usize> {
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
pub(crate) struct PagerAnchor {
    slot: isize,
    offset_from_top: f64,
}

impl PagerAnchor {
    pub(crate) fn new(slot: isize, offset_from_top: f64) -> Self {
        Self {
            slot,
            offset_from_top,
        }
    }

    pub(crate) fn aligned(slot: isize) -> Self {
        Self {
            slot,
            offset_from_top: 0.0,
        }
    }

    pub(crate) fn slot(&self) -> isize {
        self.slot
    }

    pub(crate) fn offset_from_top(&self) -> f64 {
        self.offset_from_top
    }
}
