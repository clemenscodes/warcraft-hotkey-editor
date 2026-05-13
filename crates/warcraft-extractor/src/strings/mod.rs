use std::collections::BTreeMap;

use crate::ParsedEntry;

pub type RaceAbilityStringsDatabase = BTreeMap<String, AbilityStringDefinition>;
pub type RaceUnitStringsDatabase = BTreeMap<String, UnitStringDefinition>;

#[derive(Debug, Clone, Default)]
pub struct AbilityStringDefinition {
    name: String,
    tip: Option<String>,
    ubertip: Option<String>,
    research_ubertip: Option<String>,
}

impl AbilityStringDefinition {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tip: None,
            ubertip: None,
            research_ubertip: None,
        }
    }

    pub fn with_text(
        name: String,
        tip: Option<String>,
        ubertip: Option<String>,
        research_ubertip: Option<String>,
    ) -> Self {
        Self {
            name,
            tip,
            ubertip,
            research_ubertip,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.name
    }

    pub fn tip(&self) -> Option<&str> {
        self.tip.as_deref()
    }

    pub fn ubertip(&self) -> Option<&str> {
        self.ubertip.as_deref()
    }

    pub fn research_ubertip(&self) -> Option<&str> {
        self.research_ubertip.as_deref()
    }
}

impl From<ParsedEntry> for AbilityStringDefinition {
    fn from(value: ParsedEntry) -> Self {
        Self::new(value.values().first().unwrap().to_string())
    }
}

pub fn parse_ability_strings_file(text: &str) -> RaceAbilityStringsDatabase {
    let mut database: RaceAbilityStringsDatabase = BTreeMap::new();
    let mut current_id: Option<String> = None;
    let mut current_name: Option<String> = None;
    let mut current_tip: Option<String> = None;
    let mut current_ubertip: Option<String> = None;
    let mut current_research_ubertip: Option<String> = None;
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        if let Some(inner) = line
            .strip_prefix('[')
            .and_then(|rest| rest.strip_suffix(']'))
        {
            flush_ability_string_entry(
                &mut database,
                &mut current_id,
                &mut current_name,
                &mut current_tip,
                &mut current_ubertip,
                &mut current_research_ubertip,
            );
            current_id = Some(inner.to_string());
            continue;
        }
        let Some((key_raw, value_raw)) = line.split_once('=') else {
            continue;
        };
        let key_trimmed = key_raw.trim();
        let key_lower = key_trimmed.to_ascii_lowercase();
        let value_trimmed = value_raw.trim();
        let first_value = first_quoted_value(value_trimmed);
        match key_lower.as_str() {
            "name" if current_name.is_none() => {
                current_name = Some(first_value);
            }
            "tip" if current_tip.is_none() => {
                current_tip = Some(first_value);
            }
            "ubertip" if current_ubertip.is_none() => {
                current_ubertip = Some(first_value);
            }
            "researchubertip" if current_research_ubertip.is_none() => {
                current_research_ubertip = Some(first_value);
            }
            _ => {}
        }
    }
    flush_ability_string_entry(
        &mut database,
        &mut current_id,
        &mut current_name,
        &mut current_tip,
        &mut current_ubertip,
        &mut current_research_ubertip,
    );
    database
}

fn flush_ability_string_entry(
    database: &mut RaceAbilityStringsDatabase,
    current_id: &mut Option<String>,
    current_name: &mut Option<String>,
    current_tip: &mut Option<String>,
    current_ubertip: &mut Option<String>,
    current_research_ubertip: &mut Option<String>,
) {
    let Some(id) = current_id.take() else {
        current_name.take();
        current_tip.take();
        current_ubertip.take();
        current_research_ubertip.take();
        return;
    };
    let name = current_name.take().unwrap_or_default();
    let tip = current_tip.take();
    let ubertip = current_ubertip.take();
    let research_ubertip = current_research_ubertip.take();
    if name.trim().is_empty() {
        let Some(tip_as_name) = tip else {
            return;
        };
        if tip_as_name.trim().is_empty() {
            return;
        }
        let entry =
            AbilityStringDefinition::with_text(tip_as_name, None, ubertip, research_ubertip);
        database.insert(id, entry);
        return;
    }
    let entry = AbilityStringDefinition::with_text(name, tip, ubertip, research_ubertip);
    database.insert(id, entry);
}

fn first_quoted_value(input: &str) -> String {
    let trimmed = input.trim();
    if let Some(without_open_quote) = trimmed.strip_prefix('"') {
        if let Some(close_index) = without_open_quote.find('"') {
            return without_open_quote[..close_index].to_string();
        }
        return without_open_quote.to_string();
    }
    if let Some(comma_index) = trimmed.find(',') {
        return trimmed[..comma_index].trim().to_string();
    }
    trimmed.to_string()
}

#[derive(Debug, Clone, Default)]
pub struct UnitStringDefinition {
    name: String,
    tip: Option<String>,
    ubertip: Option<String>,
}

impl UnitStringDefinition {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tip: None,
            ubertip: None,
        }
    }

    pub fn with_text(name: String, tip: Option<String>, ubertip: Option<String>) -> Self {
        Self { name, tip, ubertip }
    }

    pub fn value(&self) -> &str {
        &self.name
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tip(&self) -> Option<&str> {
        self.tip.as_deref()
    }

    pub fn ubertip(&self) -> Option<&str> {
        self.ubertip.as_deref()
    }
}

impl From<ParsedEntry> for UnitStringDefinition {
    fn from(value: ParsedEntry) -> Self {
        Self::new(value.values().first().unwrap().to_string())
    }
}

pub fn parse_unit_strings_file(text: &str) -> RaceUnitStringsDatabase {
    let mut database: RaceUnitStringsDatabase = BTreeMap::new();
    let mut current_id: Option<String> = None;
    let mut current_name: Option<String> = None;
    let mut current_tip: Option<String> = None;
    let mut current_ubertip: Option<String> = None;
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        if let Some(inner) = line
            .strip_prefix('[')
            .and_then(|rest| rest.strip_suffix(']'))
        {
            flush_unit_string_entry(
                &mut database,
                &mut current_id,
                &mut current_name,
                &mut current_tip,
                &mut current_ubertip,
            );
            current_id = Some(inner.to_string());
            continue;
        }
        let Some((key_raw, value_raw)) = line.split_once('=') else {
            continue;
        };
        let key_lower = key_raw.trim().to_ascii_lowercase();
        let value_first = first_quoted_value(value_raw.trim());
        match key_lower.as_str() {
            "name" if current_name.is_none() => {
                current_name = Some(value_first);
            }
            "tip" if current_tip.is_none() => {
                current_tip = Some(value_first);
            }
            "ubertip" if current_ubertip.is_none() => {
                current_ubertip = Some(value_first);
            }
            _ => {}
        }
    }
    flush_unit_string_entry(
        &mut database,
        &mut current_id,
        &mut current_name,
        &mut current_tip,
        &mut current_ubertip,
    );
    database
}

fn flush_unit_string_entry(
    database: &mut RaceUnitStringsDatabase,
    current_id: &mut Option<String>,
    current_name: &mut Option<String>,
    current_tip: &mut Option<String>,
    current_ubertip: &mut Option<String>,
) {
    let Some(id) = current_id.take() else {
        current_name.take();
        current_tip.take();
        current_ubertip.take();
        return;
    };
    let name = current_name.take().unwrap_or_default();
    let tip = current_tip.take();
    let ubertip = current_ubertip.take();
    if name.trim().is_empty() && tip.is_none() && ubertip.is_none() {
        return;
    }
    database.insert(id, UnitStringDefinition::with_text(name, tip, ubertip));
}

macro_rules! race_strings {
    (
        race = $Race:ident,
        abilityprefix = $abilityprefix:literal,
        unitprefix = $unitprefix:literal
    ) => {
        paste::paste! {
            pub type [<$Race AbilityStringsDatabase>] = RaceAbilityStringsDatabase;
            pub type [<$Race UnitStringsDatabase>] = RaceUnitStringsDatabase;

            pub struct [<$Race StringsExtraction>];

            impl [<$Race StringsExtraction>] {
                pub fn matches_abilities(path: &str) -> bool {
                    path.ends_with(concat!($abilityprefix, "abilitystrings.txt"))
                        && path.contains("enus.w3mod:units")
                }

                pub fn process_abilities(
                    path: &str,
                    bytes: &[u8],
                ) -> Result<$crate::ExtractResult, $crate::ExtractError> {
                    tracing::debug!(
                        "Processing {} ability strings with {}",
                        $abilityprefix,
                        path
                    );
                    let text = std::str::from_utf8(bytes)
                        .map_err(|_| {
                            std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Invalid UTF-8",
                            )
                        })?;

                    let database = match $crate::casc_filename(path).as_str() {
                        concat!($abilityprefix, "abilitystrings.txt") => {
                            $crate::strings::parse_ability_strings_file(text)
                        }
                        _ => return Err($crate::ExtractError::Heroes),
                    };

                    Ok($crate::ExtractResult::[<$Race AbilityStrings>](database))
                }

                pub fn matches_units(path: &str) -> bool {
                    path.ends_with(concat!($unitprefix, "strings.txt"))
                        && path.contains("enus.w3mod:units")
                }

                pub fn process_units(
                    path: &str,
                    bytes: &[u8],
                ) -> Result<$crate::ExtractResult, $crate::ExtractError> {
                    tracing::debug!(
                        "Processing {} unit strings with {}",
                        $unitprefix,
                        path
                    );
                    let text = std::str::from_utf8(bytes)
                        .map_err(|_| {
                            std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Invalid UTF-8",
                            )
                        })?;

                    let database = match $crate::casc_filename(path).as_str() {
                        concat!($unitprefix, "strings.txt") => {
                            $crate::strings::parse_unit_strings_file(text)
                        }
                        _ => return Err($crate::ExtractError::Heroes),
                    };

                    Ok($crate::ExtractResult::[<$Race UnitStrings>](database))
                }
            }

            pub static [<$Race:upper _ABILITY_STRINGS_EXTRACTION_RULE>]: $crate::ExtractionRule =
                $crate::ExtractionRule {
                    matcher: [<$Race StringsExtraction>]::matches_abilities,
                    target: $crate::ExtractTarget::Text,
                    output_path: |_, _| std::path::PathBuf::new(),
                    processor: [<$Race StringsExtraction>]::process_abilities,
                };

            pub static [<$Race:upper _UNIT_STRINGS_EXTRACTION_RULE>]: $crate::ExtractionRule =
                $crate::ExtractionRule {
                    matcher: [<$Race StringsExtraction>]::matches_units,
                    target: $crate::ExtractTarget::Text,
                    output_path: |_, _| std::path::PathBuf::new(),
                    processor: [<$Race StringsExtraction>]::process_units,
                };
        }
    };
}

race_strings!(
    race = Human,
    abilityprefix = "human",
    unitprefix = "humanunit"
);
race_strings!(
    race = Nightelf,
    abilityprefix = "nightelf",
    unitprefix = "nightelfunit"
);
race_strings!(race = Orc, abilityprefix = "orc", unitprefix = "orcunit");
race_strings!(
    race = Undead,
    abilityprefix = "undead",
    unitprefix = "undeadunit"
);
race_strings!(
    race = Neutral,
    abilityprefix = "neutral",
    unitprefix = "neutralunit"
);
race_strings!(race = Item, abilityprefix = "item", unitprefix = "item");
race_strings!(
    race = Campaign,
    abilityprefix = "campaign",
    unitprefix = "campaignunit"
);
race_strings!(
    race = Common,
    abilityprefix = "common",
    unitprefix = "common"
);
