use super::fixtures;
use dioxus::prelude::*;
use gallery::Story;
use hotkey_editor::components::unit_list::UnitList;
use hotkey_editor::components::unit_list::components::mobile_category_tab::MobileCategoryTab;
use hotkey_editor::components::unit_list::components::unit_category_section::components::unit_card::UnitCard;
use hotkey_editor::components::unit_list::components::unit_category_section::components::unit_card::components::unit_card_icon::UnitCardIcon;
use hotkey_editor::components::unit_list::components::unit_category_section::components::unit_card::components::unit_card_info::UnitCardInfo;
use hotkey_editor::components::unit_list::components::unit_category_section::UnitCategorySection;
use hotkey_editor::model::icons::IconUrl;
use std::collections::HashSet;
use warcraft_api::{Race, UnitKind, WarcraftObjectMeta};
use warcraft_database::{CatalogVisibility, ObjectLookup, SearchField, UnitMode};
use warcraft_keybinds::GridSlotId;

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
    let unit_id = "hfoo".to_string();
    let race = Race::Human;
    rsx! {
        UnitCardInfo { display_name, unit_id, race, is_selected: false }
    }
}

fn unit_card_info_hero() -> Element {
    let display_name = "Archmage".to_string();
    let unit_id = "Hamg".to_string();
    let race = Race::Human;
    rsx! {
        UnitCardInfo { display_name, unit_id, race, is_selected: true }
    }
}

fn mobile_category_tab_active() -> Element {
    let kind = UnitKind::Soldier;
    let is_active = true;
    let race = Race::Orc;
    let active_category = use_signal(|| UnitKind::Soldier);
    rsx! {
        MobileCategoryTab { kind, is_active, race, active_category }
    }
}

fn mobile_category_tab_inactive() -> Element {
    let kind = UnitKind::Hero;
    let is_active = false;
    let race = Race::Human;
    let active_category = use_signal(|| UnitKind::Soldier);
    rsx! {
        MobileCategoryTab { kind, is_active, race, active_category }
    }
}

fn unit_card_icon_footman() -> Element {
    let unit_object = ObjectLookup::by_id(&fixtures::sample_unit_id());
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
    let unit_object = ObjectLookup::by_id(&fixtures::sample_unit_id());
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
    let race = Race::Human;
    let is_selected = false;
    let selected_unit_id = use_signal(|| None);
    let selected_slot: Signal<Option<GridSlotId>> = use_signal(|| None);
    let active_category = use_signal(|| UnitKind::Soldier);
    rsx! {
        UnitCard {
            unit_id,
            display_name,
            icon_path,
            unit_kind,
            race,
            is_selected,
            selected_unit_id,
            selected_slot,
            active_category,
        }
    }
}

fn unit_card_archmage_selected() -> Element {
    let unit_object = ObjectLookup::by_id(&fixtures::sample_hero_id());
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
    let race = Race::Human;
    let is_selected = true;
    let selected_unit_id = use_signal(|| Some(fixtures::sample_hero_id()));
    let selected_slot: Signal<Option<GridSlotId>> = use_signal(|| None);
    let active_category = use_signal(|| UnitKind::Hero);
    rsx! {
        UnitCard {
            unit_id,
            display_name,
            icon_path,
            unit_kind,
            race,
            is_selected,
            selected_unit_id,
            selected_slot,
            active_category,
        }
    }
}

fn unit_category_section_human_soldiers() -> Element {
    let category_kind = UnitKind::Soldier;
    let category_label = "Soldiers".to_string();
    let is_collapsed = false;
    let collapsed_categories = use_signal(HashSet::new);
    let race = Race::Human;
    let mode = UnitMode::Melee;
    let query = String::new();
    let search_field = SearchField::UnitName;
    let visibility = CatalogVisibility::new(false, false);
    let active_unit_id = Some(fixtures::sample_unit_id());
    let selected_unit_id = use_signal(|| Some(fixtures::sample_unit_id()));
    let selected_slot: Signal<Option<GridSlotId>> = use_signal(|| None);
    let active_category = use_signal(|| UnitKind::Soldier);
    rsx! {
        UnitCategorySection {
            category_kind,
            category_label,
            is_collapsed,
            collapsed_categories,
            race,
            mode,
            query,
            search_field,
            visibility,
            active_unit_id,
            selected_unit_id,
            selected_slot,
            active_category,
        }
    }
}

fn unit_list_panel_human_melee() -> Element {
    let active_race = use_signal(|| Race::Human);
    let unit_mode = use_signal(|| UnitMode::Melee);
    let selected_unit_id = use_signal(|| Some(fixtures::sample_unit_id()));
    let selected_slot: Signal<Option<GridSlotId>> = use_signal(|| None);
    let search_query = use_signal(String::new);
    let search_field = use_signal(|| SearchField::UnitName);
    let show_abilityless_units = use_signal(|| false);
    let expand_variants = use_signal(|| false);
    let collapsed_categories = use_signal(HashSet::new);
    rsx! {
        UnitList {
            active_race,
            unit_mode,
            selected_unit_id,
            selected_slot,
            search_query,
            search_field,
            show_abilityless_units,
            expand_variants,
            collapsed_categories,
        }
    }
}
