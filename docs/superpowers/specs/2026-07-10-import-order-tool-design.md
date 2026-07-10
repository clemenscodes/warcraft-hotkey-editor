# Import- und Modul-Ordering-Tool — Design

**Datum:** 2026-07-10
**Status:** Design, freigegeben zur Planung

## Problem

Formatierung und Import-Sortierung sind über die Codebase inkonsistent:
Mal Leerzeile zwischen `mod`- und `use`-Blöcken, mal nicht; die Reihenfolge
von `mod`- und `use`-Statements ist willkürlich. `rustfmt`/`clippy` können das
gewünschte Schema **grundsätzlich nicht** erzwingen:

- Stable-`rustfmt` bietet nur `reorder_imports` und `reorder_modules` — beides
  rein **alphabetisch, visibility-blind**.
- `group_imports = StdExternalCrate` ist (a) nightly-only/unstable, (b) sortiert
  **std zuerst** (nicht extern), (c) kennt **keine Workspace-Crates als eigene
  Stufe** — für `rustfmt` ist `warcraft_api` dasselbe wie `serde`, es liest keine
  `cargo metadata`.
- `clippy` hat für Import-/Mod-Ordering gar nichts.

Deshalb: ein **eigenes, kleines Tool**, das das Schema exakt erzwingt.
Präzedenzfall im Ökosystem ist `tw-lint` (selbstgebauter CI-Enforcer).

## Ziel-Schema (normativ)

Kanonische Datei-Spitze, an einem `mod.rs`-Beispiel:

```rust
//! optionale Innen-Doc bleibt ganz oben

pub mod alpha;
pub mod beta;
pub(crate) mod gamma;
mod delta;

use anyhow::Result;                    // 1) extern (non-workspace), alphabetisch
use serde::Serialize;

use std::collections::HashMap;         // 2) std/core/alloc

use warcraft_api::WarcraftObjectId;    // 3) workspace (aus cargo metadata)
use warcraft_keybinds::CustomKeys;

use crate::services::Foo;              // 4) crate::

use super::super::data::EMPTY_PROMPT;  // 5) super::*  (mehr super = weiter = früher)
use super::sibling::Thing;

use self::view::FooView;               // 6) self::

// erstes echtes Item …
```

### Ordering-Regeln

1. **`mod`-Block zuerst**, dann genau eine Leerzeile, dann `use`-Block, dann
   genau eine Leerzeile vor dem ersten echten Item.
2. **`use`-Distanzgruppen**, jede durch **genau eine Leerzeile** getrennt, in
   dieser Reihenfolge (weitester Weg zuerst):
   1. **extern** — Crates, die weder std noch Workspace-Member sind.
   2. **std** — `std`, `core`, `alloc`.
   3. **workspace** — Crates, die Workspace-Member sind (aus `cargo metadata`).
   4. **crate::** — `use crate::…`.
   5. **super::\*** — relative Elternpfade; **mehr `super` = weiter weg = früher**
      (`super::super::super` vor `super::super` vor `super`).
   6. **self::** — nächster Weg, kommt zuletzt.
3. **Innerhalb jeder Gruppe** (mods wie uses):
   Visibility-Stufe **`pub` > `pub(crate)` > `pub(super)`/`pub(in …)` > privat**,
   danach alphabetisch nach vollem Pfad.
4. Nur **befüllte** Gruppen erzeugen einen Block — keine Geister-Leerzeilen für
   leere Gruppen.

### Distanz-Klassifikation

`cargo metadata --no-deps` liefert die Workspace-Member-Package-Namen
(`-`→`_` normalisiert). Das **erste Pfad-Segment** eines `use` entscheidet:

- `std` / `core` / `alloc` → **std**
- `crate` → **crate::**
- `super` → **super::\*** (Tiefe = Anzahl führender `super`)
- `self` → **self::**
- sonst: Lookup gegen die Workspace-Member-Menge → **workspace**, andernfalls
  → **extern**.

## Engine

Gewählt: **`syn` nur zum Klassifizieren, Umsortieren per Source-Text-Spans.**

- `syn` parst die Datei zu `syn::File` und liefert die geordnete Liste der
  führenden Items mit Kind (`mod`/`use`), Visibility, erstem Pfad-Segment und
  **Byte-Span**. Äußere Attribute (`#[cfg]`, `#[macro_use]`) und Doc-Comments
  (`///`, als `#[doc]`-Attribut) sind Teil des Item-Spans.
- Jeder Span wird **nach oben** um unmittelbar vorangehende `//`-Kommentarzeilen
  erweitert (keine Leerzeile dazwischen → gehört zum Item).
- Die **Original-Textblöcke** werden umsortiert und neu zusammengesetzt. Es wird
  **nicht** aus dem AST neu gedruckt → Kommentare, Formatierung und
  rustfmt-Kompatibilität bleiben exakt erhalten.

Verworfene Alternativen:

- **Reine Zeilen-Parserei (kein `syn`):** bricht bei mehrzeiligen `use`-Trees,
  `pub(in path)`, cfg-Attributen. Fragil.
- **`syn` + `prettyplease`-Reprint:** `syn` wirft `//`-Kommentare weg,
  `prettyplease` reprintet die ganze Datei → Kommentare verloren, Formatierung
  weicht von rustfmt ab.

## Scope

- **Alle Crates dieses Repos** (jede `.rs` im Workspace: `hotkey-editor`,
  `gallery`, alles unter `crates/`). Die gepatchten Sibling-Repos (`ddd`,
  `tw-lint`) bleiben unberührt.
- Das Tool normalisiert die **führende Item-Region** jeder Datei/Modul-Spitze
  (der zusammenhängende Lauf aus `mod`-Deklarationen + `use`-Statements + deren
  Attributen/Kommentaren, bis zum ersten echten Item). `mod foo { … }`-Blöcke
  mit Body und `use`/`mod` nach dem ersten echten Item werden **nicht** bewegt.
- `//!`-Innen-Docs und `#![…]`-Innen-Attribute bleiben ganz oben, über allem.
- **Kein** Merge/Split von `use`-Trees (kein `imports_granularity`). Bestehende
  Baum-Struktur bleibt; nur die Reihenfolge der Top-Level-`use`-Items ändert sich.

## Tool-Gestalt & Gate-Einbindung

- **In-Repo-Bin-Crate** (`tools/import-order/` oder `crates/import-order`),
  **nicht** vorschnell als Sibling-Repo extrahiert (Rule of Three). Später
  extrahierbar.
- CLI analog `cargo fmt`:
  - **`--check`** — Exit ≠ 0 bei Abweichung, Diff ausgeben.
  - **`--fix`** — in-place umschreiben.
- **moon-Tasks**, gespiegelt zu `rust/fmt` in `.moon/tasks/all.yml`:
  - `rust/import-order` (check) — hängt in `:ci`.
  - `rust/import-order/fix`.
- **Koexistenz mit rustfmt:** neue `rustfmt.toml` im Repo-Root mit
  `reorder_imports = false` und `reorder_modules = false` (beides **stable**).
  Damit ordnet rustfmt nichts mehr um; das Tool ist alleinige Ordnungs-Autorität.
  rustfmt macht weiter Einrückung/Wrapping/Blank-Line-Normalisierung.
- In `--fix` läuft das Tool **nach** `cargo fmt` (auf bereits normalisiertem Text
  umsortieren).
- Das Tool selbst befolgt `docs/RUST_STYLE.md` (wird von clippy im Gate mitgeprüft).

## Tests

Golden-File-Tests (Input → Erwartung) für:

- Alle Visibility-Stufen (`pub`, `pub(crate)`, `pub(super)`, `pub(in …)`, privat).
- Alle sechs Distanzgruppen inkl. mehrfacher `super`-Tiefe und Gruppen-Trennung.
- Workspace-Erkennung (Workspace-Member vs. externe Crate).
- Mehrzeilige `use`-Trees.
- `#[cfg(…)]`- und `#[macro_use]`-annotierte Items.
- Kommentar-Anheftung (führender `//`-Kommentar wandert mit seinem Item).
- Leerzeilen-Einfügung/-Normalisierung (mod↔use, zwischen Gruppen, nach use-Block).
- **Idempotenz** — Tool zweimal laufen lassen ⇒ keine Änderung.
- `--check`-Korrektheit (Exit-Code passend zu „braucht Änderung").

## Erfolgskriterien

- `cargo run -p import-order -- --check` (bzw. der moon-Task) ist über die ganze
  Codebase grün, nachdem `--fix` einmal gelaufen ist.
- `moon run :ci` bleibt grün (Tool-Crate erfüllt fmt/clippy; neuer check-Task passt).
- Jede Datei-Spitze entspricht exakt dem Ziel-Schema; erneutes `--fix` ändert nichts.
