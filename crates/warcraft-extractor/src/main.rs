use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use clap::Parser;
use warcraft_api::{
    GridCoordinate, WarcraftDatabase, WarcraftObject, WarcraftObjectId, WarcraftObjectMeta,
};
use warcraft_extractor::*;

const DEFAULT_DATABASE_FILE: &str = concat!(
    env!("CARGO_WORKSPACE_DIR"),
    "crates/warcraft-database/src/db.rs"
);

#[derive(Parser)]
#[command(
    name = "warcraft-extractor",
    version,
    about = concat!(
        "Extract the Warcraft III object database from CASC and emit Rust source.\n\n",
        "Tested against Warcraft III ", env!("WARCRAFT_SUPPORTED_VERSION"), "."
    )
)]
struct Args {
    /// Path to the Warcraft III CASC storage root.
    #[arg(short = 'c', long = "casc", env = "W3_CASC", value_name = "PATH")]
    casc_root: PathBuf,

    /// Write generated Rust to FILE. Defaults to the workspace-relative
    /// `crates/warcraft-database/src/db.rs`. Pass `-` to stream to stdout.
    #[arg(short = 'o', long = "output", value_name = "FILE", default_value = DEFAULT_DATABASE_FILE)]
    output_file: PathBuf,

    /// Skip the post-write `cargo check` verification of `warcraft-database`.
    /// Useful for debugging or when the downstream crate is already known to
    /// be broken.
    #[arg(long)]
    no_verify: bool,
}

impl Args {
    fn casc_root(&self) -> &Path {
        &self.casc_root
    }

    fn output_file(&self) -> &Path {
        &self.output_file
    }

    fn output_is_stdout(&self) -> bool {
        self.output_file.as_os_str() == "-"
    }

    fn no_verify(&self) -> bool {
        self.no_verify
    }
}

fn main() {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    let subscriber_builder = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr);
    subscriber_builder.init();

    let args = Args::parse();

    eprintln!("warcraft-extractor");
    let casc_root_display = args.casc_root().display().to_string();
    eprintln!("  CASC root: {casc_root_display}");
    let output_target = if args.output_is_stdout() {
        "<stdout>".to_string()
    } else {
        args.output_file().display().to_string()
    };
    eprintln!("  Output:    {output_target}");
    eprintln!();

    Preflight::check_or_exit(args.casc_root());

    let casc_root_string = args.casc_root().to_string_lossy().into_owned();
    eprintln!("  extracting CASC data ...");
    let extraction_result = ExtractionPipeline::run(&casc_root_string, None, &EXTRACTION_RULES);
    let results = match extraction_result {
        Ok(value) => value,
        Err(error) => {
            eprintln!("  error: {error}");
            std::process::exit(1);
        }
    };

    let aggregated_data: WarcraftDataAggregation = results.into();
    let generated_source = CodegenContext::render_database(aggregated_data);

    if args.output_is_stdout() {
        print!("{generated_source}");
        eprintln!();
        return;
    }

    eprintln!("  writing output file ...");
    let output_path = args.output_file().to_path_buf();
    let generated_bytes = generated_source.into_bytes();
    let staged_write = match StagedWrite::stage(output_path, &generated_bytes) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("  error: failed to write output: {error}");
            std::process::exit(1);
        }
    };

    if args.no_verify() {
        staged_write.commit();
        eprintln!();
        return;
    }

    eprintln!("  verifying with cargo check ...");
    let check_outcome = run_cargo_check(VERIFIED_PACKAGE);
    match check_outcome {
        CargoCheckOutcome::Success => {
            staged_write.commit();
            eprintln!();
        }
        CargoCheckOutcome::Failure { message } => {
            drop(staged_write);
            eprintln!();
            eprintln!("  error: {message}");
            eprintln!("  previous db.rs restored");
            std::process::exit(1);
        }
    }
}

const VERIFIED_PACKAGE: &str = "warcraft-database";

enum CargoCheckOutcome {
    Success,
    Failure { message: String },
}

fn run_cargo_check(package_name: &str) -> CargoCheckOutcome {
    let mut command = std::process::Command::new("cargo");
    command.args(["check", "-p", package_name]);
    let status_result = command.status();
    match status_result {
        Ok(status) if status.success() => CargoCheckOutcome::Success,
        Ok(status) => {
            let code_string = status
                .code()
                .map(|code| code.to_string())
                .unwrap_or_else(|| "<no exit code>".to_string());
            let message = format!("cargo check -p {package_name} exited with status {code_string}");
            CargoCheckOutcome::Failure { message }
        }
        Err(error) => {
            let message = format!("failed to invoke cargo: {error}");
            CargoCheckOutcome::Failure { message }
        }
    }
}

/// A single staged file overwrite that rolls back on drop unless `commit` was
/// called. Captures the pre-image (or absence) of the target before writing
/// the new bytes, so a failed verification step restores the previous file.
struct StagedWrite {
    target_path: PathBuf,
    pre_image: Option<Vec<u8>>,
    committed: bool,
}

impl StagedWrite {
    fn stage(target_path: PathBuf, new_bytes: &[u8]) -> std::io::Result<Self> {
        let read_result = std::fs::read(&target_path);
        let pre_image = match read_result {
            Ok(bytes) => Some(bytes),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => return Err(error),
        };
        if let Some(parent_dir) = target_path.parent() {
            std::fs::create_dir_all(parent_dir)?;
        }
        std::fs::write(&target_path, new_bytes)?;
        let staged = Self {
            target_path,
            pre_image,
            committed: false,
        };
        Ok(staged)
    }

    fn commit(mut self) {
        self.committed = true;
    }
}

impl Drop for StagedWrite {
    fn drop(&mut self) {
        if self.committed {
            return;
        }
        match &self.pre_image {
            Some(bytes) => {
                let _ = std::fs::write(&self.target_path, bytes);
            }
            None => {
                let _ = std::fs::remove_file(&self.target_path);
            }
        }
    }
}

struct Preflight;

impl Preflight {
    fn check_or_exit(casc_root: &Path) {
        if !casc_root.exists() {
            let casc_root_display = casc_root.display();
            eprintln!("  error: CASC root does not exist: {casc_root_display}");
            eprintln!();
            eprintln!("  To fix:");
            eprintln!("    • set W3_CASC=<path to 'Warcraft III/Data'>");
            eprintln!("    • or pass --casc <PATH>");
            eprintln!();
            eprintln!("  Typical locations:");
            eprintln!("    Windows:  C:\\Program Files (x86)\\Warcraft III\\Data");
            eprintln!("    Wine:     <WINEPREFIX>/drive_c/Program Files (x86)/Warcraft III/Data");
            eprintln!();
            std::process::exit(2);
        }
        if !casc_root.is_dir() {
            let casc_root_display = casc_root.display();
            eprintln!("  error: CASC root is not a directory: {casc_root_display}");
            eprintln!();
            std::process::exit(2);
        }
    }
}

const EXTRACTION_RULES: [ExtractionRule; 42] = [
    HEROES_EXTRACTION_RULE,
    UNITS_EXTRACTION_RULE,
    UNIT_ABILITIES_EXTRACTION_RULE,
    ABILITY_METADATA_EXTRACTION_RULE,
    UPGRADE_SWAPS_EXTRACTION_RULE,
    UNIT_DATA_EXTRACTION_RULE,
    UNIT_UI_FLAGS_EXTRACTION_RULE,
    ABILITY_DEFAULTS_EXTRACTION_RULE,
    COMMAND_DEFAULTS_EXTRACTION_RULE,
    DATA_TABLES_EXTRACTION_RULE,
    DEFAULT_POSITIONS_EXTRACTION_RULE,
    OBJECT_TEXTS_EXTRACTION_RULE,
    SYSTEM_KEYBINDS_EXTRACTION_RULE,
    ITEMS_EXTRACTION_RULE,
    ITEM_SKINS_EXTRACTION_RULE,
    UNIT_SKINS_EXTRACTION_RULE,
    ABILITY_SKINS_EXTRACTION_RULE,
    HUMAN_UPGRADES_ART_EXTRACTION_RULE,
    HUMAN_UPGRADES_NAME_EXTRACTION_RULE,
    HUMAN_ABILITY_STRINGS_EXTRACTION_RULE,
    HUMAN_UNIT_STRINGS_EXTRACTION_RULE,
    NIGHTELF_UPGRADES_ART_EXTRACTION_RULE,
    NIGHTELF_UPGRADES_NAME_EXTRACTION_RULE,
    NIGHTELF_ABILITY_STRINGS_EXTRACTION_RULE,
    NIGHTELF_UNIT_STRINGS_EXTRACTION_RULE,
    ORC_UPGRADES_ART_EXTRACTION_RULE,
    ORC_UPGRADES_NAME_EXTRACTION_RULE,
    ORC_ABILITY_STRINGS_EXTRACTION_RULE,
    ORC_UNIT_STRINGS_EXTRACTION_RULE,
    UNDEAD_UPGRADES_ART_EXTRACTION_RULE,
    UNDEAD_UPGRADES_NAME_EXTRACTION_RULE,
    UNDEAD_ABILITY_STRINGS_EXTRACTION_RULE,
    UNDEAD_UNIT_STRINGS_EXTRACTION_RULE,
    NEUTRAL_ABILITY_STRINGS_EXTRACTION_RULE,
    NEUTRAL_UNIT_STRINGS_EXTRACTION_RULE,
    ITEM_ABILITY_STRINGS_EXTRACTION_RULE,
    ITEM_UNIT_STRINGS_EXTRACTION_RULE,
    CAMPAIGN_UNIT_STRINGS_EXTRACTION_RULE,
    CAMPAIGN_ABILITY_STRINGS_EXTRACTION_RULE,
    COMMON_ABILITY_STRINGS_EXTRACTION_RULE,
    COMMON_UNIT_STRINGS_EXTRACTION_RULE,
    GAMEPLAY_CONSTANTS_EXTRACTION_RULE,
];

struct CodegenContext {
    str_slices: BTreeMap<String, Vec<String>>,
    id_slices: BTreeMap<String, Vec<String>>,
    cooldown_arrays: BTreeMap<String, [u32; 4]>,
}

impl CodegenContext {
    fn new() -> Self {
        Self {
            str_slices: BTreeMap::new(),
            id_slices: BTreeMap::new(),
            cooldown_arrays: BTreeMap::new(),
        }
    }

    fn render_database(aggregated_data: WarcraftDataAggregation) -> String {
        let system_keybinds_source = aggregated_data.system_keybinds().clone();
        let gameplay_constants = *aggregated_data.gameplay_constants();
        // The distinct, deduplicated tiered summon groups (each an ordered list
        // of unit ids, lowest tier first) gathered from every summon ability.
        let tiered_unit_groups: BTreeSet<Vec<String>> = aggregated_data
            .ability_metadata()
            .values()
            .filter(|entry| entry.tiered_summon_units().len() >= 2)
            .map(|entry| entry.tiered_summon_units().to_vec())
            .collect();
        let unit_upgrade_swaps = aggregated_data.upgrade_swaps().clone();
        let database: WarcraftDatabase = aggregated_data.into();

        let mut context = CodegenContext::new();
        let mut output = String::new();

        output.push_str(
            "// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n",
        );
        output.push_str(
            "// !! AUTO-GENERATED. DO NOT EDIT THIS FILE. EDITING IS BANNED.            !!\n",
        );
        output.push_str(
            "// !! THIS FILE IS WIPED AND REGENERATED ON EVERY PATCH IMPORT.            !!\n",
        );
        output.push_str(
            "// !! ANY CHANGE YOU MAKE HERE WILL BE GONE. DO NOT TOUCH THIS FILE.       !!\n",
        );
        output.push_str(
            "// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n",
        );
        output.push_str("use std::sync::LazyLock;\n\n");
        output.push_str("use warcraft_api::*;\n\n");
        output.push_str("use warcraft_api::WarcraftObjectKind::*;\n");
        output.push_str("use warcraft_api::Race::*;\n");
        output.push_str("use warcraft_api::UnitKind::*;\n");
        output.push_str("use warcraft_api::ItemClass::*;\n\n");

        const OBJECTS_PER_CHUNK: usize = 200;
        let database_objects = database.db();
        let total_objects = database_objects.len();
        let chunk_count = total_objects.div_ceil(OBJECTS_PER_CHUNK).max(1);
        let mut chunks_code = String::new();
        for (chunk_index, chunk_window) in database_objects
            .iter()
            .collect::<Vec<_>>()
            .chunks(OBJECTS_PER_CHUNK)
            .enumerate()
        {
            chunks_code.push_str(&format!(
                "fn insert_objects_chunk_{chunk_index}(\
                 objects: &mut std::collections::BTreeMap<WarcraftObjectId, WarcraftObject>\
                 ) {{\n"
            ));
            for (object_id, warcraft_object) in chunk_window {
                context.emit_object(&mut chunks_code, object_id, warcraft_object);
            }
            chunks_code.push_str("}\n\n");
        }

        let mut objects_code = String::new();
        objects_code.push_str(
            "pub static WARCRAFT_DATABASE: LazyLock<WarcraftDatabase> = LazyLock::new(|| {\n",
        );
        objects_code.push_str("    let mut objects = std::collections::BTreeMap::new();\n");
        for chunk_index in 0..chunk_count {
            objects_code.push_str(&format!(
                "    insert_objects_chunk_{chunk_index}(&mut objects);\n"
            ));
        }
        objects_code.push_str("    WarcraftDatabase::new(objects)\n");
        objects_code.push_str("});\n");

        output.push_str(&objects_code);
        output.push('\n');
        output.push_str(&chunks_code);
        output.push('\n');
        emit_gameplay_constants(&mut output, &gameplay_constants);
        output.push('\n');
        emit_system_keybinds(&mut output, &system_keybinds_source);
        output.push('\n');
        emit_tiered_unit_groups(&mut output, &tiered_unit_groups);
        output.push('\n');
        emit_unit_upgrade_swaps(&mut output, &unit_upgrade_swaps);
        output.push('\n');
        context.emit_consts(&mut output);

        output
    }

    fn intern_str_slice_named(&mut self, name: String, values: &[&'static str]) -> String {
        self.str_slices.entry(name.clone()).or_insert_with(|| {
            values
                .iter()
                .map(|string_value| string_value.to_string())
                .collect()
        });
        name
    }

    fn intern_id_slice_named(&mut self, name: String, values: &[WarcraftObjectId]) -> String {
        self.id_slices.entry(name.clone()).or_insert_with(|| {
            values
                .iter()
                .map(|object_id| {
                    let object_id_value = object_id.value();
                    object_id_value.to_string()
                })
                .collect()
        });
        name
    }

    fn intern_cooldowns_named(&mut self, name: String, values: [u32; 4]) -> String {
        self.cooldown_arrays.entry(name.clone()).or_insert(values);
        name
    }

    fn emit_consts(&self, output: &mut String) {
        for (name, values) in &self.str_slices {
            output.push_str(&format!("const {name}: &[&str] = &[\n"));
            for value in values {
                let escaped = escape_rust_string_literal(value);
                output.push_str(&format!("    \"{escaped}\",\n"));
            }
            output.push_str("];\n");
        }

        for (name, values) in &self.id_slices {
            output.push_str(&format!("const {name}: &[WarcraftObjectId] = &[\n"));
            for value in values {
                output.push_str(&format!("    WarcraftObjectId::new(\"{value}\"),\n"));
            }
            output.push_str("];\n");
        }

        for (name, values) in &self.cooldown_arrays {
            output.push_str(&format!(
                "const {name}: [u32; 4] = [{}, {}, {}, {}];\n",
                values[0], values[1], values[2], values[3]
            ));
        }
    }

    fn emit_object(
        &mut self,
        output: &mut String,
        object_id: &WarcraftObjectId,
        warcraft_object: &WarcraftObject,
    ) {
        let id_value = object_id.value();
        let id_normalized = Self::normalize_identifier(id_value);

        let object_names = warcraft_object.names();
        let name_const =
            self.intern_str_slice_named(format!("{id_normalized}_NAMES"), object_names);

        let object_icons = warcraft_object.icons();
        let icon_const =
            self.intern_str_slice_named(format!("{id_normalized}_ICONS"), object_icons);

        let tip_levels = warcraft_object.tip_levels();
        let ubertip_levels = warcraft_object.ubertip_levels();
        let un_tip_text = warcraft_object.un_tip();
        let un_ubertip_text = warcraft_object.un_ubertip();
        let has_alt_text = un_tip_text.is_some() || un_ubertip_text.is_some();
        let has_inline_text = has_alt_text || !tip_levels.is_empty() || !ubertip_levels.is_empty();
        let constructor_name = if has_inline_text { "with_text" } else { "new" };

        output.push_str("    objects.insert(\n");
        output.push_str(&format!("        WarcraftObjectId::new(\"{id_value}\"),\n"));
        output.push_str(&format!("        WarcraftObject::{constructor_name}(\n"));

        let object_kind = warcraft_object.kind();
        let object_race = warcraft_object.race();
        let race_code = match object_race {
            Some(race) => format!("Some({race:?})"),
            None => "None".into(),
        };
        output.push_str(&format!(
            "            WarcraftObjectId::new(\"{id_value}\"),\n"
        ));
        output.push_str(&format!("            {name_const},\n"));
        output.push_str(&format!("            {icon_const},\n"));
        output.push_str(&format!("            {object_kind:?},\n"));
        output.push_str(&format!("            {race_code},\n"));

        let object_meta = warcraft_object.meta();
        self.emit_object_metadata(output, object_id, object_meta);

        if has_inline_text {
            let tip_const =
                self.intern_str_slice_named(format!("{id_normalized}_TIP_LEVELS"), tip_levels);
            let ubertip_const = self
                .intern_str_slice_named(format!("{id_normalized}_UBERTIP_LEVELS"), ubertip_levels);
            if has_alt_text {
                let un_tip_literal = format_static_str_option(un_tip_text);
                let un_ubertip_literal = format_static_str_option(un_ubertip_text);
                output.push_str(&format!(
                    "            WarcraftObjectText::with_alt({tip_const}, {ubertip_const}, \
                     {un_tip_literal}, {un_ubertip_literal}),\n"
                ));
            } else {
                output.push_str(&format!(
                    "            WarcraftObjectText::new({tip_const}, {ubertip_const}),\n"
                ));
            }
        }

        let default_button = warcraft_object.default_button_position();
        let default_research = warcraft_object.default_research_button_position();
        let object_meta_has_position = matches!(
            warcraft_object.meta(),
            warcraft_api::WarcraftObjectMeta::Ability(_)
                | warcraft_api::WarcraftObjectMeta::Command(_)
        );
        let emit_button = default_button.is_some() && !object_meta_has_position;
        let emit_research = default_research.is_some() && !object_meta_has_position;
        let has_chain_calls = emit_button || emit_research;

        if has_chain_calls {
            output.push_str("        )\n");
            if let Some(position) = default_button
                && emit_button
            {
                let column_variant = column_index_variant(position.column());
                let row_variant = row_index_variant(position.row());
                output.push_str(&format!(
                    "        .with_default_position(Some(GridCoordinate::new(ColumnIndex::{column_variant}, RowIndex::{row_variant})))\n"
                ));
            }
            if let Some(position) = default_research
                && emit_research
            {
                let column_variant = column_index_variant(position.column());
                let row_variant = row_index_variant(position.row());
                output.push_str(&format!(
                    "        .with_default_research_position(Some(GridCoordinate::new(ColumnIndex::{column_variant}, RowIndex::{row_variant})))\n"
                ));
            }
            output.push_str("        ,\n");
        } else {
            output.push_str("        ),\n");
        }
        output.push_str("    );\n\n");
    }

    fn emit_object_metadata(
        &mut self,
        output: &mut String,
        object_id: &WarcraftObjectId,
        meta: &WarcraftObjectMeta,
    ) {
        let id_value = object_id.value();
        let id_normalized = Self::normalize_identifier(id_value);

        match meta {
            WarcraftObjectMeta::Unit(unit_meta) => {
                let unit_kind = unit_meta.unit_kind();
                let build_time = unit_meta.build_time();
                let level = unit_meta.level();
                let gold_cost = unit_meta.gold_cost();
                let unit_abilities = unit_meta.abilities();
                let abilities_const = self.intern_id_slice_named(
                    format!("{id_normalized}_UNIT_ABILITIES"),
                    unit_abilities,
                );
                let hero_abilities = unit_meta.hero_abilities();
                let hero_abilities_const = self.intern_id_slice_named(
                    format!("{id_normalized}_UNIT_HERO_ABILITIES"),
                    hero_abilities,
                );
                let unit_researches = unit_meta.researches();
                let researches_const = self.intern_id_slice_named(
                    format!("{id_normalized}_UNIT_RESEARCHES"),
                    unit_researches,
                );
                let unit_builds = unit_meta.builds();
                let builds_const =
                    self.intern_id_slice_named(format!("{id_normalized}_UNIT_BUILDS"), unit_builds);
                let unit_trains = unit_meta.trains();
                let trains_const =
                    self.intern_id_slice_named(format!("{id_normalized}_UNIT_TRAINS"), unit_trains);
                let unit_sell_items = unit_meta.sell_items();
                let sell_items_const = self.intern_id_slice_named(
                    format!("{id_normalized}_UNIT_SELL_ITEMS"),
                    unit_sell_items,
                );
                let unit_sell_units = unit_meta.sell_units();
                let sell_units_const = self.intern_id_slice_named(
                    format!("{id_normalized}_UNIT_SELL_UNITS"),
                    unit_sell_units,
                );
                let is_campaign = unit_meta.is_campaign();
                let is_in_editor = unit_meta.is_in_editor();
                let is_hidden_in_editor = unit_meta.is_hidden_in_editor();
                let is_special = unit_meta.is_special();

                let combat = unit_meta.combat();
                let hit_points = combat.hit_points();
                let hit_points_regen = combat.hit_points_regen();
                let regen_type = combat.regen_type();
                let armor = combat.armor();
                let defense_type = combat.defense_type();
                let attack_expression = match combat.attack() {
                    Some(unit_attack) => {
                        let damage_min = unit_attack.damage_min();
                        let damage_max = unit_attack.damage_max();
                        let attack_range = unit_attack.range();
                        let cooldown_seconds = unit_attack.cooldown_seconds();
                        let attack_type = unit_attack.attack_type();
                        let weapon_type = unit_attack.weapon_type();
                        format!(
                            "Some(UnitAttack::new({damage_min}, {damage_max}, {attack_range}, \
                             {cooldown_seconds:?}, AttackType::{attack_type:?}, \
                             WeaponType::{weapon_type:?}))"
                        )
                    }
                    None => String::from("None"),
                };
                let mana_pool_chain = match combat.mana_pool() {
                    Some(mana_pool) => {
                        let mana = mana_pool.mana();
                        let mana_regen = mana_pool.mana_regen();
                        format!(".with_mana_pool(ManaPool::new({mana}, {mana_regen:?}))")
                    }
                    None => String::new(),
                };
                let combat_expression = format!(
                    "UnitCombat::new({hit_points}, {hit_points_regen:?}, RegenType::{regen_type:?}, \
                     {armor:?}, DefenseType::{defense_type:?}, {attack_expression}){mana_pool_chain}"
                );
                let hero_attributes_chain = match unit_meta.hero_attributes() {
                    Some(hero_attributes) => {
                        let mana = hero_attributes.mana();
                        let mana_regen = hero_attributes.mana_regen();
                        let strength = hero_attributes.strength();
                        let agility = hero_attributes.agility();
                        let intelligence = hero_attributes.intelligence();
                        let primary = hero_attributes.primary();
                        let strength_per_level = hero_attributes.strength_per_level();
                        let agility_per_level = hero_attributes.agility_per_level();
                        let intelligence_per_level = hero_attributes.intelligence_per_level();
                        format!(
                            ".with_hero_attributes(HeroAttributes::new(\
                             ManaPool::new({mana}, {mana_regen:?}), \
                             AttributeBase::new({strength}, {agility}, {intelligence}), \
                             AttributeGrowth::new({strength_per_level:?}, {agility_per_level:?}, {intelligence_per_level:?}), \
                             PrimaryAttribute::{primary:?}))"
                        )
                    }
                    None => String::new(),
                };

                output.push_str(&format!(
                    "            WarcraftObjectMeta::Unit(UnitMeta::with_full_and_extras({unit_kind:?}, \
                     {build_time}, {abilities_const}, {hero_abilities_const}, \
                     UnitProduction::new({researches_const}, {builds_const}, {trains_const}, \
                     {sell_items_const}, {sell_units_const}), \
                     UnitFlags::new({is_campaign}, {is_in_editor}, {is_hidden_in_editor}, {is_special}))\
                     .with_combat({combat_expression}).with_level({level}).with_gold_cost({gold_cost}){hero_attributes_chain}),\n"
                ));
            }

            WarcraftObjectMeta::Ability(ability_meta) => {
                let max_level = ability_meta.max_level();
                let is_ultimate = ability_meta.is_ultimate();
                let cooldowns = ability_meta.cooldowns();
                let cooldown_const =
                    self.intern_cooldowns_named(format!("{id_normalized}_COOLDOWNS"), cooldowns);
                let default_button =
                    format_button_position_option(ability_meta.default_button_position());
                let default_research_button =
                    format_button_position_option(ability_meta.default_research_button_position());
                let ubertip_literal = format_static_str_option(ability_meta.ubertip());
                let research_ubertip_literal =
                    format_static_str_option(ability_meta.research_ubertip());
                let code_literal = format_static_str_option(ability_meta.code());
                let morph_target_literal = ability_meta
                    .morph_target_unit()
                    .map(|target| format!("Some(WarcraftObjectId::new(\"{}\"))", target.value()))
                    .unwrap_or_else(|| "None".to_string());
                let off_button_literal =
                    format_button_position_option(ability_meta.off_button_position());
                let off_tip_literal = format_static_str_option(ability_meta.off_tip());
                let off_ubertip_literal = format_static_str_option(ability_meta.off_ubertip());
                let off_icon_literal = format_static_str_option(ability_meta.off_icon());
                let evasion_chances = ability_meta.evasion_chances();
                let has_evasion = evasion_chances.iter().any(|chance| *chance > 0.0);
                let evasion_call = if has_evasion {
                    // `{:?}` keeps the decimal point so the slot stays an f32
                    // literal (`0.0`, not `0`) — and round-trips the value.
                    format!(
                        ".with_evasion_chances([{:?}, {:?}, {:?}, {:?}])",
                        evasion_chances[0],
                        evasion_chances[1],
                        evasion_chances[2],
                        evasion_chances[3]
                    )
                } else {
                    String::new()
                };

                output.push_str(&format!(
                    "            WarcraftObjectMeta::Ability(AbilityMeta::with_ubertips({max_level}, {is_ultimate}, \
                     {cooldown_const}, {default_button}, {default_research_button}, \
                     {ubertip_literal}, {research_ubertip_literal}).with_code({code_literal})\
                     .with_morph_target({morph_target_literal})\
                     {evasion_call}\
                     .with_off_state({off_button_literal}, {off_tip_literal}, {off_ubertip_literal}, {off_icon_literal})),\n"
                ));
            }

            WarcraftObjectMeta::Upgrade(upgrade_meta) => {
                let max_level = upgrade_meta.max_level();
                output.push_str(&format!(
                    "            WarcraftObjectMeta::Upgrade(UpgradeMeta::new({max_level})),\n",
                ));
            }

            WarcraftObjectMeta::Command(command_meta) => {
                let default_position =
                    format_button_position_option(command_meta.default_button_position());
                let tip_literal = format_static_str_option(command_meta.tip());
                let ubertip_literal = format_static_str_option(command_meta.ubertip());
                output.push_str(&format!(
                    "            WarcraftObjectMeta::Command(CommandMeta::with_text({default_position}, \
                     {tip_literal}, {ubertip_literal})),\n"
                ));
            }

            WarcraftObjectMeta::Item(item_meta) => {
                let item_abilities = item_meta.abilities();
                let ability_const = self
                    .intern_id_slice_named(format!("{id_normalized}_ABILITIES"), item_abilities);
                let item_class = item_meta.class();
                let item_cooldown_id = item_meta.cooldown_id();
                let cooldown_code = item_cooldown_id
                    .map(|cooldown_id| {
                        let cooldown_value = cooldown_id.value();
                        format!("Some(WarcraftObjectId::new(\"{cooldown_value}\"))")
                    })
                    .unwrap_or("None".into());

                output.push_str("            WarcraftObjectMeta::Item(ItemMeta::new(\n");
                output.push_str(&format!("                {item_class:?},\n"));
                output.push_str(&format!("                {ability_const},\n"));
                output.push_str(&format!("                {cooldown_code},\n"));
                output.push_str("            )),\n");
            }
        }
    }

    fn normalize_identifier(identifier: &str) -> String {
        identifier
            .chars()
            .map(|character| {
                if character.is_ascii_alphanumeric() {
                    character.to_ascii_uppercase()
                } else {
                    '_'
                }
            })
            .collect()
    }
}

fn format_button_position_option(position: Option<GridCoordinate>) -> String {
    match position {
        Some(value) => {
            let column_variant = column_index_variant(value.column());
            let row_variant = row_index_variant(value.row());
            format!(
                "Some(GridCoordinate::new(ColumnIndex::{column_variant}, RowIndex::{row_variant}))"
            )
        }
        None => "None".to_string(),
    }
}

fn column_index_variant(index: warcraft_api::ColumnIndex) -> &'static str {
    match index {
        warcraft_api::ColumnIndex::Zero => "Zero",
        warcraft_api::ColumnIndex::One => "One",
        warcraft_api::ColumnIndex::Two => "Two",
        warcraft_api::ColumnIndex::Three => "Three",
    }
}

fn row_index_variant(index: warcraft_api::RowIndex) -> &'static str {
    match index {
        warcraft_api::RowIndex::Zero => "Zero",
        warcraft_api::RowIndex::One => "One",
        warcraft_api::RowIndex::Two => "Two",
    }
}

/// Emits the `TIERED_UNIT_GROUPS` table: each entry is one summoned unit's tiers
/// (lowest first), so consumers can collapse the variants into a single editor
/// entry and fan ability edits across the tiers.
fn emit_tiered_unit_groups(output: &mut String, groups: &BTreeSet<Vec<String>>) {
    output.push_str("pub const TIERED_UNIT_GROUPS: &[&[WarcraftObjectId]] = &[\n");
    for group in groups {
        output.push_str("    &[");
        for (index, unit_id) in group.iter().enumerate() {
            if index > 0 {
                output.push_str(", ");
            }
            output.push_str(&format!("WarcraftObjectId::new({unit_id:?})"));
        }
        output.push_str("],\n");
    }
    output.push_str("];\n");
}

/// Emits the `UNIT_UPGRADE_SWAPS` table: each entry is an upgrade that replaces
/// one trained unit with another (`from` → `to`), parsed from the `rtma`
/// effect rows of `units/upgradedata.slk`. Ordered by from-id for stable regen.
fn emit_unit_upgrade_swaps(output: &mut String, swaps: &BTreeSet<UnitUpgradeSwapEntry>) {
    output.push_str("pub const UNIT_UPGRADE_SWAPS: &[UnitUpgradeSwap] = &[\n");
    for swap in swaps {
        let from_unit_id = swap.from_unit_id();
        let to_unit_id = swap.to_unit_id();
        output.push_str(&format!(
            "    UnitUpgradeSwap::new({from_unit_id:?}, {to_unit_id:?}),\n"
        ));
    }
    output.push_str("];\n");
}

fn emit_gameplay_constants(output: &mut String, constants: &warcraft_api::GameplayConstants) {
    let str_attack = constants.str_attack_bonus();
    let str_hp = constants.str_hit_point_bonus();
    let str_regen = constants.str_regen_bonus();
    let int_mana = constants.int_mana_bonus();
    let int_regen = constants.int_regen_bonus();
    let agi_def = constants.agi_defense_bonus();
    let agi_atk_speed = constants.agi_attack_speed_bonus();
    let max_level = constants.max_hero_level();
    let damage_normal = format_damage_effectiveness(
        constants.damage_effectiveness(warcraft_api::AttackType::Normal),
    );
    let damage_pierce = format_damage_effectiveness(
        constants.damage_effectiveness(warcraft_api::AttackType::Pierce),
    );
    let damage_siege = format_damage_effectiveness(
        constants.damage_effectiveness(warcraft_api::AttackType::Siege),
    );
    let damage_magic = format_damage_effectiveness(
        constants.damage_effectiveness(warcraft_api::AttackType::Magic),
    );
    let damage_chaos = format_damage_effectiveness(
        constants.damage_effectiveness(warcraft_api::AttackType::Chaos),
    );
    let damage_spells = format_damage_effectiveness(
        constants.damage_effectiveness(warcraft_api::AttackType::Spells),
    );
    let damage_hero =
        format_damage_effectiveness(constants.damage_effectiveness(warcraft_api::AttackType::Hero));
    output.push_str(&format!(
        "pub static WARCRAFT_GAMEPLAY_CONSTANTS: GameplayConstants = GameplayConstants::new(\
         StrengthBonuses::new({str_attack:?}, {str_hp}, {str_regen:?}), \
         IntelligenceBonuses::new({int_mana}, {int_regen:?}), \
         AgilityBonuses::new({agi_def:?}, {agi_atk_speed:?}), \
         {max_level}, \
         DamageMatrix::new({damage_normal}, {damage_pierce}, {damage_siege}, \
         {damage_magic}, {damage_chaos}, {damage_spells}, {damage_hero}));\n"
    ));
}

fn format_damage_effectiveness(effectiveness: warcraft_api::DamageEffectiveness) -> String {
    let multipliers = effectiveness.multipliers();
    let m0 = multipliers[0];
    let m1 = multipliers[1];
    let m2 = multipliers[2];
    let m3 = multipliers[3];
    let m4 = multipliers[4];
    let m5 = multipliers[5];
    let m6 = multipliers[6];
    let m7 = multipliers[7];
    format!(
        "DamageEffectiveness::new([{m0:?}, {m1:?}, {m2:?}, {m3:?}, {m4:?}, {m5:?}, {m6:?}, {m7:?}])"
    )
}

fn emit_system_keybinds(output: &mut String, entries: &[warcraft_extractor::SystemKeybindEntry]) {
    output.push_str("pub static WARCRAFT_SYSTEM_KEYBINDS: &[SystemKeybind] = &[\n");
    for entry in entries {
        let comment_escaped = format_static_str_option(Some(entry.comment()));
        let comment_literal = comment_escaped
            .trim_start_matches("Some(")
            .trim_end_matches(')');
        let modifier_variant = match entry.default_modifier() {
            warcraft_api::SystemKeybindModifier::None => "None",
            warcraft_api::SystemKeybindModifier::Alt => "Alt",
            warcraft_api::SystemKeybindModifier::Ctrl => "Ctrl",
            warcraft_api::SystemKeybindModifier::CtrlOrAlt => "CtrlOrAlt",
            warcraft_api::SystemKeybindModifier::Shift => "Shift",
        };
        let class_variant = match entry.class() {
            warcraft_api::SystemKeybindClass::Menu => "Menu",
            warcraft_api::SystemKeybindClass::ControlGroup => "ControlGroup",
            warcraft_api::SystemKeybindClass::Game => "Game",
            warcraft_api::SystemKeybindClass::Camera => "Camera",
            warcraft_api::SystemKeybindClass::Observer => "Observer",
            warcraft_api::SystemKeybindClass::Replay => "Replay",
        };
        let section_id = entry.section_id();
        let default_hotkey = entry.default_hotkey();
        output.push_str(&format!(
            "    SystemKeybind::new(\"{section_id}\", {comment_literal}, {default_hotkey}, \
             SystemKeybindModifier::{modifier_variant}, SystemKeybindClass::{class_variant}),\n"
        ));
    }
    output.push_str("];\n");
}

fn format_static_str_option(value: Option<&str>) -> String {
    let Some(text) = value else {
        return String::from("None");
    };
    let escaped = escape_rust_string_literal(text);
    format!("Some(\"{escaped}\")")
}

fn escape_rust_string_literal(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len() + 4);
    for character in text.chars() {
        match character {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            other => escaped.push(other),
        }
    }
    escaped
}
