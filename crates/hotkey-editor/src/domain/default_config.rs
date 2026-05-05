pub(crate) struct DefaultConfig;

impl DefaultConfig {
    pub(crate) fn content() -> &'static str {
        include_str!("../../templates/CustomKeys.txt")
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use warcraft_api::{SystemKeybindModifier, WarcraftObjectMeta};
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
            let Some(pos) = cmd_meta.default_button_position() else {
                continue;
            };
            let traditional = tmpl.command(id);
            out.push_str(&format!("[{id}]\n"));
            if let Some(hk) = traditional.and_then(|c| c.hotkey()) {
                out.push_str(&format!("Hotkey={hk}\n"));
            }
            out.push_str(&format!("Buttonpos={},{}\n", pos.column(), pos.row()));
            if let Some(tip) = traditional
                .and_then(|c| c.tip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.tip_levels()))
            {
                out.push_str(&format!("Tip={tip}\n"));
            }
            if let Some(ubertip) = warcraft_object.ubertip() {
                out.push_str(&format!("Ubertip={ubertip}\n"));
            }
            out.push('\n');
        }

        for (object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let id = object_id.value();
            let WarcraftObjectMeta::Ability(ability_meta) = warcraft_object.meta() else {
                continue;
            };
            let btn = warcraft_object.default_button_position();
            let res = warcraft_object.default_research_button_position();
            let off = ability_meta.off_button_position();
            if btn.is_none() && res.is_none() && off.is_none() {
                continue;
            }
            let is_passive = ObjectLookup::is_passive_ability(id);
            let existing_binding = tmpl.binding(id);

            out.push_str(&format!("[{id}]\n"));

            if let Some(p) = btn {
                if !is_passive {
                    let hotkey = existing_binding
                        .and_then(|b| b.hotkey())
                        .map(str::to_owned)
                        .or_else(|| layout.letter_at(p.column(), p.row()).map(|c| c.to_string()));
                    if let Some(hk) = hotkey {
                        out.push_str(&format!("Hotkey={hk}\n"));
                    }
                }
                out.push_str(&format!("Buttonpos={},{}\n", p.column(), p.row()));
            }

            if let Some(p) = res {
                let research_hotkey = existing_binding
                    .and_then(|b| b.research_hotkey())
                    .map(str::to_owned)
                    .or_else(|| layout.letter_at(p.column(), p.row()).map(|c| c.to_string()));
                if let Some(hk) = research_hotkey {
                    out.push_str(&format!("ResearchHotkey={hk}\n"));
                }
                out.push_str(&format!("ResearchButtonpos={},{}\n", p.column(), p.row()));
            }

            if let Some(p) = off {
                let un_hotkey = existing_binding
                    .and_then(|b| b.unhotkey())
                    .map(str::to_owned)
                    .or_else(|| layout.letter_at(p.column(), p.row()).map(|c| c.to_string()));
                if let Some(hk) = un_hotkey {
                    out.push_str(&format!("Unhotkey={hk}\n"));
                }
                out.push_str(&format!("Unbuttonpos={},{}\n", p.column(), p.row()));
            }

            if let Some(tip) = existing_binding
                .and_then(|b| b.tip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.tip_levels()))
            {
                out.push_str(&format!("Tip={tip}\n"));
            }
            if let Some(un_tip) = existing_binding
                .and_then(|b| b.un_tip())
                .map(str::to_owned)
                .or_else(|| warcraft_object.un_tip().map(str::to_owned))
            {
                out.push_str(&format!("Untip={un_tip}\n"));
            }
            if let Some(ubertip) = existing_binding
                .and_then(|b| b.ubertip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.ubertip_levels()))
            {
                out.push_str(&format!("Ubertip={ubertip}\n"));
            }
            if let Some(un_ubertip) = existing_binding
                .and_then(|b| b.un_ubertip())
                .map(str::to_owned)
                .or_else(|| warcraft_object.un_ubertip().map(str::to_owned))
            {
                out.push_str(&format!("Unubertip={un_ubertip}\n"));
            }
            if let Some(res_ubertip) = existing_binding
                .and_then(|b| b.research_ubertip())
                .map(str::to_owned)
                .or_else(|| warcraft_object.research_ubertip().map(str::to_owned))
            {
                out.push_str(&format!("Researchubertip={res_ubertip}\n"));
            }

            out.push('\n');
        }

        for (object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let id = object_id.value();
            let WarcraftObjectMeta::Unit(_) = warcraft_object.meta() else {
                continue;
            };
            let Some(pos) = warcraft_object.default_button_position() else {
                continue;
            };
            let existing_binding = tmpl.binding(id);
            out.push_str(&format!("[{id}]\n"));
            if let Some(hk) = existing_binding.and_then(|b| b.hotkey()) {
                out.push_str(&format!("Hotkey={hk}\n"));
            }
            out.push_str(&format!("Buttonpos={},{}\n", pos.column(), pos.row()));
            if let Some(tip) = existing_binding
                .and_then(|b| b.tip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.tip_levels()))
            {
                out.push_str(&format!("Tip={tip}\n"));
            }
            if let Some(ubertip) = existing_binding
                .and_then(|b| b.ubertip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.ubertip_levels()))
            {
                out.push_str(&format!("Ubertip={ubertip}\n"));
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
            let Some(pos) = warcraft_object.default_button_position() else {
                continue;
            };
            let res_pos = warcraft_object.default_research_button_position();
            let existing_binding = tmpl.binding(id);

            out.push_str(&format!("[{id}]\n"));

            let hotkey = existing_binding
                .and_then(|b| b.hotkey())
                .map(str::to_owned)
                .or_else(|| {
                    layout
                        .letter_at(pos.column(), pos.row())
                        .map(|c| c.to_string())
                });
            if let Some(hk) = hotkey {
                out.push_str(&format!("Hotkey={hk}\n"));
            }
            out.push_str(&format!("Buttonpos={},{}\n", pos.column(), pos.row()));

            if let Some(p) = res_pos {
                let rh = existing_binding
                    .and_then(|b| b.research_hotkey())
                    .map(str::to_owned)
                    .or_else(|| layout.letter_at(p.column(), p.row()).map(|c| c.to_string()));
                if let Some(hk) = rh {
                    out.push_str(&format!("ResearchHotkey={hk}\n"));
                }
                out.push_str(&format!("ResearchButtonpos={},{}\n", p.column(), p.row()));
            }

            if let Some(tip) = existing_binding
                .and_then(|b| b.tip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.tip_levels()))
            {
                out.push_str(&format!("Tip={tip}\n"));
            }
            if let Some(ubertip) = existing_binding
                .and_then(|b| b.ubertip())
                .map(str::to_owned)
                .or_else(|| join_levels(warcraft_object.ubertip_levels()))
            {
                out.push_str(&format!("Ubertip={ubertip}\n"));
            }

            out.push('\n');
        }

        // System keybinds: inventory slots, hero selection, control groups,
        // camera, and all other game-level bindings. Hotkey is the default VK
        // code; Modifier and the command-class field are written for
        // transparency (the game hardcodes both and ignores any override).
        for entry in WARCRAFT_SYSTEM_KEYBINDS.iter() {
            use warcraft_api::SystemKeybindClass;
            let id = entry.section_id();
            let existing_binding = tmpl.binding(id);
            let hotkey_code = existing_binding
                .and_then(|b| b.hotkey())
                .and_then(|h| h.parse::<u32>().ok())
                .unwrap_or(entry.default_hotkey());
            out.push_str(&format!("[{id}]\n"));
            out.push_str(&format!("Hotkey={hotkey_code}\n"));
            let class_field = match entry.class() {
                SystemKeybindClass::Game => "GameCommand=1",
                SystemKeybindClass::ControlGroup => "CtrlGroupCommand=1",
                SystemKeybindClass::Menu => "MenuCommand=1",
                SystemKeybindClass::Camera => "CameraCommand=1",
                SystemKeybindClass::Observer => "ObserverCommand=1",
                SystemKeybindClass::Replay => "ReplayCommand=1",
            };
            out.push_str(&format!("{class_field}\n"));
            let modifier = match entry.default_modifier() {
                SystemKeybindModifier::None => None,
                SystemKeybindModifier::Alt => Some("Alt"),
                SystemKeybindModifier::Ctrl => Some("Ctrl"),
                SystemKeybindModifier::CtrlOrAlt => Some("Ctrl_or_Alt"),
                SystemKeybindModifier::Shift => Some("Shift"),
            };
            if let Some(m) = modifier {
                out.push_str(&format!("Modifier={m}\n"));
            }
            out.push('\n');
        }

        out
    }

    /// After applying the default template (or any template that preserves the
    /// baseline button positions), the export that goes to localStorage must
    /// have positions that agree with what the editor displays.
    ///
    /// Regression: the cascade algorithm used to let a shared ability's position
    /// be overwritten by a different unit's cascade, causing localStorage to
    /// disagree with the display.
    #[test]
    fn export_positions_match_display_after_template_apply() {
        use crate::domain::default_config::DefaultConfig;
        use warcraft_keybinds::CustomKeysFile;
        use warcraft_keybinds::cascade::fully_normalize;

        let baseline = DefaultConfig::content();

        // Simulate applying the default template: overlay template onto a fresh
        // baseline, then normalize (same steps as templates_dialog.rs).
        let mut loaded = CustomKeysFile::from(baseline);
        fully_normalize(&mut loaded);

        // Simulate what save_export → serialize produces (= localStorage content).
        let export_content = warcraft_keybinds::export::serialize(&loaded, baseline);
        let export_file = CustomKeysFile::from(export_content.as_str());

        let pos = |id: &str| {
            export_file
                .binding(id)
                .and_then(|b| b.button_position())
                .map(|p| (p.column(), p.row()))
        };

        // nfsh (Forest Troll High Priest) card: Anh2(0,2), ACif(2,2), ACd2(1,2).
        // No conflict on this card — all three positions are distinct. The export
        // must preserve each ability's template position so that both the editor
        // display and the game agree with what is stored in localStorage.
        //
        // Regression: a cross-unit cascade used to overwrite ACd2's stored
        // position (from nith's card where ACf2 also sits at 1,2), making the
        // editor display 1,2 while localStorage held a different value.
        assert_eq!(pos("ACd2"), Some((1, 2)), "ACd2 should stay at its template position 1,2");
        assert_eq!(pos("ACif"), Some((2, 2)), "ACif should stay at its template position 2,2");

        // nfsh display (resolve_container): Anh2=0,2, ACif=2,2, ACd2=1,2. No conflicts.
        // nith display (resolve_container): Anh2=0,2, ACf2=1,2, ACd2 cascades to 2,2.
        // The export stores ACd2=1,2 (its home position, valid for nfsh). The
        // display handles the nith conflict on the fly — no write-back needed.
        assert_eq!(pos("ACf2"), Some((1, 2)), "ACf2 should keep its template position");
        assert_eq!(pos("Anh2"), Some((0, 2)), "Anh2 should keep its template position");
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
