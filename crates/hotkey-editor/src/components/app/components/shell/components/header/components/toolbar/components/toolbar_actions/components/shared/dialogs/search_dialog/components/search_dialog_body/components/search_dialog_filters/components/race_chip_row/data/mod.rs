use warcraft_api::Race;

/// The five races are a closed, game-defined set, so their labels are a match
/// rather than anything fetched. Short forms, because the chips sit five across
/// on a phone: "Night Elf" is clipped where "Elf" is not.
pub(super) fn label(race: Race) -> &'static str {
    match race {
        Race::Human => "Human",
        Race::Orc => "Orc",
        Race::Nightelf => "Elf",
        Race::Undead => "Undead",
        Race::Neutral => "Neutral",
    }
}
