use std::collections::HashMap;
use std::rc::Rc;

use dioxus::prelude::*;
use num_traits::cast::cast;
use warcraft_api::{
    AttackType, DefenseType, HeroAttributes, PrimaryAttribute, RegenType, UnitAttack, UnitCombat,
    UnitKind, WarcraftObjectId, WarcraftObjectMeta,
};
use warcraft_database::WARCRAFT_GAMEPLAY_CONSTANTS;
use warcraft_keybinds::CustomKeysFile;

use crate::components::command_grid::{CommandGridSection, CommandGridSectionProps};
use crate::components::tile_override::TileOverridePanel;
use crate::domain::building_traits::BuildingTraits;
use crate::domain::command_catalog::CommandCatalog;
use crate::domain::grid_layout::GridLayout;
use crate::domain::grid_slot::{DragFollower, DraggingSlot, DropTargetCell, GridSlotId};
use crate::domain::icons::IconUrl;
use crate::domain::inspector_detail::InspectorDetail;
use crate::domain::object_lookup::ObjectLookup;
use crate::domain::unit_kind::UnitKindHelpers;

const MAX_HERO_LEVEL_DISPLAY: u32 = 10;

// `infocard-neutral-*` and `infocard-heroattributes-*` textures are the
// creep-panel / observer-panel variants — they have no opaque level box
// baked into the corner (verified via alpha-channel inspection: BR alpha
// ~0.38 vs ~0.91 for the upgrade-tier counterparts).
//
// Hero / Divine defense reuse the gold knight helm with blue eyes
// (`infocard-neutral-attack-hero`) — same imagery in-game for both armor
// types.
const ICON_STRENGTH: Asset = asset!("/assets/webui/infocard/infocard-heroattributes-str.png");
const ICON_AGILITY: Asset = asset!("/assets/webui/infocard/infocard-heroattributes-agi.png");
const ICON_INTELLIGENCE: Asset = asset!("/assets/webui/infocard/infocard-heroattributes-int.png");
const ICON_ATTACK_MELEE: Asset = asset!("/assets/webui/infocard/infocard-neutral-attack-melee.png");
const ICON_ATTACK_PIERCING: Asset =
    asset!("/assets/webui/infocard/infocard-neutral-attack-piercing.png");
const ICON_ATTACK_SIEGE: Asset = asset!("/assets/webui/infocard/infocard-neutral-attack-siege.png");
const ICON_ATTACK_MAGIC: Asset = asset!("/assets/webui/infocard/infocard-neutral-attack-magic.png");
const ICON_ATTACK_CHAOS: Asset = asset!("/assets/webui/infocard/infocard-neutral-attack-chaos.png");
const ICON_ATTACK_HERO: Asset = asset!("/assets/webui/infocard/infocard-neutral-attack-hero.png");
const ICON_ARMOR_SMALL: Asset = asset!("/assets/webui/infocard/infocard-neutral-armor-small.png");
const ICON_ARMOR_MEDIUM: Asset = asset!("/assets/webui/infocard/infocard-neutral-armor-medium.png");
const ICON_ARMOR_LARGE: Asset = asset!("/assets/webui/infocard/infocard-neutral-armor-large.png");
const ICON_ARMOR_FORTIFIED: Asset =
    asset!("/assets/webui/infocard/infocard-neutral-armor-fortified.png");
const ICON_ARMOR_UNARMORED: Asset =
    asset!("/assets/webui/infocard/infocard-neutral-armor-unarmored.png");
// Hero/Divine share the same gold knight helm imagery. `infocard-armor-hero.dds`
// bakes in a level-overlay box at bottom-right; CSS `mask-image` on `.stat-icon`
// hides that box at render time.
const ICON_ARMOR_HERO: Asset = asset!("/assets/webui/infocard/infocard-armor-hero.png");

#[derive(Clone, Copy)]
struct StatIcon {
    asset: Asset,
}

impl StatIcon {
    fn new(asset: Asset) -> Self {
        Self { asset }
    }

    fn asset(&self) -> Asset {
        self.asset
    }
}

impl From<AttackType> for StatIcon {
    fn from(attack_type: AttackType) -> Self {
        let resolved = match attack_type {
            AttackType::Normal => ICON_ATTACK_MELEE,
            AttackType::Pierce => ICON_ATTACK_PIERCING,
            AttackType::Siege => ICON_ATTACK_SIEGE,
            AttackType::Magic | AttackType::Spells => ICON_ATTACK_MAGIC,
            AttackType::Chaos => ICON_ATTACK_CHAOS,
            AttackType::Hero => ICON_ATTACK_HERO,
            AttackType::Unknown => ICON_ATTACK_MELEE,
        };
        Self::new(resolved)
    }
}

impl From<DefenseType> for StatIcon {
    fn from(defense_type: DefenseType) -> Self {
        // Hero/Divine share the same icon — neither has an `infocard-neutral-armor-*` variant in CASC.
        let resolved = match defense_type {
            DefenseType::Light => ICON_ARMOR_SMALL,
            DefenseType::Medium | DefenseType::Normal => ICON_ARMOR_MEDIUM,
            DefenseType::Heavy => ICON_ARMOR_LARGE,
            DefenseType::Fortified => ICON_ARMOR_FORTIFIED,
            DefenseType::Hero | DefenseType::Divine => ICON_ARMOR_HERO,
            DefenseType::Unarmored => ICON_ARMOR_UNARMORED,
        };
        Self::new(resolved)
    }
}

impl From<PrimaryAttribute> for StatIcon {
    fn from(primary: PrimaryAttribute) -> Self {
        let resolved = match primary {
            PrimaryAttribute::Strength => ICON_STRENGTH,
            PrimaryAttribute::Agility => ICON_AGILITY,
            PrimaryAttribute::Intelligence => ICON_INTELLIGENCE,
        };
        Self::new(resolved)
    }
}

#[component]
pub(crate) fn UnitDetailPanel(
    selected_unit_id: Signal<Option<String>>,
    selected_slot: Signal<Option<GridSlotId>>,
    selected_from_research: Signal<bool>,
    selected_from_uprooted: Signal<bool>,
    tier_overrides: Signal<HashMap<String, usize>>,
    dragging_slot: Signal<Option<DraggingSlot>>,
    drop_target_cell: Signal<Option<DropTargetCell>>,
    drag_follower: Signal<Option<DragFollower>>,
    loaded_keys: Signal<Option<CustomKeysFile>>,
    grid_layout: Signal<GridLayout>,
) -> Element {
    let mut selected_hero_level = use_signal::<u32>(|| 1);
    let mut level_picker_open = use_signal::<bool>(|| false);
    use_effect(move || {
        let _ = selected_unit_id.read();
        selected_hero_level.set(1);
        level_picker_open.set(false);
    });

    let unit_id_option = selected_unit_id.read().clone();
    let Some(unit_id) = unit_id_option else {
        return rsx! {
            section { class: "unit-detail empty",
                "Select a unit to view its command card."
            }
        };
    };

    let Some(unit_object) = ObjectLookup::by_id(&unit_id) else {
        return rsx! {
            section { class: "unit-detail empty", "Unit not found in database." }
        };
    };

    let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
        return rsx! {
            section { class: "unit-detail empty", "Selected object is not a unit." }
        };
    };

    let unit_name = unit_object.names().first().copied().unwrap_or("(unnamed)");
    let portrait_url = unit_object
        .icons()
        .first()
        .copied()
        .map(IconUrl::from_database_path);

    let regular_abilities = unit_meta.abilities();
    let hero_abilities = unit_meta.hero_abilities();
    let primary_commands =
        CommandCatalog::primary_commands_for(unit_meta, unit_object.race(), &unit_id);
    let unit_kind = UnitKindHelpers::effective_kind(unit_meta);

    let primary_train_slots: &[WarcraftObjectId] = if unit_kind == UnitKind::Building {
        unit_meta.trains()
    } else {
        &[]
    };
    let primary_research_slots: &[WarcraftObjectId] = if unit_kind == UnitKind::Building {
        unit_meta.researches()
    } else {
        &[]
    };

    let sell_items: &[WarcraftObjectId] = if unit_kind == UnitKind::Building {
        unit_meta.sell_items()
    } else {
        &[]
    };
    let sell_units: &[WarcraftObjectId] = if unit_kind == UnitKind::Building {
        unit_meta.sell_units()
    } else {
        &[]
    };

    let mut command_card_slots: Vec<GridSlotId> = Vec::with_capacity(
        primary_train_slots.len()
            + primary_research_slots.len()
            + regular_abilities.len()
            + primary_commands.len()
            + sell_items.len()
            + sell_units.len(),
    );
    for command_name in primary_commands {
        if !ObjectLookup::has_icon(command_name) {
            continue;
        }
        command_card_slots.push(GridSlotId::command(command_name));
    }
    for trained_id in primary_train_slots {
        if !ObjectLookup::has_icon(trained_id.value()) {
            continue;
        }
        command_card_slots.push(GridSlotId::ability(trained_id.value()));
    }
    for research_id in primary_research_slots {
        if !ObjectLookup::has_icon(research_id.value()) {
            continue;
        }
        command_card_slots.push(GridSlotId::ability(research_id.value()));
    }
    for sell_item_id in sell_items {
        if !ObjectLookup::has_icon(sell_item_id.value()) {
            continue;
        }
        command_card_slots.push(GridSlotId::ability(sell_item_id.value()));
    }
    for sell_unit_id in sell_units {
        if !ObjectLookup::has_icon(sell_unit_id.value()) {
            continue;
        }
        command_card_slots.push(GridSlotId::ability(sell_unit_id.value()));
    }
    let is_uprootable = BuildingTraits::can_uproot(&unit_id);
    let host_is_burrowed = BuildingTraits::is_burrowed_form(&unit_id);
    for ability_id in regular_abilities.iter().chain(hero_abilities.iter()) {
        if is_uprootable && ability_id.value().eq_ignore_ascii_case("Aeat") {
            continue;
        }
        if host_is_burrowed && !BuildingTraits::ability_has_alt_state(ability_id.value()) {
            continue;
        }
        if !ObjectLookup::has_icon(ability_id.value()) {
            continue;
        }
        command_card_slots.push(GridSlotId::ability(ability_id.value()));
    }
    if unit_kind == UnitKind::Hero
        && !hero_abilities.is_empty()
        && let Some(select_skill_command) = CommandCatalog::known_command("CmdSelectSkill")
        && ObjectLookup::has_icon(select_skill_command)
    {
        command_card_slots.push(GridSlotId::command(select_skill_command));
    }
    let command_card_slots_rc: Rc<[GridSlotId]> = command_card_slots.into();

    let build_menu_slots_rc: Option<Rc<[GridSlotId]>> =
        if unit_kind == UnitKind::Worker && !unit_meta.builds().is_empty() {
            let unit_builds = unit_meta.builds();
            let build_menu_commands = CommandCatalog::build_menu_commands_for(unit_meta);
            let mut build_menu_slots: Vec<GridSlotId> =
                Vec::with_capacity(unit_builds.len() + build_menu_commands.len());
            for command_name in build_menu_commands {
                if !ObjectLookup::has_icon(command_name) {
                    continue;
                }
                build_menu_slots.push(GridSlotId::command(command_name));
            }
            for production_id in unit_builds {
                if !ObjectLookup::has_icon(production_id.value()) {
                    continue;
                }
                build_menu_slots.push(GridSlotId::ability(production_id.value()));
            }
            Some(build_menu_slots.into())
        } else {
            None
        };

    let uprooted_menu_slots_rc: Option<Rc<[GridSlotId]>> =
        if unit_kind == UnitKind::Building && BuildingTraits::can_uproot(&unit_id) {
            let mut uprooted_slots: Vec<GridSlotId> = Vec::new();
            for command_name in CommandCatalog::mobile_command_ids().iter().copied() {
                if !ObjectLookup::has_icon(command_name) {
                    continue;
                }
                uprooted_slots.push(GridSlotId::command(command_name));
            }
            for ability_id in regular_abilities {
                if !ObjectLookup::has_icon(ability_id.value()) {
                    continue;
                }
                uprooted_slots.push(GridSlotId::ability(ability_id.value()));
            }
            Some(uprooted_slots.into())
        } else {
            None
        };

    let research_menu_slots_rc: Option<Rc<[GridSlotId]>> = if unit_kind == UnitKind::Hero
        && !hero_abilities.is_empty()
    {
        let mut research_menu_slots: Vec<GridSlotId> = Vec::with_capacity(hero_abilities.len() + 1);
        for ability_id in hero_abilities.iter() {
            if !ObjectLookup::has_icon(ability_id.value()) {
                continue;
            }
            research_menu_slots.push(GridSlotId::ability(ability_id.value()));
        }
        if let Some(cancel_command) = CommandCatalog::known_command("CmdCancel")
            && ObjectLookup::has_icon(cancel_command)
        {
            research_menu_slots.push(GridSlotId::command(cancel_command));
        }
        Some(research_menu_slots.into())
    } else {
        None
    };

    let inspector_slot = selected_slot.read().clone();
    let inspector_from_uprooted = *selected_from_uprooted.read();
    let inspector_from_research = *selected_from_research.read();
    let inspector_panel = inspector_slot.as_ref().map(|slot| {
        InspectorDetail::build(slot, &loaded_keys.read(), &unit_id, inspector_from_uprooted)
    });
    let empty_slot_list: Rc<[GridSlotId]> = Rc::from(Vec::<GridSlotId>::new());
    let active_container_slots: Rc<[GridSlotId]> = if inspector_from_uprooted {
        uprooted_menu_slots_rc
            .clone()
            .unwrap_or_else(|| empty_slot_list.clone())
    } else if inspector_from_research {
        research_menu_slots_rc
            .clone()
            .unwrap_or_else(|| empty_slot_list.clone())
    } else {
        let inspector_slot_id = inspector_slot
            .as_ref()
            .map(|slot| slot.as_str().to_string());
        let in_build_menu = inspector_slot_id.as_deref().is_some_and(|id_value| {
            build_menu_slots_rc.as_ref().is_some_and(|list| {
                list.iter()
                    .any(|candidate| candidate.as_str().eq_ignore_ascii_case(id_value))
            })
        });
        if in_build_menu {
            build_menu_slots_rc
                .clone()
                .unwrap_or_else(|| empty_slot_list.clone())
        } else {
            command_card_slots_rc.clone()
        }
    };

    let command_card_props = CommandGridSectionProps {
        heading: "Command card",
        slot_ids: command_card_slots_rc,
        loaded_keys,
        selected_slot,
        selected_from_research,
        selected_from_uprooted,
        tier_overrides,
        dragging_slot,
        drop_target_cell,
        drag_follower,
        grid_layout,
        is_research_grid: false,
        is_uprooted_grid: false,
    };

    let unit_description = unit_object.ubertip();
    let header_hero_attributes = unit_meta.hero_attributes();
    let header_current_level = if header_hero_attributes.is_some() {
        selected_hero_level()
    } else {
        1
    };

    rsx! {
        section { class: "unit-detail",
            header { class: "unit-detail-header",
                if let Some(source) = portrait_url {
                    img { class: "unit-portrait", src: "{source}", alt: "{unit_name}", loading: "lazy", decoding: "async" }
                }
                div { class: "unit-detail-title",
                    div { class: "unit-name-row",
                        h2 { "{unit_name}" }
                        if header_hero_attributes.is_some() {
                            div { class: "hero-level-picker",
                                button {
                                    class: if level_picker_open() { "hero-level-trigger open" } else { "hero-level-trigger" },
                                    r#type: "button",
                                    onclick: move |_| level_picker_open.set(!level_picker_open()),
                                    span { class: "hero-level-trigger-label", "Level" }
                                    span { class: "hero-level-trigger-number", "{header_current_level}" }
                                    span { class: "hero-level-trigger-chevron", "▾" }
                                }
                                if level_picker_open() {
                                    div { class: "hero-level-menu",
                                        for level_index in 1..=MAX_HERO_LEVEL_DISPLAY {
                                            {
                                                let is_active = level_index == header_current_level;
                                                rsx! {
                                                    button {
                                                        key: "{level_index}",
                                                        class: if is_active { "hero-level-option active" } else { "hero-level-option" },
                                                        r#type: "button",
                                                        onclick: move |_| {
                                                            selected_hero_level.set(level_index);
                                                            level_picker_open.set(false);
                                                        },
                                                        "Level {level_index}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    div {
                                        class: "hero-level-backdrop",
                                        onclick: move |_| level_picker_open.set(false),
                                    }
                                }
                            }
                        }
                    }
                    code { "{unit_id}" }
                }
            }

            p { class: "unit-description", "{unit_description.unwrap_or_default()}" }

            {
                let combat = unit_meta.combat();
                let hero_attributes = unit_meta.hero_attributes();
                let attack_option = combat.attack();
                let current_level = if hero_attributes.is_some() { selected_hero_level() } else { 1 };
                let leveled_stats = hero_attributes.map(|attributes| LeveledStats::for_hero(combat, attributes, current_level));
                let display_hp = leveled_stats.as_ref().map(|stats| stats.hit_points()).unwrap_or_else(|| combat.hit_points());
                let display_mana = if hero_attributes.is_some() {
                    Some(leveled_stats.as_ref().map(|stats| stats.mana()).unwrap_or(0))
                } else {
                    combat.mana_pool().filter(|mp| mp.mana() > 0).map(|mp| mp.mana())
                };
                let display_armor = leveled_stats.as_ref().map(|stats| stats.armor()).unwrap_or_else(|| combat.armor());
                let leveled_damage_min = leveled_stats.as_ref().map(|stats| stats.damage_min());
                let leveled_damage_max = leveled_stats.as_ref().map(|stats| stats.damage_max());
                let armor_text = format!("{display_armor:.0}");
                let defense_label = combat.defense_type().to_string();
                let has_attack = attack_option.is_some();
                rsx! {
                    div { class: "unit-stats-panel",
                        div { class: "stat-column vitality-column",
                            if display_hp > 0 {
                                {
                                    let hp_regen = leveled_stats
                                        .as_ref()
                                        .map(LeveledStats::hit_points_regen)
                                        .unwrap_or_else(|| combat.hit_points_regen());
                                    let regen_text = format!("+{hp_regen:.2}");
                                    let regen_qualifier_text = match combat.regen_type() {
                                        RegenType::Night => Some("at night"),
                                        RegenType::Blight => Some("on blight"),
                                        RegenType::Always | RegenType::None => None,
                                    };
                                    let has_regen = hp_regen > 0.0 && combat.regen_type() != RegenType::None;
                                    rsx! {
                                        div { class: "stat-row hp",
                                            span { class: "stat-row-label", "Hit Points" }
                                            span { class: "stat-row-value", "{display_hp}" }
                                        }
                                        div { class: "stat-row regen-row",
                                            span { class: "stat-row-label", "Regeneration" }
                                            if has_regen {
                                                if let Some(qualifier_text) = regen_qualifier_text {
                                                    span { class: "regen-qualifier", "{qualifier_text}" }
                                                }
                                                span { class: "stat-row-gain", "{regen_text}" }
                                            } else {
                                                span { class: "stat-row-gain stat-zero", "+0.00" }
                                            }
                                        }
                                    }
                                }
                            }
                            {
                                let mana_display = display_mana.unwrap_or(0);
                                let has_mana = display_mana.map(|v| v > 0).unwrap_or(false);
                                let mana_regen = leveled_stats
                                    .as_ref()
                                    .map(LeveledStats::mana_regen)
                                    .unwrap_or_else(|| {
                                        hero_attributes.map(HeroAttributes::mana_regen)
                                            .unwrap_or_else(|| combat.mana_pool().map(|mp| mp.mana_regen()).unwrap_or(0.0))
                                    });
                                let has_mana_regen = has_mana && mana_regen > 0.0;
                                let mana_regen_text = if has_mana_regen { format!("+{mana_regen:.2}") } else { "+0.00".to_string() };
                                let mana_value_class = if has_mana { "stat-row-value" } else { "stat-row-value stat-zero" };
                                let mana_regen_class = if has_mana_regen { "stat-row-gain" } else { "stat-row-gain stat-zero" };
                                rsx! {
                                    div { class: "stat-row mana",
                                        span { class: "stat-row-label", "Mana" }
                                        span { class: "{mana_value_class}", "{mana_display}" }
                                    }
                                    div { class: "stat-row regen-row mana",
                                        span { class: "stat-row-label", "Regeneration" }
                                        span { class: "{mana_regen_class}", "{mana_regen_text}" }
                                    }
                                }
                            }
                        }
                        if let Some(unit_attack) = attack_option {
                            {
                                let damage_min = leveled_damage_min.unwrap_or_else(|| unit_attack.damage_min());
                                let damage_max = leveled_damage_max.unwrap_or_else(|| unit_attack.damage_max());
                                let attack_range = unit_attack.range();
                                let attack_speed_text = format!("{:.2}s", unit_attack.cooldown_seconds());
                                let attack_type_label = unit_attack.attack_type().to_string();
                                let attack_icon = StatIcon::from(unit_attack.attack_type()).asset();
                                rsx! {
                                    div { class: "stat-column combat-column with-icon",
                                        div { class: "stat-icon-frame",
                                            img { class: "stat-icon", src: "{attack_icon}", alt: "{attack_type_label} attack icon" }
                                        }
                                        div { class: "stat-rows",
                                            div { class: "stat-row",
                                                span { class: "stat-row-label", "Damage" }
                                                span { class: "stat-row-value", "{damage_min}\u{2013}{damage_max}" }
                                            }
                                            if attack_range > 0 {
                                                div { class: "stat-row",
                                                    span { class: "stat-row-label", "Range" }
                                                    span { class: "stat-row-value", "{attack_range}" }
                                                }
                                            }
                                            div { class: "stat-row",
                                                span { class: "stat-row-label", "Attack Speed" }
                                                span { class: "stat-row-value", "{attack_speed_text}" }
                                            }
                                            div { class: "stat-row",
                                                span { class: "stat-row-label", "Attack Type" }
                                                span { class: "stat-row-value", "{attack_type_label}" }
                                            }
                                            DamageMatchupRow { attack_type: unit_attack.attack_type() }
                                        }
                                    }
                                }
                            }
                        }
                        {
                            let defense_icon = StatIcon::from(combat.defense_type()).asset();
                            rsx! {
                                div { class: "stat-column defense-column with-icon",
                                    div { class: "stat-icon-frame",
                                        img { class: "stat-icon", src: "{defense_icon}", alt: "{defense_label} defense icon" }
                                    }
                                    div { class: "stat-rows",
                                        div { class: "stat-row",
                                            span { class: "stat-row-label", "Armor" }
                                            span { class: "stat-row-value", "{armor_text}" }
                                        }
                                        div { class: "stat-row",
                                            span { class: "stat-row-label", "Defense Type" }
                                            span { class: "stat-row-value", "{defense_label}" }
                                        }
                                        if !has_attack {
                                            div { class: "stat-row", "\u{00a0}" }
                                            div { class: "stat-row", "\u{00a0}" }
                                        }
                                        DefenseMatchupRow { defense_type: combat.defense_type() }
                                    }
                                }
                            }
                        }
                        if let Some(attributes) = hero_attributes
                            && let Some(stats) = leveled_stats.as_ref()
                        {
                            {
                                let primary = attributes.primary();
                                let primary_icon = StatIcon::from(primary).asset();
                                let primary_label = primary.to_string();
                                rsx! {
                                    div { class: "stat-column attributes-column with-icon",
                                        div { class: "stat-icon-frame",
                                            img { class: "stat-icon", src: "{primary_icon}", alt: "{primary_label} primary attribute icon" }
                                        }
                                        div { class: "stat-rows",
                                            AttributeRow {
                                                label: "Strength",
                                                value: stats.strength(),
                                                per_level: attributes.strength_per_level(),
                                                is_primary: primary == PrimaryAttribute::Strength,
                                            }
                                            AttributeRow {
                                                label: "Agility",
                                                value: stats.agility(),
                                                per_level: attributes.agility_per_level(),
                                                is_primary: primary == PrimaryAttribute::Agility,
                                            }
                                            AttributeRow {
                                                label: "Intelligence",
                                                value: stats.intelligence(),
                                                per_level: attributes.intelligence_per_level(),
                                                is_primary: primary == PrimaryAttribute::Intelligence,
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                }
            }

            div { class: "unit-detail-body",
                div { class: "unit-detail-row",
                    div { class: "unit-detail-grids",
                        CommandGridSection { ..command_card_props }

                        if let Some(build_menu_ids) = build_menu_slots_rc {
                            {
                                let build_menu_props = CommandGridSectionProps {
                                    heading: "Build menu",
                                    slot_ids: build_menu_ids,
                                    loaded_keys,
                                    selected_slot,
                                    selected_from_research,
                                    selected_from_uprooted,
                                    tier_overrides,
                                    dragging_slot,
                                    drop_target_cell,
                                    drag_follower,
                                    grid_layout,
                                    is_research_grid: false,
                                    is_uprooted_grid: false,
                                };
                                rsx! { CommandGridSection { ..build_menu_props } }
                            }
                        }

                        if let Some(uprooted_menu_ids) = uprooted_menu_slots_rc {
                            {
                                let uprooted_props = CommandGridSectionProps {
                                    heading: "Uprooted",
                                    slot_ids: uprooted_menu_ids,
                                    loaded_keys,
                                    selected_slot,
                                    selected_from_research,
                                    selected_from_uprooted,
                                    tier_overrides,
                                    dragging_slot,
                                    drop_target_cell,
                                    drag_follower,
                                    grid_layout,
                                    is_research_grid: false,
                                    is_uprooted_grid: true,
                                };
                                rsx! { CommandGridSection { ..uprooted_props } }
                            }
                        }

                        if let Some(research_menu_ids) = research_menu_slots_rc {
                            {
                                let research_props = CommandGridSectionProps {
                                    heading: "Research menu",
                                    slot_ids: research_menu_ids,
                                    loaded_keys,
                                    selected_slot,
                                    selected_from_research,
                                    selected_from_uprooted,
                                    tier_overrides,
                                    dragging_slot,
                                    drop_target_cell,
                                    drag_follower,
                                    grid_layout,
                                    is_research_grid: true,
                                    is_uprooted_grid: false,
                                };
                                rsx! { CommandGridSection { ..research_props } }
                            }
                        }
                    }

                    aside { class: "tile-override-panel",
                        h3 { class: "command-section-heading", "Hotkey override" }
                        if let Some(detail) = inspector_panel.clone() {
                            TileOverridePanel {
                                detail,
                                loaded_keys,
                                grid_layout,
                                selected_from_research,
                                selected_from_uprooted,
                                tier_overrides,
                                active_container_slots: active_container_slots.clone(),
                            }
                        } else {
                            div { class: "tile-override-empty",
                                p { "Select a tile in the grid to override its hotkey." }
                            }
                        }
                    }
                }

            }
        }
    }
}

const ALL_ATTACK_TYPES: [AttackType; 7] = [
    AttackType::Normal,
    AttackType::Pierce,
    AttackType::Siege,
    AttackType::Magic,
    AttackType::Chaos,
    AttackType::Hero,
    AttackType::Spells,
];

fn matchup_cell_class_attacking(multiplier: f32) -> &'static str {
    if multiplier > 1.05 {
        "matchup-cell strong"
    } else if multiplier < 0.95 {
        "matchup-cell weak"
    } else {
        "matchup-cell"
    }
}

fn matchup_cell_class_defending(multiplier: f32) -> &'static str {
    if multiplier > 1.05 {
        "matchup-cell weak"
    } else if multiplier < 0.95 {
        "matchup-cell strong"
    } else {
        "matchup-cell"
    }
}

fn percent_label(multiplier: f32) -> String {
    let percent_int: i32 = cast::<f32, i32>((multiplier * 100.0).round()).unwrap_or(0);
    format!("{percent_int}%")
}

// `Normal` defense exists in the WC3 combat math but no shipping unit uses
// it (verified: zero `DefenseType::Normal` rows in the database). Showing
// a row with always-100% multipliers adds noise without value, so we
// filter it out of both matchup grids.
const DISPLAYED_DEFENSE_TYPES: [DefenseType; 7] = [
    DefenseType::Light,
    DefenseType::Medium,
    DefenseType::Heavy,
    DefenseType::Fortified,
    DefenseType::Hero,
    DefenseType::Divine,
    DefenseType::Unarmored,
];

#[component]
fn DamageMatchupRow(attack_type: AttackType) -> Element {
    let effectiveness = WARCRAFT_GAMEPLAY_CONSTANTS.damage_effectiveness(attack_type);
    rsx! {
        div { class: "damage-matchup",
            for defense_type in DISPLAYED_DEFENSE_TYPES {
                {
                    let multiplier = effectiveness.against(defense_type);
                    let percent_text = percent_label(multiplier);
                    let cell_class = matchup_cell_class_attacking(multiplier);
                    let defense_label = defense_type.to_string();
                    rsx! {
                        div { class: "{cell_class}", title: "vs {defense_label}",
                            span { class: "matchup-label", "{defense_label}" }
                            span { class: "matchup-value", "{percent_text}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DefenseMatchupRow(defense_type: DefenseType) -> Element {
    rsx! {
        div { class: "damage-matchup",
            for attack_type in ALL_ATTACK_TYPES {
                {
                    let effectiveness = WARCRAFT_GAMEPLAY_CONSTANTS.damage_effectiveness(attack_type);
                    let multiplier = effectiveness.against(defense_type);
                    let percent_text = percent_label(multiplier);
                    let cell_class = matchup_cell_class_defending(multiplier);
                    let attack_label = attack_type.to_string();
                    rsx! {
                        div { class: "{cell_class}", title: "{attack_label} attacks",
                            span { class: "matchup-label", "{attack_label}" }
                            span { class: "matchup-value", "{percent_text}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AttributeRow(label: &'static str, value: u32, per_level: f32, is_primary: bool) -> Element {
    let row_class = if is_primary {
        "stat-row attribute-row primary"
    } else {
        "stat-row attribute-row"
    };
    let per_level_text = format!("+{per_level:.1}");
    rsx! {
        div { class: "{row_class}",
            span { class: "stat-row-label", "{label}" }
            span { class: "stat-row-value", "{value}" }
            span { class: "stat-row-gain", "{per_level_text}" }
        }
    }
}

struct LeveledStats {
    strength: u32,
    agility: u32,
    intelligence: u32,
    hit_points: u32,
    hit_points_regen: f32,
    mana: u32,
    mana_regen: f32,
    armor: f32,
    damage_min: u32,
    damage_max: u32,
}

impl LeveledStats {
    fn for_hero(combat: &UnitCombat, attributes: &HeroAttributes, level: u32) -> Self {
        let levels_added = level.saturating_sub(1);
        let levels_added_f: f32 = cast(levels_added).unwrap_or(0.0);
        let base_strength_f: f32 = cast(attributes.strength()).unwrap_or(0.0);
        let base_agility_f: f32 = cast(attributes.agility()).unwrap_or(0.0);
        let base_intelligence_f: f32 = cast(attributes.intelligence()).unwrap_or(0.0);
        let strength_total_f = base_strength_f + attributes.strength_per_level() * levels_added_f;
        let agility_total_f = base_agility_f + attributes.agility_per_level() * levels_added_f;
        let intelligence_total_f =
            base_intelligence_f + attributes.intelligence_per_level() * levels_added_f;
        let strength: u32 =
            cast::<f32, u32>(strength_total_f.floor()).unwrap_or(attributes.strength());
        let agility: u32 =
            cast::<f32, u32>(agility_total_f.floor()).unwrap_or(attributes.agility());
        let intelligence: u32 =
            cast::<f32, u32>(intelligence_total_f.floor()).unwrap_or(attributes.intelligence());
        let strength_delta = strength.saturating_sub(attributes.strength());
        let intelligence_delta = intelligence.saturating_sub(attributes.intelligence());
        let agility_delta_f: f32 =
            cast(agility.saturating_sub(attributes.agility())).unwrap_or(0.0);
        let strength_f: f32 = cast(strength).unwrap_or(0.0);
        let intelligence_f: f32 = cast(intelligence).unwrap_or(0.0);
        let hit_points = combat.hit_points()
            + strength_delta * WARCRAFT_GAMEPLAY_CONSTANTS.str_hit_point_bonus();
        let hit_points_regen =
            combat.hit_points_regen() + strength_f * WARCRAFT_GAMEPLAY_CONSTANTS.str_regen_bonus();
        let mana =
            attributes.mana() + intelligence_delta * WARCRAFT_GAMEPLAY_CONSTANTS.int_mana_bonus();
        let mana_regen = attributes.mana_regen()
            + intelligence_f * WARCRAFT_GAMEPLAY_CONSTANTS.int_regen_bonus();
        let armor =
            combat.armor() + agility_delta_f * WARCRAFT_GAMEPLAY_CONSTANTS.agi_defense_bonus();
        let primary_now = match attributes.primary() {
            PrimaryAttribute::Strength => strength,
            PrimaryAttribute::Agility => agility,
            PrimaryAttribute::Intelligence => intelligence,
        };
        let primary_base = match attributes.primary() {
            PrimaryAttribute::Strength => attributes.strength(),
            PrimaryAttribute::Agility => attributes.agility(),
            PrimaryAttribute::Intelligence => attributes.intelligence(),
        };
        let primary_delta = primary_now.saturating_sub(primary_base);
        let primary_delta_f: f32 = cast(primary_delta).unwrap_or(0.0);
        let attack_bonus_f: f32 = primary_delta_f * WARCRAFT_GAMEPLAY_CONSTANTS.str_attack_bonus();
        let primary_delta_attack: u32 =
            cast::<f32, u32>(attack_bonus_f.floor()).unwrap_or(primary_delta);
        let attack_min_base = combat.attack().map(UnitAttack::damage_min).unwrap_or(0);
        let attack_max_base = combat.attack().map(UnitAttack::damage_max).unwrap_or(0);
        Self {
            strength,
            agility,
            intelligence,
            hit_points,
            hit_points_regen,
            mana,
            mana_regen,
            armor,
            damage_min: attack_min_base + primary_delta_attack,
            damage_max: attack_max_base + primary_delta_attack,
        }
    }

    fn strength(&self) -> u32 {
        self.strength
    }

    fn agility(&self) -> u32 {
        self.agility
    }

    fn intelligence(&self) -> u32 {
        self.intelligence
    }

    fn hit_points(&self) -> u32 {
        self.hit_points
    }

    fn hit_points_regen(&self) -> f32 {
        self.hit_points_regen
    }

    fn mana(&self) -> u32 {
        self.mana
    }

    fn mana_regen(&self) -> f32 {
        self.mana_regen
    }

    fn armor(&self) -> f32 {
        self.armor
    }

    fn damage_min(&self) -> u32 {
        self.damage_min
    }

    fn damage_max(&self) -> u32 {
        self.damage_max
    }
}
