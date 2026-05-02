use warcraft_api::Race;

pub(crate) const SUPPORTED_RACES: &[Race] = &[
    Race::Human,
    Race::Orc,
    Race::Nightelf,
    Race::Undead,
    Race::Neutral,
];

pub(crate) struct RaceLabels;

impl RaceLabels {
    pub(crate) fn display_name(race: Race) -> &'static str {
        match race {
            Race::Human => "Human",
            Race::Orc => "Orc",
            Race::Nightelf => "Night Elf",
            Race::Undead => "Undead",
            Race::Neutral => "Neutral",
        }
    }

    pub(crate) fn data_attribute(race: Race) -> &'static str {
        match race {
            Race::Human => "human",
            Race::Orc => "orc",
            Race::Nightelf => "nightelf",
            Race::Undead => "undead",
            Race::Neutral => "neutral",
        }
    }

    pub(crate) fn build_command(race: Option<Race>) -> Option<&'static str> {
        let race_value = race?;
        let preferred_name = match race_value {
            Race::Human => "CmdBuildHuman",
            Race::Orc => "CmdBuildOrc",
            Race::Nightelf => "CmdBuildNightElf",
            Race::Undead => "CmdBuildUndead",
            Race::Neutral => "CmdBuild",
        };
        let preferred_match =
            crate::domain::command_catalog::CommandCatalog::known_command(preferred_name);
        if preferred_match.is_some() {
            return preferred_match;
        }
        crate::domain::command_catalog::CommandCatalog::known_command("CmdBuild")
    }
}
