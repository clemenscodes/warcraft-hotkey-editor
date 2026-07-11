use warcraft_api::Race;

/// The lowercase URL slug for a race, the inverse of `Race::try_from(&str)`.
pub trait RaceSlug {
    fn slug(&self) -> &'static str;
}

impl RaceSlug for Race {
    fn slug(&self) -> &'static str {
        match self {
            Race::Human => "human",
            Race::Orc => "orc",
            Race::Nightelf => "nightelf",
            Race::Undead => "undead",
            Race::Neutral => "neutral",
        }
    }
}
