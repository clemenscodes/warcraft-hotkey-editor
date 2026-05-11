use crate::object::Race;

pub const SUPPORTED_RACES: &[Race] = &[
    Race::Human,
    Race::Orc,
    Race::Nightelf,
    Race::Undead,
    Race::Neutral,
];

pub struct RaceLabels;

impl RaceLabels {
    pub fn display_name(race: Race) -> &'static str {
        match race {
            Race::Human => "Human",
            Race::Orc => "Orc",
            Race::Nightelf => "Night Elf",
            Race::Undead => "Undead",
            Race::Neutral => "Neutral",
        }
    }

    pub fn data_attribute(race: Race) -> &'static str {
        match race {
            Race::Human => "human",
            Race::Orc => "orc",
            Race::Nightelf => "nightelf",
            Race::Undead => "undead",
            Race::Neutral => "neutral",
        }
    }
}
