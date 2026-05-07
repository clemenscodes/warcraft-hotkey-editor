use std::collections::{BTreeMap, HashSet};
use std::fmt;

use warcraft_api::{WarcraftObjectId, WarcraftObjectKind, WarcraftObjectMeta};
use warcraft_database::{WARCRAFT_DATABASE, WARCRAFT_SYSTEM_KEYBINDS};

use crate::model::{
    AbilityBinding, BindingEntry, ButtonPosition, CommandBinding, CommandEntry, SectionAccumulator,
    SectionKind, SystemBinding, WarcraftKeybinding,
};

const BUNDLED_BASELINE: &str = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
const GRID_COLUMNS: u8 = 4;
const GRID_ROWS: u8 = 3;

#[derive(Clone)]
pub struct CustomKeysFile {
    entries: BTreeMap<WarcraftObjectId, WarcraftKeybinding>,
}

impl CustomKeysFile {
    pub(crate) fn from_parts(entries: BTreeMap<WarcraftObjectId, WarcraftKeybinding>) -> Self {
        Self { entries }
    }

    pub fn binding(&self, id: &str) -> Option<&AbilityBinding> {
        self.entries.get(id)?.as_ability()
    }

    pub fn binding_mut(&mut self, id: &str) -> Option<&mut AbilityBinding> {
        self.entries.get_mut(id)?.as_ability_mut()
    }

    pub fn binding_or_default_mut(
        &mut self,
        id: impl Into<WarcraftObjectId>,
    ) -> Option<&mut AbilityBinding> {
        let object_id = id.into();
        if !matches!(
            self.entries.get(object_id.value()),
            Some(WarcraftKeybinding::Ability(_))
        ) {
            self.entries.insert(
                object_id,
                WarcraftKeybinding::Ability(AbilityBinding::default()),
            );
        }
        self.entries
            .get_mut(object_id.value())
            .and_then(WarcraftKeybinding::as_ability_mut)
    }

    pub fn bindings_in_order(&self) -> impl Iterator<Item = BindingEntry<'_>> {
        self.entries.iter().filter_map(|(id, binding)| {
            binding
                .as_ability()
                .map(|ability| BindingEntry::new(*id, ability))
        })
    }

    pub fn command(&self, name: &str) -> Option<&CommandBinding> {
        self.entries.get(name)?.as_command()
    }

    pub fn command_mut(&mut self, name: &str) -> Option<&mut CommandBinding> {
        self.entries.get_mut(name)?.as_command_mut()
    }

    pub fn command_or_default_mut(
        &mut self,
        name: impl Into<WarcraftObjectId>,
    ) -> Option<&mut CommandBinding> {
        let object_id = name.into();
        if !matches!(
            self.entries.get(object_id.value()),
            Some(WarcraftKeybinding::Command(_))
        ) {
            self.entries.insert(
                object_id,
                WarcraftKeybinding::Command(CommandBinding::default()),
            );
        }
        self.entries
            .get_mut(object_id.value())
            .and_then(WarcraftKeybinding::as_command_mut)
    }

    pub fn commands_in_order(&self) -> impl Iterator<Item = CommandEntry<'_>> {
        self.entries.iter().filter_map(|(name, binding)| {
            binding
                .as_command()
                .map(|command| CommandEntry::new(*name, command))
        })
    }

    pub fn system(&self, id: &str) -> Option<&SystemBinding> {
        self.entries.get(id)?.as_system()
    }

    pub fn system_mut(&mut self, id: &str) -> Option<&mut SystemBinding> {
        self.entries.get_mut(id)?.as_system_mut()
    }

    pub fn builder() -> crate::builder::CustomKeysFileBuilder {
        crate::builder::CustomKeysFileBuilder::new()
    }

    pub fn put_ability(&mut self, id: impl Into<WarcraftObjectId>, binding: AbilityBinding) {
        let object_id = id.into();
        self.entries
            .insert(object_id, WarcraftKeybinding::Ability(binding));
    }

    pub fn put_command(&mut self, name: impl Into<WarcraftObjectId>, binding: CommandBinding) {
        let object_id = name.into();
        self.entries
            .insert(object_id, WarcraftKeybinding::Command(binding));
    }

    pub fn put_system(&mut self, id: impl Into<WarcraftObjectId>, binding: SystemBinding) {
        let object_id = id.into();
        self.entries
            .insert(object_id, WarcraftKeybinding::System(binding));
    }

    pub fn normalize(&self) -> Self {
        let mut result = Self::from(BUNDLED_BASELINE);
        let overlay_clone = self.clone();
        result.extend(overlay_clone);
        result.materialize_default_positions();
        result.materialize_shop_item_positions();
        result
    }

    pub fn serialize(&self, baseline: &str) -> String {
        let mut export_file = Self::from(baseline);
        let overlay_clone = self.clone();
        export_file.extend(overlay_clone);
        export_file.materialize_default_positions();
        export_file.materialize_shop_item_positions();
        export_file.to_string()
    }

    fn materialize_default_positions(&mut self) {
        for (object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let default_button = warcraft_object.default_button_position().map(|position| {
                let column = position.column();
                let row = position.row();
                ButtonPosition::new(column, row)
            });
            let default_research =
                warcraft_object
                    .default_research_button_position()
                    .map(|position| {
                        let column = position.column();
                        let row = position.row();
                        ButtonPosition::new(column, row)
                    });

            match warcraft_object.kind() {
                WarcraftObjectKind::Command => continue,
                WarcraftObjectKind::Ability => {
                    if default_button.is_none() && default_research.is_none() {
                        continue;
                    }
                    let canonical_id = *object_id;
                    let Some(binding) = self.binding_or_default_mut(canonical_id) else {
                        continue;
                    };
                    if binding.button_position().is_none()
                        && let Some(position_value) = default_button
                    {
                        binding.set_button_position(Some(position_value));
                    }
                    if binding.research_button_position().is_none()
                        && let Some(position_value) = default_research
                    {
                        binding.set_research_button_position(Some(position_value));
                    }
                    if binding.unbutton_position().is_none()
                        && !warcraft_object.is_passive_ability()
                    {
                        let database_off = match warcraft_object.meta() {
                            WarcraftObjectMeta::Ability(ability_meta) => {
                                ability_meta.off_button_position().map(|position| {
                                    let column = position.column();
                                    let row = position.row();
                                    ButtonPosition::new(column, row)
                                })
                            }
                            _ => None,
                        };
                        if let Some(off_position) = database_off {
                            binding.set_unbutton_position(Some(off_position));
                        } else if let Some(button_position) = binding.button_position() {
                            let position_copy = *button_position;
                            binding.set_unbutton_position(Some(position_copy));
                        }
                    }
                }
                _ => continue,
            }
        }
    }

    fn materialize_shop_item_positions(&mut self) {
        for (_object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let WarcraftObjectMeta::Unit(unit_meta) = warcraft_object.meta() else {
                continue;
            };
            let sell_items = unit_meta.sell_items();
            if sell_items.is_empty() {
                continue;
            }

            let mut occupied_positions: HashSet<ButtonPosition> = HashSet::new();
            for item_id_object in sell_items {
                let item_id = item_id_object.value();
                let item_binding = self.binding(item_id);
                let position_ref = item_binding.and_then(|binding| binding.button_position());
                let existing_position = position_ref.copied();
                if let Some(position) = existing_position {
                    occupied_positions.insert(position);
                }
            }

            for item_id_object in sell_items {
                let item_id = item_id_object.value();
                let item_binding = self.binding(item_id);
                let position_ref = item_binding.and_then(|binding| binding.button_position());
                let has_position = position_ref.is_some();
                if has_position {
                    continue;
                }
                let Some(free_position) = Self::next_free_grid_cell(&occupied_positions) else {
                    continue;
                };
                occupied_positions.insert(free_position);
                let item_canonical_id = *item_id_object;
                if let Some(item_binding) = self.binding_or_default_mut(item_canonical_id) {
                    item_binding.set_button_position(Some(free_position));
                }
            }
        }
    }

    fn next_free_grid_cell(occupied_positions: &HashSet<ButtonPosition>) -> Option<ButtonPosition> {
        for row in 0..GRID_ROWS {
            for column in 0..GRID_COLUMNS {
                let candidate = ButtonPosition::new(column, row);
                if !occupied_positions.contains(&candidate) {
                    return Some(candidate);
                }
            }
        }
        None
    }

    fn write_ability_section(
        formatter: &mut fmt::Formatter<'_>,
        object_id: WarcraftObjectId,
        binding: &AbilityBinding,
    ) -> fmt::Result {
        let id_lowercase = object_id.value().to_ascii_lowercase();
        writeln!(formatter, "[{id_lowercase}]")?;
        if let Some(hotkey) = binding.hotkey() {
            let hotkey_string = hotkey.to_string();
            writeln!(formatter, "Hotkey={hotkey_string}")?;
        }
        if let Some(hotkey) = binding.unhotkey() {
            let hotkey_string = hotkey.to_string();
            writeln!(formatter, "Unhotkey={hotkey_string}")?;
        }
        if let Some(position) = binding.button_position() {
            writeln!(formatter, "Buttonpos={position}")?;
        }
        if let Some(position) = binding.unbutton_position() {
            writeln!(formatter, "Unbuttonpos={position}")?;
        }
        if let Some(hotkey) = binding.research_hotkey() {
            let hotkey_string = hotkey.to_string();
            writeln!(formatter, "Researchhotkey={hotkey_string}")?;
        }
        if let Some(position) = binding.research_button_position() {
            writeln!(formatter, "Researchbuttonpos={position}")?;
        }
        if let Some(value) = binding.tip() {
            writeln!(formatter, "Tip={value}")?;
        }
        if let Some(value) = binding.research_tip() {
            writeln!(formatter, "Researchtip={value}")?;
        }
        if let Some(value) = binding.un_tip() {
            writeln!(formatter, "UnTip={value}")?;
        }
        if let Some(value) = binding.ubertip() {
            writeln!(formatter, "Ubertip={value}")?;
        }
        if let Some(value) = binding.research_ubertip() {
            writeln!(formatter, "Researchubertip={value}")?;
        }
        if let Some(value) = binding.un_ubertip() {
            writeln!(formatter, "Unubertip={value}")?;
        }
        if let Some(value) = binding.icon() {
            writeln!(formatter, "Icon={value}")?;
        }
        if let Some(modifier) = binding.modifier() {
            let modifier_string = modifier.to_string();
            writeln!(formatter, "Modifier={modifier_string}")?;
        }
        writeln!(formatter)
    }

    fn write_command_section(
        formatter: &mut fmt::Formatter<'_>,
        object_id: WarcraftObjectId,
        binding: &CommandBinding,
    ) -> fmt::Result {
        let id_lowercase = object_id.value().to_ascii_lowercase();
        writeln!(formatter, "[{id_lowercase}]")?;
        if let Some(hotkey) = binding.hotkey() {
            let hotkey_string = hotkey.to_string();
            writeln!(formatter, "Hotkey={hotkey_string}")?;
        }
        if let Some(position) = binding.button_position() {
            writeln!(formatter, "Buttonpos={position}")?;
        }
        if let Some(position) = binding.unbutton_position() {
            writeln!(formatter, "Unbuttonpos={position}")?;
        }
        if let Some(value) = binding.tip() {
            writeln!(formatter, "Tip={value}")?;
        }
        if let Some(value) = binding.un_tip() {
            writeln!(formatter, "UnTip={value}")?;
        }
        writeln!(formatter)
    }

    fn write_system_section(
        formatter: &mut fmt::Formatter<'_>,
        object_id: WarcraftObjectId,
        binding: &SystemBinding,
    ) -> fmt::Result {
        let id_lowercase = object_id.value().to_ascii_lowercase();
        writeln!(formatter, "[{id_lowercase}]")?;
        let hotkey = binding.hotkey();
        writeln!(formatter, "Hotkey={hotkey}")?;
        let binding_class = binding.class();
        let class_field = binding_class.ini_field();
        writeln!(formatter, "{class_field}")?;
        if let Some(modifier) = binding.modifier()
            && let Some(modifier_text) = modifier.ini_str()
        {
            writeln!(formatter, "Modifier={modifier_text}")?;
        }
        writeln!(formatter)
    }
}

impl Default for CustomKeysFile {
    fn default() -> Self {
        Self::from("")
    }
}

impl fmt::Display for CustomKeysFile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (object_id, entry) in &self.entries {
            match entry {
                WarcraftKeybinding::Ability(binding) => {
                    Self::write_ability_section(formatter, *object_id, binding)?;
                }
                WarcraftKeybinding::Command(binding) => {
                    Self::write_command_section(formatter, *object_id, binding)?;
                }
                WarcraftKeybinding::System(binding) => {
                    Self::write_system_section(formatter, *object_id, binding)?;
                }
            }
        }
        Ok(())
    }
}

/// Owned iterator over `(WarcraftObjectId, WarcraftKeybinding)` pairs.
/// Keys are canonical-case, in alphabetical order.
pub struct CustomKeysFileIntoIter {
    inner: std::collections::btree_map::IntoIter<WarcraftObjectId, WarcraftKeybinding>,
}

impl Iterator for CustomKeysFileIntoIter {
    type Item = (WarcraftObjectId, WarcraftKeybinding);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl IntoIterator for CustomKeysFile {
    type Item = (WarcraftObjectId, WarcraftKeybinding);
    type IntoIter = CustomKeysFileIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        let inner = self.entries.into_iter();
        CustomKeysFileIntoIter { inner }
    }
}

impl Extend<(WarcraftObjectId, WarcraftKeybinding)> for CustomKeysFile {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (WarcraftObjectId, WarcraftKeybinding)>,
    {
        for (object_id, binding) in iter {
            let raw_key = object_id.value();
            match binding {
                WarcraftKeybinding::Ability(source_binding) => {
                    if self.system(raw_key).is_some() {
                        continue;
                    }
                    let Some(target_binding) = self.binding_or_default_mut(object_id) else {
                        continue;
                    };
                    if let Some(hotkey) = source_binding.hotkey() {
                        let hotkey_clone = hotkey.clone();
                        target_binding.set_hotkey(Some(hotkey_clone));
                    }
                    if let Some(position) = source_binding.button_position().copied() {
                        target_binding.set_button_position(Some(position));
                    }
                    if let Some(position) = source_binding.unbutton_position().copied() {
                        target_binding.set_unbutton_position(Some(position));
                    }
                    if let Some(hotkey) = source_binding.research_hotkey() {
                        let hotkey_clone = hotkey.clone();
                        target_binding.set_research_hotkey(Some(hotkey_clone));
                    }
                    if let Some(position) = source_binding.research_button_position().copied() {
                        target_binding.set_research_button_position(Some(position));
                    }
                    if let Some(tip) = source_binding.tip() {
                        let tip_string = tip.to_string();
                        target_binding.set_tip(Some(tip_string));
                    }
                    if let Some(tip) = source_binding.research_tip() {
                        let tip_string = tip.to_string();
                        target_binding.set_research_tip(Some(tip_string));
                    }
                    if let Some(tip) = source_binding.un_tip() {
                        let tip_string = tip.to_string();
                        target_binding.set_un_tip(Some(tip_string));
                    }
                    if let Some(icon) = source_binding.icon() {
                        let icon_string = icon.to_string();
                        target_binding.set_icon(Some(icon_string));
                    }
                }
                WarcraftKeybinding::Command(source_binding) => {
                    let Some(target_binding) = self.command_or_default_mut(object_id) else {
                        continue;
                    };
                    if let Some(hotkey) = source_binding.hotkey() {
                        let hotkey_clone = hotkey.clone();
                        target_binding.set_hotkey(Some(hotkey_clone));
                    }
                    if let Some(position) = source_binding.button_position().copied() {
                        target_binding.set_button_position(Some(position));
                    }
                    if let Some(position) = source_binding.unbutton_position().copied() {
                        target_binding.set_unbutton_position(Some(position));
                    }
                    if let Some(tip) = source_binding.tip() {
                        let tip_string = tip.to_string();
                        target_binding.set_tip(Some(tip_string));
                    }
                    if let Some(tip) = source_binding.un_tip() {
                        let tip_string = tip.to_string();
                        target_binding.set_un_tip(Some(tip_string));
                    }
                }
                WarcraftKeybinding::System(_) => {}
            }
        }
    }
}

// ──────────────── Parser ────────────────────────────────────────────────────

fn parse_section_id(line: &str) -> Option<String> {
    let without_brackets = line.strip_prefix('[')?.strip_suffix(']')?;
    let section_id = without_brackets.trim();
    if section_id.is_empty() {
        None
    } else {
        Some(section_id.to_string())
    }
}

fn flush_section(
    current_key: &mut Option<WarcraftObjectId>,
    accumulator: &mut Option<SectionAccumulator>,
    entries: &mut BTreeMap<WarcraftObjectId, WarcraftKeybinding>,
) {
    let maybe_key = current_key.take();
    let maybe_accumulated = accumulator.take();
    if let Some(object_id) = maybe_key
        && let Some(accumulated) = maybe_accumulated
    {
        let binding = WarcraftKeybinding::from(accumulated);
        entries.insert(object_id, binding);
    }
}

impl SectionKind {
    fn for_section_id(id: &str) -> Option<(WarcraftObjectId, Self)> {
        let lowercase = id.to_ascii_lowercase();
        if let Some((canonical_id, object)) = WARCRAFT_DATABASE
            .iter()
            .find(|(key, _)| key.value().to_ascii_lowercase() == lowercase)
        {
            let section_kind = match object.kind() {
                WarcraftObjectKind::Command => SectionKind::Command,
                _ => SectionKind::Ability,
            };
            return Some((*canonical_id, section_kind));
        }
        if let Some(entry) = WARCRAFT_SYSTEM_KEYBINDS
            .iter()
            .find(|entry| entry.section_id().to_ascii_lowercase() == lowercase)
        {
            let canonical_id = WarcraftObjectId::new(entry.section_id());
            return Some((canonical_id, SectionKind::System(entry.class())));
        }
        None
    }
}

impl From<&str> for CustomKeysFile {
    fn from(text: &str) -> Self {
        let mut entries: BTreeMap<WarcraftObjectId, WarcraftKeybinding> = BTreeMap::new();

        let mut current_key: Option<WarcraftObjectId> = None;
        let mut accumulator: Option<SectionAccumulator> = None;

        for line in text.lines() {
            let trimmed = line.trim();
            let is_blank = trimmed.is_empty();
            let is_comment = trimmed.starts_with("//") || trimmed.starts_with(';');

            let header = if is_blank || is_comment {
                None
            } else {
                parse_section_id(trimmed)
            };

            if let Some(original_id) = header {
                flush_section(&mut current_key, &mut accumulator, &mut entries);

                if let Some((canonical_id, section_kind)) =
                    SectionKind::for_section_id(&original_id)
                {
                    if entries.contains_key(canonical_id.value()) {
                        current_key = None;
                        accumulator = None;
                    } else {
                        let section_accumulator = SectionAccumulator::new(section_kind);
                        current_key = Some(canonical_id);
                        accumulator = Some(section_accumulator);
                    }
                } else {
                    current_key = None;
                    accumulator = None;
                }
            } else if !is_blank
                && !is_comment
                && let Some((key, value)) = trimmed.split_once('=')
                && let Some(section_accumulator) = accumulator.as_mut()
            {
                section_accumulator.apply(key.trim(), value);
            }
        }

        flush_section(&mut current_key, &mut accumulator, &mut entries);

        CustomKeysFile::from_parts(entries)
    }
}

impl From<String> for CustomKeysFile {
    fn from(text: String) -> Self {
        Self::from(text.as_str())
    }
}
