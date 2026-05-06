pub(crate) struct DefaultConfig;

impl DefaultConfig {
    pub(crate) fn content() -> &'static str {
        include_str!("../../templates/CustomKeys.txt")
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use warcraft_api::WarcraftObjectMeta;
    use warcraft_database::{WARCRAFT_DATABASE, WARCRAFT_SYSTEM_KEYBINDS};
    use warcraft_keybinds::CustomKeysFile;

    use crate::domain::grid_layout::GridLayout;
    use crate::domain::object_lookup::ObjectLookup;

    fn join_levels(levels: &[&str]) -> Option<String> {
        if levels.is_empty() {
            None
        } else {
            Some(levels.join(","))
        }
    }

    fn build_text(layout: &GridLayout) -> String {
        let tmpl = CustomKeysFile::from(include_str!("../../templates/CustomKeys.txt"));
        let mut out = String::new();

        for (object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let id = object_id.value();
            let WarcraftObjectMeta::Command(cmd_meta) = warcraft_object.meta() else {
                continue;
            };
            let Some(default_position) = cmd_meta.default_button_position() else {
                continue;
            };
            let traditional = tmpl.command(id);
            let section_header = format!("[{id}]\n");
            out.push_str(&section_header);
            if let Some(hotkey_string) = traditional
                .and_then(|c| c.hotkey())
                .map(|hotkey_display| hotkey_display.to_string())
            {
                let hotkey_line = format!("Hotkey={hotkey_string}\n");
                out.push_str(&hotkey_line);
            }
            let buttonpos_line = format!(
                "Buttonpos={},{}\n",
                default_position.column(),
                default_position.row()
            );
            out.push_str(&buttonpos_line);
            if let Some(tip) = traditional
                .and_then(|c| c.tip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.tip_levels()))
            {
                let tip_line = format!("Tip={tip}\n");
                out.push_str(&tip_line);
            }
            if let Some(ubertip) = warcraft_object.ubertip() {
                let ubertip_line = format!("Ubertip={ubertip}\n");
                out.push_str(&ubertip_line);
            }
            out.push('\n');
        }

        for (object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let id = object_id.value();
            let WarcraftObjectMeta::Ability(ability_meta) = warcraft_object.meta() else {
                continue;
            };
            let default_button_position = warcraft_object.default_button_position();
            let default_research_position = warcraft_object.default_research_button_position();
            let off_button_position = ability_meta.off_button_position();
            if default_button_position.is_none()
                && default_research_position.is_none()
                && off_button_position.is_none()
            {
                continue;
            }
            let is_passive = ObjectLookup::is_passive_ability(id);
            let existing_binding = tmpl.binding(id);

            let section_header = format!("[{id}]\n");
            out.push_str(&section_header);

            if let Some(button_position) = default_button_position {
                if !is_passive {
                    let hotkey = existing_binding
                        .and_then(|binding| binding.hotkey())
                        .map(|hotkey_display| hotkey_display.to_string())
                        .or_else(|| {
                            layout
                                .letter_at(button_position.column(), button_position.row())
                                .map(|letter| letter.to_string())
                        });
                    if let Some(hotkey_string) = hotkey {
                        let hotkey_line = format!("Hotkey={hotkey_string}\n");
                        out.push_str(&hotkey_line);
                    }
                }
                let buttonpos_line = format!(
                    "Buttonpos={},{}\n",
                    button_position.column(),
                    button_position.row()
                );
                out.push_str(&buttonpos_line);
            }

            if let Some(research_position) = default_research_position {
                let research_hotkey = existing_binding
                    .and_then(|binding| binding.research_hotkey())
                    .map(|hotkey_display| hotkey_display.to_string())
                    .or_else(|| {
                        layout
                            .letter_at(research_position.column(), research_position.row())
                            .map(|letter| letter.to_string())
                    });
                if let Some(research_hotkey_string) = research_hotkey {
                    let research_hotkey_line = format!("ResearchHotkey={research_hotkey_string}\n");
                    out.push_str(&research_hotkey_line);
                }
                let research_buttonpos_line = format!(
                    "ResearchButtonpos={},{}\n",
                    research_position.column(),
                    research_position.row()
                );
                out.push_str(&research_buttonpos_line);
            }

            if let Some(off_position) = off_button_position {
                let un_hotkey = existing_binding
                    .and_then(|binding| binding.unhotkey())
                    .map(|hotkey_display| hotkey_display.to_string())
                    .or_else(|| {
                        layout
                            .letter_at(off_position.column(), off_position.row())
                            .map(|letter| letter.to_string())
                    });
                if let Some(unhotkey_string) = un_hotkey {
                    let unhotkey_line = format!("Unhotkey={unhotkey_string}\n");
                    out.push_str(&unhotkey_line);
                }
                let unbuttonpos_line = format!(
                    "Unbuttonpos={},{}\n",
                    off_position.column(),
                    off_position.row()
                );
                out.push_str(&unbuttonpos_line);
            }

            if let Some(tip) = existing_binding
                .and_then(|binding| binding.tip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.tip_levels()))
            {
                let tip_line = format!("Tip={tip}\n");
                out.push_str(&tip_line);
            }
            if let Some(un_tip) = existing_binding
                .and_then(|binding| binding.un_tip())
                .map(str::to_owned)
                .or_else(|| warcraft_object.un_tip().map(str::to_owned))
            {
                let untip_line = format!("Untip={un_tip}\n");
                out.push_str(&untip_line);
            }
            if let Some(ubertip) = existing_binding
                .and_then(|binding| binding.ubertip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.ubertip_levels()))
            {
                let ubertip_line = format!("Ubertip={ubertip}\n");
                out.push_str(&ubertip_line);
            }
            if let Some(un_ubertip) = existing_binding
                .and_then(|binding| binding.un_ubertip())
                .map(str::to_owned)
                .or_else(|| warcraft_object.un_ubertip().map(str::to_owned))
            {
                let un_ubertip_line = format!("Unubertip={un_ubertip}\n");
                out.push_str(&un_ubertip_line);
            }
            if let Some(research_ubertip) = existing_binding
                .and_then(|binding| binding.research_ubertip())
                .map(str::to_owned)
                .or_else(|| warcraft_object.research_ubertip().map(str::to_owned))
            {
                let research_ubertip_line = format!("Researchubertip={research_ubertip}\n");
                out.push_str(&research_ubertip_line);
            }

            out.push('\n');
        }

        for (object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let id = object_id.value();
            let WarcraftObjectMeta::Unit(_) = warcraft_object.meta() else {
                continue;
            };
            let Some(default_position) = warcraft_object.default_button_position() else {
                continue;
            };
            let existing_binding = tmpl.binding(id);
            let section_header = format!("[{id}]\n");
            out.push_str(&section_header);
            if let Some(hotkey_string) = existing_binding
                .and_then(|binding| binding.hotkey())
                .map(|hotkey_display| hotkey_display.to_string())
            {
                let hotkey_line = format!("Hotkey={hotkey_string}\n");
                out.push_str(&hotkey_line);
            }
            let buttonpos_line = format!(
                "Buttonpos={},{}\n",
                default_position.column(),
                default_position.row()
            );
            out.push_str(&buttonpos_line);
            if let Some(tip) = existing_binding
                .and_then(|binding| binding.tip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.tip_levels()))
            {
                let tip_line = format!("Tip={tip}\n");
                out.push_str(&tip_line);
            }
            if let Some(ubertip) = existing_binding
                .and_then(|binding| binding.ubertip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.ubertip_levels()))
            {
                let ubertip_line = format!("Ubertip={ubertip}\n");
                out.push_str(&ubertip_line);
            }
            out.push('\n');
        }

        for (object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let id = object_id.value();
            if !matches!(
                warcraft_object.meta(),
                WarcraftObjectMeta::Upgrade(_) | WarcraftObjectMeta::Item(_)
            ) {
                continue;
            }
            let Some(default_position) = warcraft_object.default_button_position() else {
                continue;
            };
            let research_position = warcraft_object.default_research_button_position();
            let existing_binding = tmpl.binding(id);

            let section_header = format!("[{id}]\n");
            out.push_str(&section_header);

            let hotkey = existing_binding
                .and_then(|binding| binding.hotkey())
                .map(|hotkey_display| hotkey_display.to_string())
                .or_else(|| {
                    layout
                        .letter_at(default_position.column(), default_position.row())
                        .map(|letter| letter.to_string())
                });
            if let Some(hotkey_string) = hotkey {
                let hotkey_line = format!("Hotkey={hotkey_string}\n");
                out.push_str(&hotkey_line);
            }
            let buttonpos_line = format!(
                "Buttonpos={},{}\n",
                default_position.column(),
                default_position.row()
            );
            out.push_str(&buttonpos_line);

            if let Some(research_button_position) = research_position {
                let research_hotkey_string = existing_binding
                    .and_then(|binding| binding.research_hotkey())
                    .map(|hotkey_display| hotkey_display.to_string())
                    .or_else(|| {
                        layout
                            .letter_at(
                                research_button_position.column(),
                                research_button_position.row(),
                            )
                            .map(|letter| letter.to_string())
                    });
                if let Some(research_hotkey_line_value) = research_hotkey_string {
                    let research_hotkey_line =
                        format!("ResearchHotkey={research_hotkey_line_value}\n");
                    out.push_str(&research_hotkey_line);
                }
                let research_buttonpos_line = format!(
                    "ResearchButtonpos={},{}\n",
                    research_button_position.column(),
                    research_button_position.row()
                );
                out.push_str(&research_buttonpos_line);
            }

            if let Some(tip) = existing_binding
                .and_then(|binding| binding.tip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.tip_levels()))
            {
                let tip_line = format!("Tip={tip}\n");
                out.push_str(&tip_line);
            }
            if let Some(ubertip) = existing_binding
                .and_then(|binding| binding.ubertip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.ubertip_levels()))
            {
                let ubertip_line = format!("Ubertip={ubertip}\n");
                out.push_str(&ubertip_line);
            }

            out.push('\n');
        }

        // System keybinds: inventory slots, hero selection, control groups,
        // camera, and all other game-level bindings. Hotkey is the default VK
        // code; Modifier and the command-class field are written for
        // transparency (the game hardcodes both and ignores any override).
        for entry in WARCRAFT_SYSTEM_KEYBINDS.iter() {
            let id = entry.section_id();
            let hotkey_code = tmpl
                .system(id)
                .map(|binding| binding.hotkey())
                .unwrap_or_else(|| entry.default_hotkey());
            let section_header = format!("[{id}]\n");
            out.push_str(&section_header);
            let hotkey_line = format!("Hotkey={hotkey_code}\n");
            out.push_str(&hotkey_line);
            out.push_str(entry.class().ini_field());
            out.push('\n');
            if let Some(modifier_text) = entry.default_modifier().ini_str() {
                let modifier_line = format!("Modifier={modifier_text}\n");
                out.push_str(&modifier_line);
            }
            out.push('\n');
        }

        out
    }

    /// Under the global cascade, every ability has exactly one stored
    /// `Buttonpos`. The render path no longer cascades, so whatever
    /// is stored is what the editor displays — there is no longer a
    /// way for localStorage to disagree with the display.
    ///
    /// This test pins the invariant: after `fully_normalize`, every
    /// shared ability used as a regression case in earlier per-unit
    /// cascade fixes (Anh2, ACd2, ACif, ACf2) has a concrete
    /// `Buttonpos` value, and that value is what `serialize` writes
    /// into localStorage.
    #[test]
    fn export_positions_match_display_after_template_apply() {
        use crate::domain::default_config::DefaultConfig;
        use warcraft_keybinds::ButtonPosition;
        use warcraft_keybinds::CustomKeysFile;
        use warcraft_keybinds::cascade::fully_normalize;

        let baseline = DefaultConfig::content();

        let mut loaded = CustomKeysFile::from(baseline);
        fully_normalize(&mut loaded);

        let export_content = warcraft_keybinds::export::serialize(&loaded, baseline);
        let export_file = CustomKeysFile::from(export_content.as_str());

        let normalized_position = |id: &str| -> Option<ButtonPosition> {
            let binding = loaded.binding(id)?;
            let position_ref = binding.button_position()?;
            let column_value = position_ref.column();
            let row_value = position_ref.row();
            Some(ButtonPosition::new(column_value, row_value))
        };
        let exported_position = |id: &str| -> Option<ButtonPosition> {
            let binding = export_file.binding(id)?;
            let position_ref = binding.button_position()?;
            let column_value = position_ref.column();
            let row_value = position_ref.row();
            Some(ButtonPosition::new(column_value, row_value))
        };

        let regression_ids = ["Anh2", "ACd2", "ACif", "ACf2"];
        for ability_id in regression_ids {
            let after_normalize = normalized_position(ability_id);
            let after_export = exported_position(ability_id);
            assert!(
                after_normalize.is_some(),
                "{ability_id} must have a Buttonpos after fully_normalize"
            );
            assert_eq!(
                after_normalize, after_export,
                "{ability_id}: serialized export must match the normalized in-memory position",
            );
        }
    }

    /// Regenerates CustomKeys.txt from the database. Run this whenever
    /// warcraft-database changes to keep the default template in sync.
    #[test]
    fn regenerate_default_template() {
        let content = build_text(&GridLayout::qwerty_grid());
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/templates/CustomKeys.txt");
        std::fs::write(path, &content).expect("failed to write default template");
        println!("wrote {} bytes to {path}", content.len());
    }
}
