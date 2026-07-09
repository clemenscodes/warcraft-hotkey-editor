use super::editor_mount::EditorMount;
use super::fixtures;
use dioxus::prelude::*;
use dioxus_gallery::Story;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_list::UnitList;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_list::components::mobile_category_tabs::components::mobile_category_tab::MobileCategoryTab;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_list::components::category_scroll::components::category_track::components::unit_category_section::components::unit_card::UnitCard;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_list::components::category_scroll::components::category_track::components::unit_category_section::components::unit_card::components::unit_card_surface::components::shared::unit_card_icon::UnitCardIcon;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_list::components::category_scroll::components::category_track::components::unit_category_section::components::unit_card::components::unit_card_surface::components::shared::unit_card_info::UnitCardInfo;
use hotkey_editor::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_list::components::category_scroll::components::category_track::components::unit_category_section::UnitCategorySection;
use hotkey_editor::components::app::components::shell::components::shared::icons::IconUrl;
use warcraft_api::{ObjectLookup, Race, UnitKind, WarcraftObjectMeta};

pub fn stories() -> Vec<Story> {
    vec![
        Story::new(
            "Unit list",
            "UnitCardInfo",
            "Footman",
            unit_card_info_footman,
        ),
        Story::new("Unit list", "UnitCardInfo", "Hero", unit_card_info_hero),
        Story::new(
            "Unit list",
            "MobileCategoryTab",
            "Active",
            mobile_category_tab_active,
        ),
        Story::new(
            "Unit list",
            "MobileCategoryTab",
            "Inactive",
            mobile_category_tab_inactive,
        ),
        Story::single("Unit list", "UnitCardIcon", unit_card_icon_footman),
        Story::new("Unit list", "UnitCard", "Footman", unit_card_footman),
        Story::new(
            "Unit list",
            "UnitCard",
            "Archmage (selected)",
            unit_card_archmage_selected,
        ),
        Story::single(
            "Unit list",
            "UnitCategorySection",
            unit_category_section_human_soldiers,
        ),
        Story::single("Unit list", "UnitList", unit_list_panel_human_melee),
    ]
}

fn unit_card_info_footman() -> Element {
    let display_name = "Footman".to_string();
    let unit_id = ObjectLookup::resolve_raw("hfoo").expect("known object id");
    rsx! {
        UnitCardInfo { display_name, unit_id, is_selected: false }
    }
}

fn unit_card_info_hero() -> Element {
    let display_name = "Archmage".to_string();
    let unit_id = ObjectLookup::resolve_raw("Hamg").expect("known object id");
    rsx! {
        div {
            class: "[--race-accent:var(--color-race-human)]",
            UnitCardInfo { display_name, unit_id, is_selected: true }
        }
    }
}

fn mobile_category_tab_active() -> Element {
    rsx! {
        EditorMount {
            MobileCategoryTab { kind: UnitKind::Soldier }
        }
    }
}

fn mobile_category_tab_inactive() -> Element {
    rsx! {
        EditorMount {
            MobileCategoryTab { kind: UnitKind::Hero }
        }
    }
}

fn unit_card_icon_footman() -> Element {
    let unit_object = ObjectLookup::object(fixtures::sample_unit_id());
    let icon_path = unit_object.and_then(|obj| {
        obj.icons()
            .first()
            .copied()
            .map(IconUrl::from_database_path)
    });
    let display_name = "Footman".to_string();
    rsx! {
        UnitCardIcon { icon_path, display_name }
    }
}

fn unit_card_footman() -> Element {
    let unit_object = ObjectLookup::object(fixtures::sample_unit_id());
    let Some(unit_object) = unit_object else {
        return rsx! { "Footman not found in database." };
    };
    let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
        return rsx! { "Unexpected object kind." };
    };
    let display_name = unit_object
        .names()
        .first()
        .copied()
        .unwrap_or("Footman")
        .to_string();
    let icon_path = unit_object
        .icons()
        .first()
        .copied()
        .map(IconUrl::from_database_path);
    let unit_id = fixtures::sample_unit_id();
    let unit_kind = unit_meta.unit_kind();
    rsx! {
        EditorMount {
            UnitCard { unit_id, display_name, icon_path, unit_kind }
        }
    }
}

fn unit_card_archmage_selected() -> Element {
    let unit_object = ObjectLookup::object(fixtures::sample_hero_id());
    let Some(unit_object) = unit_object else {
        return rsx! { "Archmage not found in database." };
    };
    let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
        return rsx! { "Unexpected object kind." };
    };
    let display_name = unit_object
        .names()
        .first()
        .copied()
        .unwrap_or("Archmage")
        .to_string();
    let icon_path = unit_object
        .icons()
        .first()
        .copied()
        .map(IconUrl::from_database_path);
    let unit_id = fixtures::sample_hero_id();
    let unit_kind = unit_meta.unit_kind();
    let selected_unit_id = Some(fixtures::sample_hero_id());
    rsx! {
        EditorMount {
            active_race: Race::Human,
            selected_unit_id,
            div {
                class: "[--race-color:var(--color-race-human)] [--race-accent:var(--color-race-human)]",
                UnitCard { unit_id, display_name, icon_path, unit_kind }
            }
        }
    }
}

fn unit_category_section_human_soldiers() -> Element {
    let selected_unit_id = Some(fixtures::sample_unit_id());
    rsx! {
        EditorMount {
            selected_unit_id,
            UnitCategorySection { category_kind: UnitKind::Soldier }
        }
    }
}

fn unit_list_panel_human_melee() -> Element {
    let selected_unit_id = Some(fixtures::sample_unit_id());
    rsx! {
        EditorMount {
            active_race: Race::Human,
            selected_unit_id,
            UnitList {}
        }
    }
}
