use warcraft_api::Race;

/// The selected/hover accent a selectable entity card wears. One variant per race
/// (the editor's unit cards take the active race's colour) plus the fixed gold
/// accent the collision sidebars use. Chosen at the wrapper and handed to the card
/// as a prop, so the shared surface owns every accent look in one `states!` table.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum CardAccent {
    Human,
    Orc,
    Undead,
    Nightelf,
    Neutral,
    CollisionGold,
}

impl From<Race> for CardAccent {
    fn from(race: Race) -> Self {
        match race {
            Race::Human => Self::Human,
            Race::Orc => Self::Orc,
            Race::Undead => Self::Undead,
            Race::Nightelf => Self::Nightelf,
            Race::Neutral => Self::Neutral,
        }
    }
}
