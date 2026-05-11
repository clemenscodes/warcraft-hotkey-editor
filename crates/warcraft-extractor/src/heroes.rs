use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
    sync::LazyLock,
};

use warcraft_slk::SlkTable;

use crate::{ExtractError, ExtractResult, ExtractTarget, ExtractionRule, Race, casc_filename};

pub type HeroDatabase = BTreeMap<String, BTreeSet<HeroAbility>>;

pub static HEROES_EXTRACTION_RULE: ExtractionRule = ExtractionRule {
    matcher: HeroesExtraction::matches,
    target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(),
    processor: HeroesExtraction::process,
};

static SUPPORTED_HERO_RACES: LazyLock<BTreeSet<&'static str>> =
    LazyLock::new(|| BTreeSet::from(["human", "nightelf", "orc", "undead", "creeps"]));

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct HeroAbility {
    race: Race,
    max_level: usize,
    is_ultimate: bool,
    ability: String,
    id: String,
    cooldowns: [u32; 4],
}

impl HeroAbility {
    pub fn race(&self) -> Race {
        self.race
    }

    pub fn max_level(&self) -> usize {
        self.max_level
    }

    pub fn is_ultimate(&self) -> bool {
        self.is_ultimate
    }

    pub fn ability(&self) -> &str {
        &self.ability
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn cooldowns(&self) -> [u32; 4] {
        self.cooldowns
    }
}

#[derive(Debug)]
struct ParsedAbility<'a> {
    hero: &'a str,
    ability: &'a str,
}

impl<'a> ParsedAbility<'a> {
    fn from_comments(comment: &'a str) -> Option<Self> {
        let (hero, rest) = comment.split_once(" - ")?;
        let ability = rest
            .split_once('(')
            .map(|(name, _)| name)
            .unwrap_or(rest)
            .trim();

        Some(Self {
            hero: hero.trim(),
            ability,
        })
    }
}

struct HeroesExtraction;

impl HeroesExtraction {
    fn matches(path: &str) -> bool {
        path.starts_with("war3.w3mod:units") && casc_filename(path).ends_with("abilitydata.slk")
    }

    fn process(path: &str, bytes: &[u8]) -> Result<ExtractResult, ExtractError> {
        tracing::debug!("Processing heroes with {path}");

        let text = std::str::from_utf8(bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid UTF-8"))?;

        let table = SlkTable::from(text);

        let hero_database = match casc_filename(path).as_str() {
            "abilitydata.slk" => Self::process_abilities(table),
            _ => return Err(ExtractError::Heroes),
        };

        Ok(ExtractResult::Heroes(hero_database))
    }

    fn process_abilities(table: SlkTable) -> HeroDatabase {
        let mut hero_database: HeroDatabase = BTreeMap::new();

        for row in table.into_iter() {
            let id = row.get("alias").unwrap_or("").trim();
            let comments = row.get("comments").unwrap_or("");
            let hero_flag = row.get("hero").unwrap_or("");
            let race_raw = row.get("race").unwrap_or("");
            let levels_raw = row.get("levels").unwrap_or("");

            if hero_flag != "1" || id.is_empty() {
                continue;
            }

            if !SUPPORTED_HERO_RACES.contains(race_raw) {
                continue;
            }

            let Some(parsed) = ParsedAbility::from_comments(comments) else {
                continue;
            };

            let max_level: usize = if levels_raw == "3" { 3 } else { 1 };
            let is_ultimate = levels_raw != "3";

            let race = match race_raw {
                "human" => Race::Human,
                "nightelf" => Race::Nightelf,
                "orc" => Race::Orc,
                "undead" => Race::Undead,
                "creeps" => Race::Neutral,
                _ => continue,
            };

            let cool1 = row.get("Cool1");
            let cool2 = row.get("Cool2");
            let cool3 = row.get("Cool3");
            let cool4 = row.get("Cool4");
            let cooldowns = [
                Self::parse_cooldown_milliseconds(cool1),
                Self::parse_cooldown_milliseconds(cool2),
                Self::parse_cooldown_milliseconds(cool3),
                Self::parse_cooldown_milliseconds(cool4),
            ];

            let ability = HeroAbility {
                race,
                max_level,
                is_ultimate,
                ability: parsed.ability.to_string(),
                id: id.to_string(),
                cooldowns,
            };

            hero_database
                .entry(parsed.hero.to_string())
                .or_default()
                .insert(ability);
        }

        hero_database
    }

    fn parse_cooldown_milliseconds(text: Option<&str>) -> u32 {
        let seconds = text.unwrap_or("0").trim().parse::<f32>().unwrap_or(0.0);
        let milliseconds = (seconds * 1000.0).round();
        if milliseconds.is_nan() || milliseconds <= 0.0 {
            return 0;
        }
        let maximum_representable_milliseconds =
            num_traits::cast::cast::<_, f32>(u32::MAX).expect("u32::MAX representable as f32");
        if milliseconds >= maximum_representable_milliseconds {
            return u32::MAX;
        }
        num_traits::cast::cast::<_, u32>(milliseconds).expect("milliseconds representable as u32")
    }
}
