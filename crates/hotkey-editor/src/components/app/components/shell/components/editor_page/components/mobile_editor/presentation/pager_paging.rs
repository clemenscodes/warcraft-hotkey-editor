use warcraft_api::{Race, WarcraftObjectId};

// Units side by side on one page on a laptop, where a single card would leave
// half the screen empty.
const LAPTOP_UNITS_PER_PAGE: usize = 2;
const NARROW_UNITS_PER_PAGE: usize = 1;

/// How the roster folds into the pager's pages: one unit per page on a phone or
/// a tablet, two side by side on a laptop. Every race starts on a page of its
/// own, so a race's first unit never shares a page with the last unit of the
/// race before it. A slot of the pager window is a page, so every swipe turns a
/// whole page.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(super) struct PagerPaging {
    units_per_page: usize,
}

impl PagerPaging {
    pub(super) fn new(is_laptop: bool) -> Self {
        let units_per_page = if is_laptop {
            LAPTOP_UNITS_PER_PAGE
        } else {
            NARROW_UNITS_PER_PAGE
        };
        Self { units_per_page }
    }

    /// The index of each page's first unit, given the race of every unit in
    /// roster order.
    fn page_starts(&self, races: &[Race]) -> Vec<usize> {
        let mut starts = Vec::new();
        let mut units_on_page = 0;
        let mut previous_race = None;
        for (unit_index, race) in races.iter().enumerate() {
            let race_changed = previous_race.is_some_and(|previous| previous != *race);
            if units_on_page == 0 || units_on_page == self.units_per_page || race_changed {
                starts.push(unit_index);
                units_on_page = 0;
            }
            units_on_page += 1;
            previous_race = Some(*race);
        }
        starts
    }

    pub(super) fn page_count(&self, races: &[Race]) -> usize {
        self.page_starts(races).len()
    }

    pub(super) fn page_of(&self, unit_index: usize, races: &[Race]) -> usize {
        let starts = self.page_starts(races);
        starts
            .iter()
            .rposition(|start| *start <= unit_index)
            .unwrap_or(0)
    }

    /// The unit a settle on `page_index` selects: the first one on the page.
    pub(super) fn first_unit_of(&self, page_index: usize, races: &[Race]) -> usize {
        let starts = self.page_starts(races);
        starts.get(page_index).copied().unwrap_or(0)
    }

    /// The units `page_index` shows, in roster order. A race's last page may
    /// hold fewer than a full page.
    pub(super) fn units_of(
        &self,
        page_index: usize,
        roster: &[WarcraftObjectId],
        races: &[Race],
    ) -> Vec<WarcraftObjectId> {
        let starts = self.page_starts(races);
        let Some(first_unit) = starts.get(page_index).copied() else {
            return Vec::new();
        };
        let end = starts.get(page_index + 1).copied().unwrap_or(roster.len());
        roster
            .iter()
            .skip(first_unit)
            .take(end - first_unit)
            .copied()
            .collect()
    }
}
