# Extraction pipeline

`crates/warcraft-database/src/db.rs` is the machine output of
`warcraft-extractor`. It is **not** hand-written and must not be
hand-edited — every regeneration overwrites it in full. If a value in
`db.rs` is wrong, the bug lives in an extraction rule, not in the
generated file.

This document describes:

- the mental model and pipeline shape,
- every extraction rule and the CASC file it reads,
- the on-disk formats the rules parse (SLK, `.txt`, FDF tables, etc.),
- the `db.rs` codegen layout,
- the verification + rollback flow,
- the diagnostic loop for a wrong value,
- how to add a new rule,
- the four inspector examples and when to use each,
- common failure modes,
- explicit non-goals.

---

## Mental model

```
                  CASC storage  (Warcraft III: Reforged)
                            │
           ┌────────────────┼────────────────┐
           ▼                ▼                ▼
       SLK tables      .txt sections     localized strings
   (units/*.slk)    (units/*func.txt)   (enus.w3mod:units/*strings.txt)
           │                │                │
           └─────────┬──────┴────────┬───────┘
                     ▼               ▼
              one ExtractionRule per source file
                     │
                     ▼     (41 rules total)
              ExtractResult variants
                     │
                     ▼
            WarcraftDataAggregation
   (heroes, units, abilities, items, upgrades, skins,
    ability/unit/upgrade strings, default positions,
    system keybinds, gameplay constants, …)
                     │
                     ▼
          impl From<…> for WarcraftDatabase
       (composes a single BTreeMap<WarcraftObjectId, WarcraftObject>
        cascading text/icon/position/race resolution rules)
                     │
                     ▼
              CodegenContext::render_database
       (interns &[&str] / &[WarcraftObjectId] / [u32; 4] slices,
        chunks the BTreeMap into groups of 200,
        emits one fn insert_objects_chunk_N per chunk,
        appends gameplay constants + system keybinds + interned consts)
                     │
                     ▼
          crates/warcraft-database/src/db.rs
                     │
                     ▼
              cargo check -p warcraft-database
                     │
                ┌────┴────┐
              pass       fail
                │         │
            commit     restore previous db.rs, exit 1
```

The pipeline never re-reads CASC after the initial pass. It never
talks to Battle.net, never reads process memory, never decrypts
anything. It only translates the patch-shipped data files in CASC into
Rust source the editor can link against.

---

## Quick start

```sh
nix develop                               # gets cmake, zlib, pinned CascLib

# W3_CASC is auto-detected from common Wine paths in shellHook.
# Override if needed:
export W3_CASC="$HOME/Games/W3Champions/drive_c/Program Files (x86)/Warcraft III/Data"

# Regenerate db.rs in place. Verifies via `cargo check -p warcraft-database`
# and rolls back to the previous file on failure.
cargo run -p warcraft-extractor

# One-shot, no devshell entry. Same result.
nix run .#extract -- --casc "$W3_CASC"

# Stream the regenerated source to stdout (no file write):
cargo run -p warcraft-extractor -- --output -

# Skip verification — debug only. Will overwrite even if cargo check fails.
cargo run -p warcraft-extractor -- --no-verify

# Run the 35 synthetic-input rule tests (no game files needed):
cargo test -p warcraft-extractor
```

Always re-run `cargo fmt -p warcraft-database` after a regeneration —
the codegen emits unwrapped lines on purpose (so generated diffs are
easy to grep and the emitter stays simple); rustfmt then wraps them.
Without that step you'll see ~70k lines of pure formatting churn.

---

## CASC namespace primer

Warcraft III: Reforged stores data in CASC, the same storage format
Blizzard uses for World of Warcraft. Paths inside CASC look like
`war3.w3mod:units/abilitydata.slk`. The colon-prefixed segment is a
**namespace**:

| Namespace             | Contains                                                                  |
| --------------------- | ------------------------------------------------------------------------- |
| `war3.w3mod:`         | gameplay data (SLK tables, `.txt` ability/unit/upgrade definitions)       |
| `enus.w3mod:`         | English (US) localized strings — names, tooltips, ubertips                |
| `dede.w3mod:`         | German strings (and so on per locale)                                     |
| `war3sd.w3mod:`       | Classic-graphics asset overlay                                            |
| `_locale_.w3mod:`     | bound to whatever locale CASC was opened with                             |

The extractor reads `war3.w3mod:` for structure and `enus.w3mod:` for
strings. All matchers do `path.starts_with("war3.w3mod:units")` or
`path.contains("enus.w3mod:units")`. That keeps locale strings stable
even when the user's CASC is set to a non-English locale (`casclib`
opens with the locale of the install, but each entry is namespaced).

`casclib` exposes Windows-style backslashes; the pipeline normalizes
to forward slashes via `normalize()` before any matcher runs, so rule
matchers always see `units/foo.slk`, never `units\foo.slk`.

---

## The pipeline end-to-end

`ExtractionPipeline::run(casc_root, output_dir, rules)` in `lib.rs`:

1. **Open CASC** via `casclib::open(casc_root)`. Errors here
   (`CASC root does not exist`, "open failed") usually mean a wrong
   `W3_CASC` or a corrupt install.

2. **Iterate every CASC entry.** For each entry, normalize the path,
   then ask each rule's `matcher: fn(&str) -> bool` whether it cares.
   An entry can match multiple rules (e.g. `unitabilities.slk` is
   read by both `UNIT_ABILITIES_EXTRACTION_RULE` and the abilities
   metadata rule).

3. **Skip non-matching entries** — most CASC files are not relevant.
   The current pipeline matches ~57 entries out of tens of thousands.

4. **Extract bytes.** Each matched entry has its bytes pulled out via
   `casclib::CascEntry::open(...).extract(&mut bytes)`. CASC error
   `code = 1007` (entry exists but has no readable data, common for
   stub files) is silently swallowed; everything else is fatal.

5. **Dispatch by `ExtractTarget`:**
   - `Text` — pass bytes to `processor: fn(&str, &[u8])` and append
     the resulting `ExtractResult` to the in-memory results vector.
     This is what the 41 production rules use.
   - `Raw` — write raw bytes to `output_dir`. Used by `dump_casc`
     example only; not part of the production pipeline.
   - `Image` / `ImageFallback` — decode DDS via `DdsDecoder`, save as
     PNG. Used by separate asset-extraction tooling, not by the
     production pipeline.

6. **Aggregate.** `Vec<ExtractResult>` is converted into
   `WarcraftDataAggregation`, a struct that fans out each variant
   into the right typed sub-database (`HeroDatabase`,
   `UnitDatabase`, `AbilityMetadataDatabase`, etc.).

7. **Compose.** `impl From<WarcraftDataAggregation> for
   WarcraftDatabase` (in `extractor/src/db.rs`) walks the
   sub-databases and produces the final
   `BTreeMap<WarcraftObjectId, WarcraftObject>`. This is where
   text-substitution, icon-fallback (skin overlays), and per-race
   resolution happen. Read this file when a value comes out wrong
   despite the source SLK looking right.

8. **Codegen.** `CodegenContext::render_database` (in `main.rs`)
   walks the database and emits Rust. Three-stage:
   - splits objects into chunks of 200 entries to keep functions
     under rustc's frame-size limit;
   - emits one `fn insert_objects_chunk_N(objects)` per chunk;
   - emits a `LazyLock<WarcraftDatabase>` whose body calls every
     chunk function;
   - then appends `WARCRAFT_GAMEPLAY_CONSTANTS`, the
     `WARCRAFT_SYSTEM_KEYBINDS` slice, and finally every interned
     `&[&str]` / `&[WarcraftObjectId]` / `[u32; 4]` constant.

9. **Stage write.** `StagedWrite::stage(target_path, bytes)` reads the
   pre-image, writes the new bytes, and remembers what to roll back to.

10. **Verify.** `cargo check -p warcraft-database`. On success, commit
    the staged write. On non-zero exit, drop the staged write — its
    `Drop` impl restores the pre-image (or deletes the file if it
    didn't exist before). This is the load-bearing safety net: a broken
    extraction never overwrites a working `db.rs`.

---

## The 41 extraction rules

Every rule lives in `crates/warcraft-extractor/src/`. All matchers are
trivially regex-free (substring + suffix checks on the normalized
path). Rules are listed in the order they appear in `EXTRACTION_RULES`
in `main.rs`, which is also (roughly) the order downstream
aggregation depends on.

### Structure rules (read SLK tables in `war3.w3mod:units/*.slk`)

| Rule                              | Source CASC path              | Format | Provides                                                                                  |
| --------------------------------- | ----------------------------- | ------ | ----------------------------------------------------------------------------------------- |
| `HEROES_EXTRACTION_RULE`          | `units/abilitydata.slk`       | SLK    | `HeroDatabase` — per-hero ability lists, max-level, ultimate flag, cooldowns              |
| `UNITS_EXTRACTION_RULE`           | `units/unitbalance.slk`       | SLK    | `UnitDatabase` — units bucketed by `Race × UnitKind` (combat, hero attrs, build time)     |
| `UNIT_ABILITIES_EXTRACTION_RULE`  | `units/unitabilities.slk`     | SLK    | `UnitAbilitiesDatabase` — ability id list per unit (incl. hero abilities)                 |
| `ABILITY_METADATA_EXTRACTION_RULE`| `units/abilitydata.slk`       | SLK    | `AbilityMetadataDatabase` — max level, ultimate flag, cooldowns, ability `code` field     |
| `UNIT_DATA_EXTRACTION_RULE`       | `units/unitdata.slk`          | SLK    | `UnitDataDatabase` — campaign/special flags, race override, primary attribute             |
| `UNIT_UI_FLAGS_EXTRACTION_RULE`   | `units/unitui.slk`            | SLK    | `UnitUiFlagsDatabase` — `inEditorVersion`, `hiddenInEditor`                               |
| `ITEMS_EXTRACTION_RULE`           | `units/itemdata.slk`          | SLK    | `ItemDatabase` — items bucketed by `ItemClass`                                            |

### `.txt`-section rules (read `units/*func.txt` and `units/*ability*func.txt`)

The `.txt` files use the `[id] / Art=… / Tip=… / Ubertip=…` section
format described in [`Section format`](#section-format) below. The
`SectionedListParser` and a small per-rule state machine drive these.

| Rule                              | Source CASC path                          | Provides                                                                              |
| --------------------------------- | ----------------------------------------- | ------------------------------------------------------------------------------------- |
| `ABILITY_DEFAULTS_EXTRACTION_RULE`| `units/*abilityfunc.txt` (any race/loc)   | `AbilityDefaultsDatabase` — default button position, off-state, code, morph target    |
| `COMMAND_DEFAULTS_EXTRACTION_RULE`| `units/commandfunc.txt`                   | `CommandDefaultsDatabase` — default positions for non-ability commands (move, stop…)  |
| `DATA_TABLES_EXTRACTION_RULE`     | (SLK column metadata)                     | `DataTablesDatabase` — schema lookups used during aggregation                         |
| `DEFAULT_POSITIONS_EXTRACTION_RULE`| `units/*func.txt` `Buttonpos=` / `Researchbuttonpos=` lines | `DefaultPositionsDatabase` — fallback positions if a unit doesn't override |
| `OBJECT_TEXTS_EXTRACTION_RULE`    | `units/*func.txt` Tip/Ubertip lines       | `ObjectTextDatabase` — raw tooltip text before placeholder substitution               |
| `SYSTEM_KEYBINDS_EXTRACTION_RULE` | `customkeys.txt` (the template shipped with the game) | `SystemKeybindsDatabase` — every menu/control-group/camera hotkey         |
| `GAMEPLAY_CONSTANTS_EXTRACTION_RULE`| `units/miscgame.txt`                    | `GameplayConstants` — strength bonuses, mana/agi tables, damage matrix, max hero level |

### Skin overlay rules (read `units/*skin.txt`)

Skin files override the icon/art for the Reforged graphics setting. The
extractor merges `units/abilityskin.txt` etc. with the base
ability/unit/item defs so the editor renders the right icon regardless
of graphics mode.

| Rule                          | Source CASC path           | Provides                                                |
| ----------------------------- | -------------------------- | ------------------------------------------------------- |
| `ABILITY_SKINS_EXTRACTION_RULE` | `units/abilityskin.txt`  | `AbilitySkins` — Reforged-vs-Classic icon-path overlay  |
| `ITEM_SKINS_EXTRACTION_RULE`  | `units/itemfunc.txt`       | `ItemSkins`                                             |
| `UNIT_SKINS_EXTRACTION_RULE`  | `units/unitskin.txt`       | `UnitSkins`                                             |

The unit skin processor falls back to `Art_SD` when `Art` is absent —
that fallback was a real bug fix and has a regression test in
`tests/rules.rs::skins::unit_skin_processor_falls_back_to_art_sd_when_art_absent`.

### Per-race upgrade rules (read `units/<race>upgrade*.txt`)

Each playable race has two `.txt` files: `art` (icons + buttonpos) and
`name` (display name lookup, only in `enus.w3mod:`). One pair per race.

| Rule                                  | Source CASC path                                  |
| ------------------------------------- | ------------------------------------------------- |
| `HUMAN_UPGRADES_ART_EXTRACTION_RULE`  | `war3.w3mod:units/humanupgradefunc.txt`           |
| `HUMAN_UPGRADES_NAME_EXTRACTION_RULE` | `enus.w3mod:units/humanupgradestrings.txt`        |
| `NIGHTELF_UPGRADES_ART_EXTRACTION_RULE`  | `war3.w3mod:units/nightelfupgradefunc.txt`     |
| `NIGHTELF_UPGRADES_NAME_EXTRACTION_RULE` | `enus.w3mod:units/nightelfupgradestrings.txt`  |
| `ORC_UPGRADES_ART_EXTRACTION_RULE`    | `war3.w3mod:units/orcupgradefunc.txt`             |
| `ORC_UPGRADES_NAME_EXTRACTION_RULE`   | `enus.w3mod:units/orcupgradestrings.txt`          |
| `UNDEAD_UPGRADES_ART_EXTRACTION_RULE` | `war3.w3mod:units/undeadupgradefunc.txt`          |
| `UNDEAD_UPGRADES_NAME_EXTRACTION_RULE`| `enus.w3mod:units/undeadupgradestrings.txt`       |

### Localized string rules (read `enus.w3mod:units/*strings.txt`)

These are produced by the `race_strings!` macro in
`src/strings/mod.rs`. Each macro invocation expands to two
`ExtractionRule` constants — one ability, one unit. The
matcher requires both `path.contains("enus.w3mod:units")` and the
exact filename suffix.

| Rule                                       | Source CASC path                                    |
| ------------------------------------------ | --------------------------------------------------- |
| `HUMAN_ABILITY_STRINGS_EXTRACTION_RULE`    | `enus.w3mod:units/humanabilitystrings.txt`          |
| `HUMAN_UNIT_STRINGS_EXTRACTION_RULE`       | `enus.w3mod:units/humanunitstrings.txt`             |
| `NIGHTELF_ABILITY_STRINGS_EXTRACTION_RULE` | `enus.w3mod:units/nightelfabilitystrings.txt`       |
| `NIGHTELF_UNIT_STRINGS_EXTRACTION_RULE`    | `enus.w3mod:units/nightelfunitstrings.txt`          |
| `ORC_ABILITY_STRINGS_EXTRACTION_RULE`      | `enus.w3mod:units/orcabilitystrings.txt`            |
| `ORC_UNIT_STRINGS_EXTRACTION_RULE`         | `enus.w3mod:units/orcunitstrings.txt`               |
| `UNDEAD_ABILITY_STRINGS_EXTRACTION_RULE`   | `enus.w3mod:units/undeadabilitystrings.txt`         |
| `UNDEAD_UNIT_STRINGS_EXTRACTION_RULE`      | `enus.w3mod:units/undeadunitstrings.txt`            |
| `NEUTRAL_ABILITY_STRINGS_EXTRACTION_RULE`  | `enus.w3mod:units/neutralabilitystrings.txt`        |
| `NEUTRAL_UNIT_STRINGS_EXTRACTION_RULE`     | `enus.w3mod:units/neutralunitstrings.txt`           |
| `ITEM_ABILITY_STRINGS_EXTRACTION_RULE`     | `enus.w3mod:units/itemabilitystrings.txt`           |
| `ITEM_UNIT_STRINGS_EXTRACTION_RULE`        | `enus.w3mod:units/itemstrings.txt`                  |
| `CAMPAIGN_ABILITY_STRINGS_EXTRACTION_RULE` | `enus.w3mod:units/campaignabilitystrings.txt`       |
| `CAMPAIGN_UNIT_STRINGS_EXTRACTION_RULE`    | `enus.w3mod:units/campaignunitstrings.txt`          |
| `COMMON_ABILITY_STRINGS_EXTRACTION_RULE`   | `enus.w3mod:units/commonabilitystrings.txt`         |
| `COMMON_UNIT_STRINGS_EXTRACTION_RULE`      | `enus.w3mod:units/commonstrings.txt`                |

The matchers are deliberately disjoint — `tests/rules.rs::strings`
pins this. A path can match at most one of the per-race string rules,
so reordering them in `EXTRACTION_RULES` is safe.

### `GAME_EXTRACTION_RULE` (special, not in `EXTRACTION_RULES`)

Defined in `game.rs`, exported separately. Matcher returns `true` for
every entry; target is `Raw`; processor returns `ExtractResult::IO`.
Used by the `dump_casc` example to dump the entire CASC tree to disk.
Not part of the production pipeline.

---

## On-disk formats the rules parse

### SLK (Symbolic Link / SYLK)

Tab-separated table format from the 1980s, still used by Blizzard for
`unitbalance.slk`, `abilitydata.slk`, `unitabilities.slk`, etc. Each
line starts with a single-letter tag:

```
ID;P                              ← header
C;X1;Y1;K"alias"                  ← C = column header at row 1, col 1, value "alias"
C;X2;Y1;K"sortBalance"
C;X1;Y2;K"hpea"                   ← row 2 col 1: value "hpea" (peasant)
C;X2;Y2;K"a"
E                                 ← end of file
```

`crates/warcraft-slk` parses this into an `SlkTable` with
`columns: BTreeMap<ColumnKey, String>` and
`rows: BTreeMap<RowKey, BTreeMap<ColumnKey, String>>`. Quoted strings
have their surrounding quotes stripped. Use
`table.get(row_key, column_name)` to read a cell.

The first content row (`Y2`) is conventionally the column-name row.
Every subsequent row is a data row keyed by the value in column 1.

The repo has a single-file `warcraft-slk/src/lib.rs` (~550 LOC, no
deps) — minimal but sufficient. It does **not** support formulas,
boolean cells, or globals. Cells are always strings; rules parse them
into typed values themselves.

### Section format (`.txt` files)

Used by `abilityfunc.txt`, `unitfunc.txt`, `commandfunc.txt`,
`*upgradefunc.txt`, `*strings.txt`, `customkeys.txt`. Structure:

```
// optional comment lines
[id]                           ← section header, e.g. [hpea] or [AHbz]
Art=ReplaceableTextures\…\Foo.blp
Tip=Build |cffffcc00P|reasant
Ubertip="Trains the peasant.","Now upgrades faster."
Researchart=ReplaceableTextures\…\Bar.blp
Researchbuttonpos=0,2
```

- Section ids in `[…]` are the canonical 4-char Warcraft IDs (`hpea`,
  `AHbz`).
- Keys are case-insensitive; the per-rule processors lowercase before
  matching.
- Values can be plain (`0,2`) or comma-separated quoted lists
  (`"Level 1 tip","Level 2 tip"`). The `QuotedListParser` in `lib.rs`
  handles the latter.
- Comments start with `//`. The system-keybinds processor uses
  comments specifically — the comment immediately above a `[id]`
  becomes that entry's `comment` field.
- Line endings: CRLF or LF, both tolerated. UTF-8 BOM at file start
  is stripped (see `unit_skin_processor_strips_utf8_bom` test).

### Customkeys.txt (system keybinds)

The `customkeys.txt` shipped with the game is a section file with this
shape:

```
// Menu - Quit Game
[FileQuitMission]
Hotkey=81           ← virtual-key code (81 = 'Q')

// Game - Send Chat Message
[ChatMessage]
Hotkey=13           ← VK_RETURN
```

Every section has:
- `Hotkey=<u32 VK code>` — the default key
- optional `Modifier=Alt|Ctrl|Shift|CtrlOrAlt`
- (implicit) class — `SystemKeybindClass::Menu` for `File*` /
  `Menu*` ids, `ControlGroup` for `Group<n>`, etc. The classification
  is hard-coded in `system_keybinds.rs::classify_section_id`.

The comment immediately above each section becomes the human-readable
description shown in the editor's system-hotkeys dialog.

### DDS (DirectDraw Surface)

Reforged ships icons as `.dds` files (BC1/BC3 compressed). The
`DdsDecoder` in `src/image.rs` decodes via `bcdec_rs` + `ddsfile`.
Production pipeline does not write images — they're handled by a
separate asset extractor outside this repo. Available here for the
inspector examples and any future tooling.

### FDF (Frame Definition File)

Not currently parsed. If a future rule needs to read from
`UI/*.fdf`, it would need its own parser — `warcraft-slk` does not
help.

---

## The `db.rs` codegen layout

The generated file looks like this (lightly elided):

```rust
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// !! AUTO-GENERATED. DO NOT EDIT THIS FILE. EDITING IS BANNED.          !!
// !! THIS FILE IS WIPED AND REGENERATED ON EVERY PATCH IMPORT.          !!
// !! ANY CHANGE YOU MAKE HERE WILL BE GONE. DO NOT TOUCH THIS FILE.     !!
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use std::sync::LazyLock;
use warcraft_api::*;
use warcraft_api::WarcraftObjectKind::*;
use warcraft_api::Race::*;
use warcraft_api::UnitKind::*;
use warcraft_api::ItemClass::*;

pub static WARCRAFT_DATABASE: LazyLock<WarcraftDatabase> = LazyLock::new(|| {
    let mut objects = std::collections::BTreeMap::new();
    insert_objects_chunk_0(&mut objects);
    insert_objects_chunk_1(&mut objects);
    // … one call per chunk of 200 objects
    WarcraftDatabase::new(objects)
});

fn insert_objects_chunk_0(objects: &mut std::collections::BTreeMap<…>) {
    objects.insert(
        WarcraftObjectId::new("hpea"),
        WarcraftObject::with_text(
            WarcraftObjectId::new("hpea"),
            HPEA_NAMES,                  // → const HPEA_NAMES: &[&str]
            HPEA_ICONS,                  // → const HPEA_ICONS: &[&str]
            Unit,                        // WarcraftObjectKind variant
            Some(Human),                 // Race
            WarcraftObjectMeta::Unit(UnitMeta::with_full_and_extras(
                Worker, 15,
                HPEA_UNIT_ABILITIES,     // → const _: &[WarcraftObjectId]
                HPEA_UNIT_HERO_ABILITIES,
                UnitProduction::new(…),
                UnitFlags::new(false, true, false, false))
                .with_combat(UnitCombat::new(…)),
            ),
            WarcraftObjectText::new(
                HPEA_TIP_LEVELS,         // → const _: &[&str]
                HPEA_UBERTIP_LEVELS,
            ),
        ),
    );
    // … 199 more inserts
}

// … chunks 1..N

pub static WARCRAFT_GAMEPLAY_CONSTANTS: GameplayConstants =
    GameplayConstants::new(StrengthBonuses::new(…), …);

pub static WARCRAFT_SYSTEM_KEYBINDS: &[SystemKeybind] = &[
    SystemKeybind::new("FileQuitMission", "Quit Mission", 81,
        SystemKeybindModifier::None, SystemKeybindClass::Menu),
    // …
];

const HPEA_NAMES: &[&str] = &["Peasant"];
const HPEA_ICONS: &[&str] = &["ReplaceableTextures\\CommandButtons\\BTNPeasant.blp"];
const HPEA_UNIT_ABILITIES: &[WarcraftObjectId] = &[
    WarcraftObjectId::new("Aatk"),
    WarcraftObjectId::new("Amov"),
    // …
];
const HPEA_COOLDOWNS: [u32; 4] = [0, 0, 0, 0];
// …
```

Key properties of the codegen:

1. **Chunked.** Without chunking, the giant `LazyLock` body blows
   rustc's frame-size budget. 200 entries per chunk is the empirically
   safe size.
2. **Interned slices.** Names, icons, ability lists, tooltip levels,
   and cooldown arrays are emitted as named `const`s and referenced
   from inside the `WarcraftObject` constructors. This keeps the
   constructor calls short and readable in diffs.
3. **Identifier normalization.** A 4-char Warcraft ID like `Hpea` is
   normalized to `HPEA` (uppercase ASCII; non-ASCII chars become `_`)
   for the const name. `CodegenContext::normalize_identifier` is the
   single source of truth.
4. **No formatting on emit.** The codegen writes everything on long
   lines on purpose. Formatting after the fact is rustfmt's job. This
   keeps the emitter simple and bug-free, and means a regenerated
   `db.rs` always wants `cargo fmt` afterwards.
5. **No metadata footer.** The file does not record the patch version
   it was generated from. The patch is in
   `.cargo/config.toml::WARCRAFT_SUPPORTED_VERSION` and (currently)
   in `crates/warcraft-api/src/domain/mod.rs::SUPPORTED_VERSION_STRING`.

---

## Verification + rollback

In `main.rs`, the relevant types are `StagedWrite` and
`run_cargo_check`. The flow:

```
1. let bytes = render_database(...);                  // pure Rust → String
2. let staged = StagedWrite::stage(path, bytes)?;     // reads pre-image, writes new
3. match run_cargo_check("warcraft-database") {
     Success         => staged.commit();              // sets committed=true
     Failure { msg } => drop(staged);                 // Drop restores pre-image
                        std::process::exit(1);
   }
```

`StagedWrite::Drop`:

```
if !committed {
    match pre_image {
        Some(bytes) => fs::write(target, bytes),       // restore previous
        None        => fs::remove_file(target),         // delete if didn't exist
    }
}
```

This is intentionally simpler than the original `Transaction` /
`Sha256Manifest` from vk-overlay — the extractor only ever writes a
single file (`db.rs`), so multi-file atomicity isn't needed.

The `--no-verify` flag bypasses the entire verify+rollback flow. Use
it only when actively debugging a broken downstream crate where you
expect the verify to fail and want to see the raw output anyway.

---

## Diagnostic loop: a value in `db.rs` is wrong

Scenario: the editor shows the Phoenix Fire ability with cooldown 0,
but in-game it's clearly 0.5 seconds.

**Wrong reflex:** open `db.rs`, find `PHOENIX_FIRE_COOLDOWNS`, set to
`[500, 500, 500, 500]`. The next regeneration deletes your edit.

**Right loop:**

1. **Find the rule that owns the field.** Cooldowns come from
   `abilities.rs` (extracts from `units/abilitydata.slk`) and feed
   `AbilityMetadataDatabase`. Scopes: cooldowns per ability live in
   one place.

2. **Inspect the raw source.** Use `inspect_slk` to see what the SLK
   actually contains for that ability id (e.g. `AOcl` for Phoenix Fire):

   ```sh
   cargo run -p warcraft-extractor --example inspect_slk -- \
       "$W3_CASC" units/abilitydata.slk AOcl
   ```

   If the SLK row's `Cool1`..`Cool4` cells show `0.5`, the rule is
   the bug: the rule reads cooldowns as integer milliseconds but the
   SLK gives seconds, so `0.5` truncates to `0`. If the SLK row
   itself shows `0`, the bug is in the SLK (a Blizzard data error or
   a column shift across patches).

3. **Write a failing test.** In `tests/rules.rs`, add a test that
   feeds a synthetic SLK with the expected cell values and asserts
   the resulting `AbilityMetadataEntry::cooldowns()` is `[500, 500,
   500, 500]`. Run it to confirm it fails.

4. **Fix the rule.** Adjust the parsing to multiply seconds by 1000,
   round-to-nearest, etc. Re-run the test until it passes.

5. **Regenerate and commit.** `cargo run -p warcraft-extractor`,
   `cargo fmt -p warcraft-database`. Commit both the rule change and
   the regenerated `db.rs`.

For other classes of fields:

| Wrong field                          | Owner module               | Inspect with                             |
| ------------------------------------ | -------------------------- | ---------------------------------------- |
| Default button position (column/row) | `default_positions.rs`, `command_defaults.rs`, `ability_defaults.rs` | `inspect_abilityfunc` against the relevant `*func.txt` |
| Display name                         | `strings/mod.rs`           | `inspect_abilityfunc` against `*strings.txt` (or grep CASC) |
| Tooltip / ubertip text               | `object_texts.rs`          | `inspect_abilityfunc`                    |
| Icon path                            | `skins/`                   | `inspect_abilityfunc` against `*skin.txt` |
| Unit attack/hp/armor numbers         | `units.rs` (unit_data + unit_balance) | `inspect_slk units/unitbalance.slk <id>` |
| Hero attribute growth                | `units.rs::hero_attributes`| `inspect_slk units/unitbalance.slk <id>` |
| Hero ability list / max-level         | `heroes.rs`                | `inspect_slk units/abilitydata.slk <id>` |
| Item ability slot                    | `items.rs`                 | `inspect_slk units/itemdata.slk <id>`    |
| System hotkey VK code                | `system_keybinds.rs`       | `inspect_abilityfunc customkeys.txt <section>` (or grep CASC) |
| Damage matrix / max hero level       | `gameplay_constants.rs`    | `inspect_abilityfunc miscgame.txt`       |

If the bug is in the cascade composition (raw values look right but
the assembled `WarcraftObject` is wrong), the bug is in
`extractor/src/db.rs::WarcraftDataAggregation::get_ids` or the
`build_object_with_text` / metadata builder helpers. That's where
`object_text_lookup`, `resolve_object_tip_levels`,
`substitute_placeholders`, and skin-overlay merge happen.

---

## Adding a new extraction rule

You want a new field — e.g. ability "casting time" — exposed in
`db.rs`. Steps:

1. **Find the source.** Run `inspect_slk` against the suspected SLK
   to confirm the column exists and check its name. For
   `units/abilitydata.slk`, casting time is probably `Cast1`..`Cast4`.

2. **Decide where it lives.** A new field on `AbilityMetadataEntry`?
   A new `ExtractResult` variant with its own database?
   - Adding a field to an existing entry is easier — extend the
     struct in `abilities.rs`, populate it from the same SLK row.
   - A new variant means a new rule, a new processor, and an aggregator
     update.

3. **Define the field.** In the entry struct, add `casting_seconds:
   [f32; 4]` (use named struct fields, not tuples — see RUST_STYLE.md).
   Add a getter.

4. **Populate it.** In the processor that builds that entry, read
   the SLK columns and parse to `f32`.

5. **Test it.** Add a test in `tests/rules.rs` that feeds a synthetic
   SLK with known values and asserts the parsed entry exposes them.

6. **Plumb it through aggregation.** If a downstream consumer
   (`extractor/src/db.rs` → `WarcraftDatabase`) needs to use the new
   field, add it to the relevant builder. Add a getter on
   `warcraft_api::AbilityMeta` if the field needs to surface in
   `db.rs`.

7. **Update codegen.** In `main.rs::CodegenContext::emit_object_metadata`,
   include the new field in the emitted constructor call. Match the
   new shape of the `AbilityMeta` constructor or builder.

8. **Regenerate.** `cargo run -p warcraft-extractor`. Verify
   `db.rs` now contains the new field and `cargo check
   -p warcraft-database` passes (it will fail if `warcraft-api` or
   the codegen are out of sync — fix until clean).

9. **Use it in the editor** (separate change in `hotkey-editor`).

If your new rule reads a CASC file no other rule reads:

1. Create `crates/warcraft-extractor/src/<name>.rs`.
2. Define a `pub static <NAME>_EXTRACTION_RULE: ExtractionRule =
    ExtractionRule { matcher: …, target: ExtractTarget::Text,
    output_path: |_, _| PathBuf::new(), processor: … };`
3. `pub use <name>::*;` in `lib.rs`.
4. Add a new `ExtractResult::<Name>(<NameDatabase>)` variant.
5. Add the rule to `EXTRACTION_RULES` in `main.rs`.
6. Add a field on `WarcraftDataAggregation` and a `From<…>` arm.
7. Add `tests/rules.rs::<name>` with at least matcher + processor tests.

---

## The four inspector examples

All under `crates/warcraft-extractor/examples/`. Each takes a CASC
root as the first arg.

### `dump_casc <casc-root> <dest-dir>`

Dumps every CASC entry to `dest-dir`, preserving the namespaced path
structure. Useful when you want to grep across all data files
locally instead of repeatedly hitting CASC. ~tens of thousands of
files, ~2-4 GB on disk for Reforged.

```sh
cargo run -p warcraft-extractor --example dump_casc -- "$W3_CASC" /tmp/casc-dump
```

### `find_files <casc-root> [pattern]`

Lists every CASC entry whose name contains `pattern` (default:
`selecthero`). Use when you suspect a file exists but don't know its
exact path.

```sh
cargo run -p warcraft-extractor --example find_files -- "$W3_CASC" abilityskin
```

### `inspect_slk <casc-root> <slk-suffix> [id...]`

Merges every CASC SLK entry whose path ends in `<slk-suffix>`, prints
column headers, then prints each row whose primary id matches a
filter. Empty filter list = print column headers only.

```sh
# Show all columns:
cargo run -p warcraft-extractor --example inspect_slk -- "$W3_CASC" units/unitbalance.slk

# Show specific units:
cargo run -p warcraft-extractor --example inspect_slk -- "$W3_CASC" units/unitbalance.slk hpea hfoo
```

The example tries `unitAbilID`, `alias`, `unitID`, `unitUIID`,
`unitBalanceID`, `abilCode` as candidate primary-key columns. Falls
back to the first column if none match.

### `inspect_abilityfunc <casc-root> <filename> [id...]`

Reads every CASC entry whose name ends in `<filename>` (e.g.
`nightelfabilityfunc.txt`) and prints the parsed sections matching
the id filter. Use for `*func.txt`, `*strings.txt`, `*skin.txt`,
`commandfunc.txt`, `miscgame.txt`, `customkeys.txt`.

```sh
cargo run -p warcraft-extractor --example inspect_abilityfunc -- \
    "$W3_CASC" abilityskin.txt AOcl
```

---

## Failure modes and fixes

| Symptom                                                              | Likely cause                                                       | Fix                                                                                                       |
| -------------------------------------------------------------------- | ------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------- |
| `error: CASC root does not exist`                                    | `W3_CASC` unset or pointing at the wrong dir                       | Set `W3_CASC` to the `Data/` directory of the install, or pass `--casc <PATH>`                            |
| `failed to open CASC storage`                                        | Right path, but install is corrupt or locked by a running game     | Close Warcraft III; verify integrity in the launcher                                                      |
| `is cmake not installed?` during `casclib-sys` build                 | Outside `nix develop` shell                                        | Enter the dev shell, or use `nix run .#extract`                                                            |
| `unable to find library -lz`                                         | Outside dev shell, or shell missing `zlib`                         | Same as above                                                                                              |
| Compile error in `db.rs` after extraction (verify failed → rolled back) | `warcraft-api` shape changed; codegen now emits stale calls    | Update `main.rs::CodegenContext::emit_object_metadata` to match the new API. Also ensure `warcraft-api` is at the version this extractor expects |
| New patch landed; `db.rs` regenerates but values are subtly wrong    | A SLK column was renamed / shifted                                 | Use `inspect_slk` to confirm new column shape; update the relevant rule's SLK column lookup                |
| New patch added a new ability/unit; not appearing in editor          | Could be: missing in CASC dump (run `find_files` to confirm), or rule's matcher excludes it, or aggregator filters it | Trace top-down: confirm CASC has it, confirm a rule matches, confirm aggregator includes it |
| `casclib-sys` fails to build with linker errors mentioning `z` or `stdc++` | Stale `target/` from a previous half-broken build              | `cargo clean -p casclib-sys && cargo run -p warcraft-extractor`                                            |
| Test `tests/rules.rs::<name>::<…>` fails after a refactor             | The rule's parsing changed but the test's expected output didn't  | If parsing is now correct, update the test. If parsing regressed, fix the rule                            |
| Generated diff is huge after a regeneration with no actual changes   | Forgot to run `cargo fmt -p warcraft-database`                    | Run it; diff should shrink to near-zero                                                                    |

---

## What the extractor is NOT

The extractor reads **static, patch-shipped data** out of CASC and
turns it into Rust source. It does not:

- read live process memory of Warcraft III,
- attach to or inject into the running game,
- talk to Battle.net or any online service,
- decrypt encrypted CASC content,
- modify or repackage any CASC file,
- ship Blizzard's icon art or sound assets to the user.

Extracted strings (unit names, tooltips) and structural data
(positions, cooldowns) are bundled into the editor because they're
needed to render the UI. The editor's icons are reused via
`ReplaceableTextures\…\BTN<thing>.blp` paths that point back into
the user's own install at runtime — the editor never re-hosts art.

If you're looking for memory-reading or live-game-monitoring code,
you're in the wrong repo. That tooling lives elsewhere and is
intentionally not open-sourced.

---

## Glossary

- **CASC** — "Content Addressable Storage Container", Blizzard's
  storage format. Replaces the older MPQ format.
- **`war3.w3mod:`** — the gameplay-data namespace inside CASC.
- **`enus.w3mod:`** — English (US) localized strings.
- **SLK** — Symbolic Link / SYLK file. Tab-separated table format.
- **section file** — a `.txt` file with `[id]` headers, used for
  ability/unit/upgrade/skin metadata and the customkeys template.
- **rule** — one matcher + one processor, defined in
  `crates/warcraft-extractor/src/<name>.rs`.
- **aggregator** — `WarcraftDataAggregation`; collects per-rule
  outputs and composes the final `WarcraftDatabase`.
- **codegen** — `CodegenContext` in `main.rs`; turns the database
  into emittable Rust source.
- **chunk** — a function `insert_objects_chunk_N` containing 200
  `objects.insert(...)` calls. Splits the giant database init across
  many functions to fit rustc's frame budget.
- **interned slice** — a named `const` (e.g. `HPEA_NAMES: &[&str]`)
  emitted once and referenced from every object that uses those
  exact values. Reduces line length in object constructors and lets
  rustc deduplicate `&str` literals.
- **object id** — the canonical 4-char Warcraft ID (`hpea`, `AHbz`,
  `Aatk`). Wrapped as `WarcraftObjectId` in `warcraft-api`.
- **skin overlay** — the Reforged-graphics replacement art layer.
  `units/*skin.txt` overrides icon paths from the base `*func.txt`
  files when the user is running Reforged graphics mode.
- **placeholder substitution** — Warcraft tooltips contain tokens
  like `<AOcl,DataA1,%>` that are replaced by data values at
  display time. The extractor pre-substitutes these in
  `extractor/src/db.rs::substitute_placeholders` so the editor
  doesn't need to.
- **Warcraft format codes** — color/style markup like
  `|cffffcc00…|r` embedded in tooltips. Stripped during cascade by
  `strip_wc3_format_codes`.

---

## Where to look first

If you're an agent or human dropped into this codebase to fix
something extraction-related:

1. **`docs/EXTRACTION.md` — this doc.** You're already here.
2. **`crates/warcraft-extractor/src/main.rs`** — pipeline entry,
   codegen, CLI flags, verify/rollback.
3. **`crates/warcraft-extractor/src/lib.rs`** — `ExtractionPipeline`,
   `ExtractionRule`, `ExtractResult`, helper parsers.
4. **`crates/warcraft-extractor/src/<rule>.rs`** — the rule that
   owns the field you care about. The table in
   ["Diagnostic loop"](#diagnostic-loop-a-value-in-dbrs-is-wrong)
   tells you which.
5. **`crates/warcraft-extractor/src/db.rs`** — aggregator + cascade
   composition. Read this if raw extraction looks right but the
   final `WarcraftObject` is wrong.
6. **`crates/warcraft-extractor/tests/rules.rs`** — every rule's
   matcher + processor pinned against synthetic input. Failing test
   here is fastest way to understand intended behavior.
7. **`crates/warcraft-api/src/`** — the typed shapes the codegen
   targets. If you're adding a field, this is where you add the
   getter the editor reads.

Do NOT start in `crates/warcraft-database/src/db.rs`. It is output,
not input.
