# warcraft-slk

Parser for Warcraft III's SLK (SYLK — Symbolic Link) table format.

Warcraft stores unit stats, ability data, and item metadata in `.slk` files
inside its CASC archive. These are a subset of the SYmbolic LinK spreadsheet
format: each row encodes a cell by its column (`C;X<col>`) and row
(`C;Y<row>`) coordinates, with the value carried in the same record or
appended to the column descriptor.

## API

```rust
use warcraft_slk::{SlkTable, SlkParser};

let table: SlkTable = SlkParser::parse(slk_text)?;

// Column-name lookup
let value: Option<&str> = table.get(row_key, "HitPoints");

// Raw index lookup
let value: Option<&str> = table.get_by_index(row_key, column_key);

// Iterate all rows
for (row_key, row) in table.iter_rows() { ... }
```

## Usage in the extraction pipeline

`warcraft-extractor` opens the CASC archive, reads files such as
`Units/HeroData.slk`, `Units/UnitData.slk`, `Units/ItemData.slk`, and
`Units/AbilityData.slk`, and feeds their text content to this parser. The
resulting `SlkTable` is then queried by column name to build the typed
`HeroDatabase`, `UnitDatabase`, etc. that end up in `warcraft-database`.

## Related crates

- [`warcraft-extractor`](../extractor/README.md) — the only consumer
