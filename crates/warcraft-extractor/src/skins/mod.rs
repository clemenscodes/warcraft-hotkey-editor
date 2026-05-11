use std::collections::BTreeMap;

mod ability;
mod item;
mod unit;

pub type SkinDatabase = BTreeMap<String, String>;

pub use ability::*;
pub use item::*;
pub use unit::*;

#[derive(Debug, Clone, Default)]
pub struct AbilitySkinIcons {
    on_icons: SkinDatabase,
    off_icons: SkinDatabase,
}

impl AbilitySkinIcons {
    pub fn on_icons(&self) -> &SkinDatabase {
        &self.on_icons
    }

    pub fn off_icons(&self) -> &SkinDatabase {
        &self.off_icons
    }
}

pub struct SkinParser;

impl SkinParser {
    pub fn parse(text: &str) -> SkinDatabase {
        let mut skin_database = SkinDatabase::new();

        let mut current_id: Option<String> = None;
        let mut art: Option<String> = None;
        let mut art_sd: Option<String> = None;

        let flush = |database: &mut SkinDatabase,
                     id: &mut Option<String>,
                     art: &mut Option<String>,
                     art_sd: &mut Option<String>| {
            if let Some(unit_id) = id.take() {
                let chosen = art.take().or_else(|| art_sd.take());

                if let Some(path) = chosen {
                    let first_path = path
                        .split(',')
                        .map(str::trim)
                        .find(|segment| !segment.is_empty())
                        .unwrap_or(&path)
                        .to_string();
                    database.insert(unit_id, first_path.replace('\\', "/"));
                } else {
                    art_sd.take();
                }
            }
        };

        for raw_line in text.lines() {
            let line = raw_line.trim();

            if line.is_empty() {
                continue;
            }

            if let Some(id) = line
                .strip_prefix('[')
                .and_then(|line_inner| line_inner.strip_suffix(']'))
            {
                flush(&mut skin_database, &mut current_id, &mut art, &mut art_sd);
                current_id = Some(id.to_string());
                continue;
            }

            if let Some(value) = line.strip_prefix("Art=") {
                art = Some(value.trim().to_string());
                continue;
            }

            if let Some(value) = line.strip_prefix("Art:sd=") {
                art_sd = Some(value.trim().to_string());
                continue;
            }
        }

        flush(&mut skin_database, &mut current_id, &mut art, &mut art_sd);

        skin_database
    }

    pub fn parse_ability_icons(text: &str) -> AbilitySkinIcons {
        let mut on_icons = SkinDatabase::new();
        let mut off_icons = SkinDatabase::new();

        let mut current_id: Option<String> = None;
        let mut art: Option<String> = None;
        let mut art_sd: Option<String> = None;
        let mut unart: Option<String> = None;
        let mut unart_sd: Option<String> = None;

        let flush = |on_database: &mut SkinDatabase,
                     off_database: &mut SkinDatabase,
                     id: &mut Option<String>,
                     art: &mut Option<String>,
                     art_sd: &mut Option<String>,
                     unart: &mut Option<String>,
                     unart_sd: &mut Option<String>| {
            if let Some(ability_id) = id.take() {
                let on_path = art.take().or_else(|| art_sd.take());
                if let Some(path) = on_path {
                    let normalized = Self::first_path_segment(&path);
                    on_database.insert(ability_id.clone(), normalized);
                } else {
                    art_sd.take();
                }

                let off_path = unart.take().or_else(|| unart_sd.take());
                if let Some(path) = off_path {
                    let normalized = Self::first_path_segment(&path);
                    off_database.insert(ability_id, normalized);
                } else {
                    unart_sd.take();
                }
            }
        };

        for raw_line in text.lines() {
            let line = raw_line.trim();

            if line.is_empty() {
                continue;
            }

            if let Some(id) = line
                .strip_prefix('[')
                .and_then(|line_inner| line_inner.strip_suffix(']'))
            {
                flush(
                    &mut on_icons,
                    &mut off_icons,
                    &mut current_id,
                    &mut art,
                    &mut art_sd,
                    &mut unart,
                    &mut unart_sd,
                );
                current_id = Some(id.to_string());
                continue;
            }

            if let Some(value) = line.strip_prefix("Art=") {
                art = Some(value.trim().to_string());
                continue;
            }

            if let Some(value) = line.strip_prefix("Art:sd=") {
                art_sd = Some(value.trim().to_string());
                continue;
            }

            if let Some(value) = line.strip_prefix("Unart=") {
                unart = Some(value.trim().to_string());
                continue;
            }

            if let Some(value) = line.strip_prefix("Unart:sd=") {
                unart_sd = Some(value.trim().to_string());
                continue;
            }
        }

        flush(
            &mut on_icons,
            &mut off_icons,
            &mut current_id,
            &mut art,
            &mut art_sd,
            &mut unart,
            &mut unart_sd,
        );

        AbilitySkinIcons {
            on_icons,
            off_icons,
        }
    }

    fn first_path_segment(path: &str) -> String {
        path.split(',')
            .map(str::trim)
            .find(|segment| !segment.is_empty())
            .unwrap_or(path)
            .replace('\\', "/")
    }
}
