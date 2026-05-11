mod human;
mod nightelf;
mod orc;
mod undead;

use std::collections::BTreeMap;

pub use human::*;
pub use nightelf::*;
pub use orc::*;
pub use undead::*;

use warcraft_api::Race;

use crate::{ParsedEntry, SectionedListParser};

pub type RaceUpgradeDatabase = BTreeMap<String, UpgradeDefinition>;
pub type RaceUpgradeArtDatabase = BTreeMap<String, UpgradeArtDefinition>;
pub type RaceUpgradeNameDatabase = BTreeMap<String, UpgradeNameDefinition>;

pub struct RaceUpgradeEntry<'a> {
    race: Race,
    art_database: &'a RaceUpgradeArtDatabase,
    name_database: &'a RaceUpgradeNameDatabase,
}

impl<'a> RaceUpgradeEntry<'a> {
    pub fn new(
        race: Race,
        art_database: &'a RaceUpgradeArtDatabase,
        name_database: &'a RaceUpgradeNameDatabase,
    ) -> Self {
        Self {
            race,
            art_database,
            name_database,
        }
    }

    pub fn race(&self) -> Race {
        self.race
    }

    pub fn art_database(&self) -> &RaceUpgradeArtDatabase {
        self.art_database
    }

    pub fn name_database(&self) -> &RaceUpgradeNameDatabase {
        self.name_database
    }
}

#[derive(Debug, Clone)]
pub struct UpgradeArtDefinition {
    values: Vec<String>,
}

impl From<ParsedEntry> for UpgradeArtDefinition {
    fn from(value: ParsedEntry) -> Self {
        let values = value.values().to_vec();
        Self::new(values)
    }
}

impl UpgradeArtDefinition {
    pub fn new(values: Vec<String>) -> Self {
        Self { values }
    }

    pub fn get_icons(&self) -> Vec<String> {
        self.values.clone()
    }
}

#[derive(Debug, Clone)]
pub struct UpgradeNameDefinition {
    values: Vec<String>,
}

impl From<ParsedEntry> for UpgradeNameDefinition {
    fn from(value: ParsedEntry) -> Self {
        let values = value.values().to_vec();
        Self::new(values)
    }
}

impl UpgradeNameDefinition {
    pub fn new(values: Vec<String>) -> Self {
        Self { values }
    }
    pub fn get_names(&self) -> Vec<String> {
        self.values.clone()
    }
}

#[derive(Debug, Clone)]
pub struct UpgradeDefinition {
    arts: UpgradeArtDefinition,
    names: UpgradeNameDefinition,
}

impl UpgradeDefinition {
    pub fn new(arts: UpgradeArtDefinition, names: UpgradeNameDefinition) -> Self {
        Self { arts, names }
    }

    pub fn max_level(&self) -> usize {
        self.arts.values.len().min(self.names.values.len())
    }

    pub fn art_for_level(&self, level: usize) -> Option<&str> {
        if level == 0 {
            None
        } else {
            self.arts.values.get(level - 1).map(String::as_str)
        }
    }
}

pub struct UpgradeFileParser;

impl UpgradeFileParser {
    pub fn parse_art_database(text: &str) -> RaceUpgradeArtDatabase {
        SectionedListParser::parse(text, "Art=")
            .into_iter()
            .map(|(id, entry)| (id, entry.into()))
            .collect()
    }

    pub fn parse_name_database(text: &str) -> RaceUpgradeNameDatabase {
        SectionedListParser::parse(text, "Name=")
            .into_iter()
            .map(|(id, entry)| (id, entry.into()))
            .collect()
    }
}
