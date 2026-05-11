# warcraft-extractor

Opens a Warcraft III CASC archive and extracts game assets — SLK data tables,
DDS images, and string files — producing either in-memory Rust data structures
(for `warcraft-database` generation) or files on disk (for the asset pipeline).

## What CASC is

CASC (Content Addressable Storage Container) is Blizzard's game archive
format, introduced with Warcraft III: Reforged. This crate uses a fork of
`casclib` to enumerate and read files from the archive without unpacking it.
The archive root is specified via the `W3_CASC` environment variable, which
should point to the game installation directory.

## Extraction rules

The library exposes a rule-based extraction API:

```rust
pub struct ExtractionRule {
    pub matcher:     fn(&str) -> bool,         // which CASC filenames to process
    pub target:      ExtractTarget,            // Text | Raw | Image
    pub output_path: fn(&str, &Path) -> PathBuf,
    pub processor:   fn(&str, &[u8]) -> Result<ExtractResult, ExtractError>,
}

extract_with_rules(casc_root, output_dir, &rules)?;
```

`ExtractTarget::Text` feeds the bytes into a `processor` function that
parses SLK / ini-format data and returns typed result structs.
`ExtractTarget::Image` decodes DDS textures (BC1/BC3/BC7 via `bcdec_rs`)
to RGBA and writes PNG files. `ExtractTarget::Raw` copies files verbatim.

## What it extracts

**Tabular game data** (returns `ExtractResult` variants):

- `HeroDatabase` — hero base stats from `HeroData.slk`
- `UnitDatabase` — unit stats from `UnitData.slk`
- `ItemDatabase` — item metadata from `ItemData.slk`
- Per-race ability, upgrade, and string databases from SLK and INI files

**Image assets** (written to disk):

- Hero and unit ability icons (`.blp`/`.dds` → PNG)
- Item icons
- Observer UI tile and frame textures

**Raw assets** (written verbatim):

- `frizqt.ttf` — the game's primary font

## Generating `warcraft-database`

```sh
export W3_CASC="/path/to/Warcraft III"
cargo run -p warcraft-extractor > crates/warcraft/database/src/db.rs
```

The binary serialises all `ExtractResult` variants to a Rust source file
that `warcraft-database` compiles directly into the overlay binary.

## Generating art assets

```sh
cargo run -p warcraft-assets -- crates/warcraft/overlay/assets
```

`warcraft-assets` calls `extract_with_rules` with image and raw rules and
then uses the extracted files to regenerate `warcraft-icons`,
`warcraft-images`, `warcraft-ui`, and `warcraft-fonts`.

## Related crates

- [`warcraft-slk`](../slk/README.md) — parses the SLK files read from CASC
- [`warcraft-database`](../database/README.md) — the generated output
- [`warcraft-assets`](../assets/README.md) — asset pipeline consumer
