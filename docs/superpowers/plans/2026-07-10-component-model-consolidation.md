# Component-Model-Konsolidierung Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use subagent-driven-development (recommended) or executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Konsolidiere die per-Komponenten-Struktur des Renderers auf drei klar benannte formale Rollen (`ddd::View` → `ddd::Model` → `ddd::Presentation`) mit einem einzigen Presentation-Builder pro Komponente (`presentation/`) und einer uniformen, splittbaren Verzeichnis-Struktur — ohne den View↔Modell-Split anzutasten.

**Architecture:** Pilot-first. Zuerst die ddd-Trait-Arbeit (`Props`→`Model`, neuer `Presentation`-Marker), dann zwei voll ausgearbeitete Referenz-Komponenten (eine connected, eine pure leaf), die das Zielschema als Role-Model festschreiben (wie `grid_editor`/`header`). Danach Sweeps pro Archetyp, jeder als Rezept mit einem echten Vorher/Nachher-Beispiel, Aufzählung der Zielmenge, Sonderfällen und `moon run :ci`-Gate.

**Tech Stack:** Rust 1.96.1 (stable), Dioxus 0.7.9 (wasm), ddd (Sibling-Repo, lokal gepatcht), tw-macro (Styling), moon (Task-Runner).

## Global Constraints

- **Einziges Verifikations-Kommando: `moon run :ci`.** Niemals `cargo check`/`cargo clippy`/`cargo test`/`dx`/einzelne `moon`-Targets. Niemals zwei `moon run :ci` gleichzeitig (CPU/Port-8123-Contention → Massen-Flakiness); vor jedem Lauf `pgrep -af "moon run|playwright"` prüfen.
- **Dev-Server: `moon run :dev`**, URL immer `http://localhost:8123/warcraft-hotkey-editor/` (trailing slash; bare `/` = 404). Bei „being rebuilt"-Overlay: **Seite neu laden**, nicht warten.
- **Drei formale Rollen, Pipeline `View → Model → Presentation`** (je ein ddd-Trait). `Model: From<&View>` ist ausnahmsloses Gesetz. `Presentation` ist die signalfreie, render-fertige Stufe, die der Body platziert; gebaut rein (`From<&Model>`) oder effektvoll.
- **Die Komponente nimmt IMMER genau EINEN Props-Struct `FooModel` als Parameter** — `fn Foo(props: FooModel)`. NIE die bare `View`, NIE inline-Felder. `view.rs` bleibt separater, distinkter Contract. View↔Modell = Entkopplung, keine Redundanz — `view.rs` wird NIE in `model/` verschmolzen.
- **Kein `Signal<T>` in `view/`, `model/` oder `presentation/`-Typen.** `use_signal`/`use_effect`/`use_memo`/Context-Reads leben ausschließlich im effektvollen Builder in `presentation/`. View/Model/Presentation tragen nur Werte + `EventHandler`.
- **Wall:** Kein Domain-Logik im Renderer. Keine neuen Imports aus `warcraft_keybinds::cascade`, keine `binding.set_*`.
- **RUST_STYLE mandatory** (volle semantische Namen, keine Tupel, keine `as`-Casts außerhalb `From`/`TryFrom`, private Felder + Accessoren, `Self` in impls, keine numerischen Suffixe, keine Section-Header-Kommentare). Wird von clippy im Gate erzwungen.
- **COMPONENTS.md-Regeln:** directory = component = class, pure-RSX-Body, Builder liefern DATA nie Element, Kind-Props via `From<&ParentProps>`.
- **Commit-Identität:** `Clemens <clemenscodes@gmail.com>`. Commits sind gpg-signiert (YubiKey-Touch) → **selten committen, pro Phase/Batch bündeln**, niemals Signing deaktivieren. `develop` ist Wegwerf (squash on merge) — keine Commit-Message-Hygiene-Sorgen.

**Verifikations-Modell:** Struktureller Refactor der Renderer-Crate; die Domain-Crate (`warcraft-keybinds`) wird **nicht** angefasst, es entstehen **keine neuen Unit-Tests**. „Test" heißt: `moon run :ci` grün (inkl. bestehender Tests + Playwright-e2e) **plus** Browser-Verifikation für Komponenten mit Verhalten (connected). Klassisches TDD-per-Unit entfällt mangels neuer Logik.

**Batching wegen Gate-Kosten:** `moon run :ci` ist teuer. Steps innerhalb einer Phase editieren mehrere Komponenten/einen Subtree ohne Zwischen-Gate; Gate + Commit kommen **pro Batch** (ein Subtree oder ~15–20 Komponenten).

---

## File Structure (Ziel-Anatomie pro Komponente)

```
foo/
  mod.rs              # #[component] fn Foo(props: FooModel) — reines RSX. Nimmt IMMER
                      #   EINEN Props-Struct, NIE die View, NIE inline-Felder. Baut via
                      #   presentation/ die FooPresentation, platziert deren Felder.
                      #   (Ist das Model render-fertig, platziert der Body es direkt.)
  view/mod.rs         # FooView — ddd::View, öffentlicher Contract (Typ re-exportiert).
                      #   Werte + EventHandler, NIE Signals. Splittbar.
  model/mod.rs        # FooModel — ddd::Model (ex-ddd::Props), empfangener Props-Struct,
                      #   From<&FooView>, #[derive(Props)]. Privat, nie re-exportiert.
  presentation/mod.rs # optional — FooPresentation (ddd::Presentation) UND ihr Builder.
                      #   Pure-shaping leaf: impl From<&FooModel>. Connected:
                      #   use_foo_presentation (besitzt Signals, liest Context). Absorbiert
                      #   owner-internes ex-logic.rs/ex-hooks.rs. Privat. Splittbar.
  style/mod.rs        # classes!-Consts. Splittbar.
  data/mod.rs         # optional — statischer Content. Splittbar.
  state/mod.rs        # optional — Diskriminanten-Enums + Runtime-UI-State.
  components/          # Kind-Komponenten (unverändert).
```

Drei Archetypen:
- **Pure passthrough leaf** (Model render-fertig): keine `presentation/`; Body platziert `FooModel`-Felder direkt.
- **Pure-shaping leaf** (heutige `logic.rs`-only): `presentation/mod.rs` mit `impl From<&FooModel> for FooPresentation`; kein effektvoller Builder.
- **Connected** (heutige `hooks.rs`): `presentation/mod.rs` mit `use_foo_presentation` (Signals/Context) → `FooPresentation`.

**Bestandszahlen:** 509 Komponenten; `style.rs` 509, `props.rs` 494, `view.rs` 491, `hooks.rs` 73, `logic.rs` 63 (+2 `logic/`-dirs), `data.rs` 34, `state.rs` 24. `impl ddd::Props` 492. Alle Belange außer `logic/` (2 dirs) sind flache Dateien.

---

## Phase 0: ddd `Props` → `Model` + `Presentation`-Marker (Fundament)

**Files:**
- Modify: `~/.local/src/ddd/crates/ddd/src/props.rs` → umbenennen nach `model.rs`
- Create: `~/.local/src/ddd/crates/ddd/src/presentation.rs`
- Modify: `~/.local/src/ddd/crates/ddd/src/lib.rs`
- Modify: alle `crates/hotkey-editor/src/**/props.rs` mit `impl ddd::Props` (492)

**Interfaces:**
- Produces: `ddd::View` (`pub trait View: Clone + PartialEq {}`), `ddd::Model` (`pub trait Model: for<'view> From<&'view Self::View> + Clone + PartialEq { type View: View; }`), `ddd::Presentation` (`pub trait Presentation { type Model: Model; }`).

- [ ] **Step 1: `view.rs` — `Clone + PartialEq`-Bound ergänzen**

`~/.local/src/ddd/crates/ddd/src/view.rs`: `pub trait View {}` → `pub trait View: Clone + PartialEq {}`. Doc-Kommentar behalten.

- [ ] **Step 2: `props.rs` → `model.rs` umbenennen**

`~/.local/src/ddd/crates/ddd/src/props.rs` → `src/model.rs`, Inhalt:

```rust
use crate::View;

/// A component's private internal model, built from its published [`View`].
///
/// A type cannot be `Model` without being `From<&View>` — so every internal model
/// publishes a contract, and the `From` conversion is the translation at the
/// component's boundary (an [`crate::AntiCorruptionLayer`] when the model is richer
/// than the contract). `Clone + PartialEq` because the model is the component's
/// Dioxus props. This is what forces "no model without a view".
pub trait Model: for<'view> From<&'view Self::View> + Clone + PartialEq {
    /// The published contract this internal model is built from.
    type View: View;
}
```

- [ ] **Step 3: `presentation.rs` anlegen**

`~/.local/src/ddd/crates/ddd/src/presentation.rs`:

```rust
use crate::Model;

/// A component's signal-free, render-ready presentation: the shaped data its body
/// places into the RSX, one render-tick snapshot with no reactive `Signal` inside.
///
/// Built from the component's [`Model`] — purely (`From<&Model>`) for a shaping leaf,
/// or effectfully (owning the local signals and context reads) for a connected
/// component. The associated `Model` wires the pipeline `View → Model → Presentation`
/// at the type level (`Self::Model::View` is navigable); there is no `From` supertrait
/// because the effectful build is not a pure conversion.
pub trait Presentation {
    /// The internal model this presentation is built from.
    type Model: Model;
}
```

- [ ] **Step 4: ddd `lib.rs` anpassen**

`mod props;` → `mod model;`, `+ mod presentation;`. `pub use props::Props;` → `pub use model::Model;`, `+ pub use presentation::Presentation;`. Doc-Kommentar-Referenzen auf `Props` auf `Model` aktualisieren.

- [ ] **Step 5: App-weit `impl ddd::Props` → `impl ddd::Model`**

```bash
cd /home/clemens/.local/src/warcraft-hotkey-editor
grep -rl 'ddd::Props' crates/hotkey-editor/src | while read f; do
  sed -i 's/ddd::Props/ddd::Model/g' "$f"
done
grep -rn 'ddd::Props' crates/hotkey-editor/src   # muss leer sein
```

- [ ] **Step 6: Gate** — `moon run :ci` → PASS (Rename + additive Bounds/Trait). Bei rot: übersehene `ddd::Props`-Referenz, ein `View`-Impl-Typ ohne `Clone`/`PartialEq`, oder Doctest im ddd.

- [ ] **Step 7: Commit**

```bash
cd ~/.local/src/ddd && git add -A && git commit -m "rename ddd::Props to ddd::Model, add ddd::Presentation"
cd /home/clemens/.local/src/warcraft-hotkey-editor && git add -A && git commit -m "adopt ddd::Model rename across renderer"
```

---

## Phase 1: Pilots — Referenz-Anatomie festschreiben

Zwei Komponenten voll auf das Zielschema bringen; sie werden Role-Model. **Erst wenn der User diese zwei absegnet, beginnen die Sweeps.**

### Task 1.1: Pilot A (connected) — `key_picker`

**Files** (Basis: `.../toolbar_actions/components/shared/dialogs/key_picker/`):
- Create: `view/mod.rs`, `model/mod.rs`, `presentation/mod.rs`, `style/mod.rs`, `state/mod.rs`
- Delete: `view.rs`, `props.rs`, `hooks.rs`, `logic.rs`, `style.rs`, `state.rs`
- Modify: `mod.rs`

**Interfaces:**
- Consumes: `ddd::{View, Model, Presentation}`. `KeyPickerCell`, `KeyPickerCellState` (state/). `KeyColumn` (shared board), `HotkeyToken`/`KeyCode` (`warcraft_keybinds`).
- Produces: das Referenz-Muster für connected Sweeps (Phase 3): `presentation/` baut effektvoll die `FooPresentation`; `mod.rs` nimmt `FooModel`; `model/` ist `From<&View>`, signalfrei.

**Ausgangslage (real):** `mod.rs` nimmt `KeyPickerProps`, ruft `use_key_picker(&props)` → `KeyPickerModel { open: Signal<bool>, … }` (Signal im Struct!), dann `KeyPickerShell::from(&model)`. `props.rs` = `KeyPickerProps` (`From<&KeyPickerView>`, `ddd::Model`). `view.rs` = `KeyPickerView`. `logic.rs` = `LetterColumnInputs`.

- [ ] **Step 1: `view/mod.rs` anlegen** — Contract, roh, signalfrei (feld-identisch zum heutigen `KeyPickerView`):

```rust
use super::state::KeyPickerCell;
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

/// The published `View` contract: title, board rows, open flag, conflict-pick policy,
/// and the pick/close handlers. Values + handlers only — never a Signal. (The
/// open-signal mirroring + on_open_change are presentation concerns, not the contract.)
#[derive(Clone, PartialEq)]
pub struct KeyPickerView {
    pub title: String,
    pub rows: Vec<Vec<KeyPickerCell>>,
    pub open: bool,
    pub allow_conflict_pick: bool,
    pub on_pick: EventHandler<HotkeyToken>,
    pub on_close: EventHandler<()>,
}

impl ddd::View for KeyPickerView {}
```

- [ ] **Step 2: `model/mod.rs` anlegen** — der **empfangene Props-Struct** (ex-`KeyPickerProps`, nur umbenannt): rohe Felder, `#[derive(Props)]`, `From<&KeyPickerView>`, `ddd::Model`. Signalfrei, **keine** geformten `columns` (die entstehen in `presentation/`):

```rust
use super::view::KeyPickerView;
use super::state::KeyPickerCell;
use dioxus::prelude::*;
use warcraft_keybinds::HotkeyToken;

/// The key picker's private internal model — the props the component receives. Mirrors
/// KeyPickerView field-for-field (decoupling, not duplication); the From<&View> is the
/// boundary translation. Signal-free; the board shaping + open-signal live in
/// presentation.
#[derive(Props, Clone, PartialEq)]
pub struct KeyPickerModel {
    #[props(into)]
    pub title: String,
    pub rows: Vec<Vec<KeyPickerCell>>,
    pub open: bool,
    #[props(default = false)]
    pub allow_conflict_pick: bool,
    pub on_pick: EventHandler<HotkeyToken>,
    pub on_close: EventHandler<()>,
}

impl From<&KeyPickerView> for KeyPickerModel {
    fn from(view: &KeyPickerView) -> Self {
        let KeyPickerView { title, rows, open, allow_conflict_pick, on_pick, on_close } = view.clone();
        Self { title, rows, open, allow_conflict_pick, on_pick, on_close }
    }
}

impl ddd::Model for KeyPickerModel {
    type View = KeyPickerView;
}
```

- [ ] **Step 3: `presentation/mod.rs` anlegen** — Typ `KeyPickerPresentation` (`ddd::Presentation`, signalfrei) **und** ihr effektvoller Builder: nimmt `&KeyPickerModel`, besitzt das lokale `open`-Signal, spiegelt den empfangenen `open`, feuert `on_close` beim Schließen, baut die Spalten, adaptiert `KeyCode`→`HotkeyToken`:

```rust
use super::model::KeyPickerModel;
use crate::components::app::components::shell::components::shared::key_picker_board::KeyColumn;
use dioxus::prelude::*;
use warcraft_keybinds::{HotkeyToken, KeyCode};

/// The picker's shaped, signal-free presentation: the built board columns, the current
/// open value, the title, and the adapted pick/close/open-change handlers. The body only
/// places these.
pub(super) struct KeyPickerPresentation {
    pub(super) open: bool,
    pub(super) title: String,
    pub(super) columns: Vec<KeyColumn>,
    pub(super) on_pick: EventHandler<KeyCode>,
    pub(super) on_close: EventHandler<()>,
    pub(super) on_open_change: EventHandler<bool>,
}

impl ddd::Presentation for KeyPickerPresentation {
    type Model = KeyPickerModel;
}

/// The one effectful edge: mirrors the received open flag into a local signal the dialog
/// shell can close (firing the caller's on_close when it does), builds the letter column,
/// and adapts the board's KeyCode pick back to the caller's HotkeyToken. Focus/keyboard
/// fallback belong to the board host — nothing here listens or focuses.
pub(super) fn use_key_picker_presentation(model: &KeyPickerModel) -> KeyPickerPresentation {
    let parent_on_close = model.on_close;
    let mut open_signal = use_signal(|| model.open);
    use_effect(move || {
        if !open_signal() {
            parent_on_close.call(());
        }
    });
    let on_open_change = EventHandler::new(move |next: bool| open_signal.set(next));
    let on_close = EventHandler::new(move |_event: ()| open_signal.set(false));
    let column_inputs = LetterColumnInputs {
        rows: model.rows.clone(),
        allow_conflict_pick: model.allow_conflict_pick,
    };
    let column = KeyColumn::from(column_inputs);
    let columns: Vec<KeyColumn> = vec![column];
    let letter_on_pick = model.on_pick;
    let on_pick = EventHandler::new(move |code: KeyCode| {
        if let Ok(token) = HotkeyToken::try_from(code) {
            letter_on_pick.call(token);
        }
    });
    let title = model.title.clone();
    let key_picker_presentation = KeyPickerPresentation {
        open: open_signal(),
        title,
        columns,
        on_pick,
        on_close,
        on_open_change,
    };
    key_picker_presentation
}
```

`LetterColumnInputs` (owner-intern, aus dem alten `logic.rs`) zieht als privater Helfer mit in `presentation/` (oder `presentation/letter_column.rs` beim Split). Entkommt nicht → privat.

- [ ] **Step 4: `style/mod.rs`, `state/mod.rs` anlegen** — Inhalt aus `style.rs`/`state.rs` unverändert (nur Datei→Verzeichnis).

- [ ] **Step 5: `mod.rs` neu verdrahten** — nimmt **einen** Props-Struct `KeyPickerModel`, baut via `presentation/` die Presentation, platziert:

```rust
pub mod components;
mod model;
mod presentation;
mod state;
mod style;
mod view;

pub use view::KeyPickerView;
pub use state::{KeyPickerCell, KeyPickerCellState};

use components::key_picker_panel::KeyPickerPanel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogRoot;
use model::KeyPickerModel;
use presentation::{use_key_picker_presentation, KeyPickerPresentation};
use style::CLASS;
use tw_macro::assert_component;

/// Assigns an ability hotkey from an on-screen letter keyboard. It owns its own dialog
/// shell: it takes the KeyPickerModel props, presentation mirrors the open flag into a
/// local signal and shapes the board, and this places the panel inside its own backdrop
/// `div` within the library `DialogRoot`. No project class touches the library element.
#[component]
pub fn KeyPicker(props: KeyPickerModel) -> Element {
    let presentation = use_key_picker_presentation(&props);
    use_body_scroll_lock(presentation.open);
    let KeyPickerPresentation { open, title, columns, on_pick, on_close, on_open_change } = presentation;
    rsx! {
        DialogRoot {
            open,
            on_open_change,
            div { class: CLASS,
                KeyPickerPanel { title, on_close, columns, on_pick, on_board_close: on_close }
            }
        }
    }
}

assert_component!(KeyPicker);
```

(Anmerkung: die genaue Feld-/Handler-Verdrahtung des `KeyPickerPanel` an das heutige `KeyPickerShell` angleichen; `KeyPickerShell` entfällt, `KeyPickerPresentation` übernimmt die Rolle. Beim Umsetzen die realen Panel-Props gegenprüfen.)

- [ ] **Step 6: alte Dateien löschen** — `rm view.rs props.rs hooks.rs logic.rs style.rs state.rs`

- [ ] **Step 7: Import-Pfade** — `view/mod.rs` und `model/mod.rs` referenzieren das Geschwister-Modul als `super::state::KeyPickerCell`. `presentation/mod.rs` nutzt `super::model::KeyPickerModel`. `KeyPickerCell`/`KeyPickerCellState` bleiben `pub use`'d aus `mod.rs`.

- [ ] **Step 8: Gate + Browser** — `moon run :ci` → PASS. Dann `moon run :dev`, Browser `http://localhost:8123/warcraft-hotkey-editor/`, Ability-Hotkey-Picker öffnen, Taste wählen, schließen — Verhalten unverändert. Playwright-MCP-Screenshot vorher/nachher.

- [ ] **Step 9: Commit** — `git commit -m "pilot: key_picker to View/Model/Presentation anatomy"`

### Task 1.2: Pilot B (pure-shaping leaf) — `race_tab_state`

**Files** (Basis: `.../race_tabs/components/shared/race_tab_state/`):
- Create: `presentation/mod.rs` (aus `logic.rs`; `RaceTabBehavior` → `RaceTabPresentation`, `From<&RaceTabBinding>`)
- Delete: `logic.rs`
- Modify: `mod.rs`

**Interfaces:**
- Consumes: `RaceTabBinding` (importierter Parent-Contract; spielt die Model/Props-Rolle für dieses Leaf — der Contract wird vom Parent `race_tabs` publiziert, daher **kein** eigenes `view.rs`/`model/`).
- Produces: das Referenz-Muster für pure-shaping-leaf Sweeps (Phase 2): `presentation/mod.rs` mit `impl From<&Contract> for FooPresentation`, **kein** `mod.rs`-Signal, **kein** effektvoller Builder.

**Ausgangslage (real):** `mod.rs` nimmt `RaceTabBinding`, baut `RaceTabBehavior::from(&props)` (in `logic.rs`), dispatcht `ActiveRaceTab`/`InactiveRaceTab`. Kein `view.rs`/`props.rs`/`hooks.rs`.

- [ ] **Step 1: `presentation/mod.rs` anlegen** — `logic.rs`-Inhalt übernehmen, `RaceTabBehavior` → `RaceTabPresentation`, `From<&RaceTabBinding>` bleibt. `impl ddd::Presentation for RaceTabPresentation { type Model = …; }` — **hier beißt der `type Model: Model`-Bound**: das Leaf hat heute kein eigenes Model, also muss `type Model` auf *etwas* zeigen, das `ddd::Model` ist.

  **Offene Pilot-Frage (importierter Parent-Contract), jetzt durch den Bound erzwungen:**
  - (a) `RaceTabBinding` IST das Model dieses Leafs → `type Model = RaceTabBinding`; dann muss `RaceTabBinding: ddd::Model` (braucht `From<&SomeView> + Clone + PartialEq` + `type View`). Prüfen, ob der Parent `race_tabs` `RaceTabBinding` als `ddd::Model` mit eigener View führen kann. Minimal an Struktur, aber Zwang auf den Parent-Typ.
  - (b) Das Leaf bekommt ein eigenes `RaceTabModel: From<&RaceTabView>` (+ `view/`, `model/`), `RaceTabBinding` wird dessen View → `type Model = RaceTabModel`. Volle Drei-Stufen-Pipeline, mehr Zeremonie, aber der Parent-Typ bleibt unangetastet.

  **Diese Wahl mit dem User am Pilot entscheiden — sie prägt alle imported-contract-Leaves.**

- [ ] **Step 2: `mod.rs` anpassen** — `mod logic;`→`mod presentation;`, `use logic::RaceTabBehavior;`→`use presentation::RaceTabPresentation;`. Body: `RaceTabPresentation::from(&props)`, Accessoren unverändert.

- [ ] **Step 3: `logic.rs` löschen** — `rm logic.rs`

- [ ] **Step 4: Gate + Browser** — `moon run :ci` → PASS; Race-Tabs klicken, aktiv/inaktiv-Look unverändert.

- [ ] **Step 5: Commit** — `git commit -m "pilot: race_tab_state pure-shaping leaf to presentation/"`

### Task 1.3: COMPONENTS.md — neue Anatomie dokumentieren

- [ ] **Step 1:** In `docs/COMPONENTS.md` die Konvention aktualisieren: Pipeline `View → Model → Presentation` (je ein ddd-Trait); `view/` (Contract), `model/` (ddd::Model, From<&View>, der Props-Parameter, signalfrei), `presentation/` (ddd::Presentation + Builder: rein `From<&Model>` oder effektvoll mit Signals), `style/`/`data/`/`state/`. Explizit: Komponente nimmt EINEN Props-Struct `FooModel`, nie View, nie inline-Felder; View↔Model = Entkopplung; kein Signal in view/model/presentation-Typen. Pilots `key_picker` (connected) + `race_tab_state` (pure-shaping leaf) als Role-Model.
- [ ] **Step 2: Gate + Commit** — `moon run :ci`; Commit `docs: View/Model/Presentation component anatomy`.

**→ HALT: User-Review der Pilots + COMPONENTS.md, bevor Phase 2 startet.**

---

## Phase 2: Pure-Leaf-Sweep (logic.rs, kein hooks.rs)

**Zielmenge:** Komponenten mit `logic.rs` und **ohne** `hooks.rs`, deren `logic`-Typen die Komponente **nicht** verlassen (owner-intern) — die 40 „logic.rs only" minus die 7 escaping-Subtree-Owner (Phase 4).

Enumeration:
```bash
cd /home/clemens/.local/src/warcraft-hotkey-editor/crates/hotkey-editor/src
comm -23 \
  <(find . -name logic.rs -printf '%h\n' | sort) \
  <(find . -name hooks.rs -printf '%h\n' | sort)
```
Escaping-Owner ausschließen: `unit_detail`, `burger_menu`, `drag_follower_ghost`, `grid_editor_tile`, `inventory_grid`, `collisions_page`, `resolve_page` (+ deren escaping-definierende Kinder).

**Rezept pro Komponente** (Template: Pilot B):
- [ ] `logic.rs` → `presentation/mod.rs`; das dort definierte `*Model`/`*Behavior`-Struct → `*Presentation`, `impl ddd::Presentation` ergänzen, `From<&…>`-Impl beibehalten.
- [ ] Hat die Komponente ein eigenes `view.rs`+`props.rs`: `props.rs` → `model/mod.rs` (`FooProps`→`FooModel`), und die Presentation ist `From<&FooModel>`. Imported-contract-Leaf (kein eigenes view/model): Regel gemäß Pilot-B-Entscheidung.
- [ ] `mod.rs`: `mod logic;`→`mod presentation;`, Import-Pfad `logic::`→`presentation::`; Body baut/platziert die Presentation.
- [ ] `rm logic.rs`.

- [ ] **Batch-Gate:** nach je ~15–20 Komponenten `moon run :ci` → PASS, Commit `sweep: pure leaves batch N to presentation/`. Bei rot: zuletzt geänderter Batch = Ursache.

---

## Phase 3: Connected-Sweep (hooks.rs → presentation/)

**Zielmenge:** die 73 Komponenten mit `hooks.rs`. `find . -name hooks.rs -printf '%h\n'`. „both" (23, mit `logic.rs`) und escaping-Owner beachten.

**Rezept pro Komponente** (Template: Pilot A):
- [ ] `props.rs` → `model/mod.rs` (`FooProps`→`FooModel`, `#[derive(Props)]`, `From<&FooView>`, `ddd::Model`), falls vorhanden; `view.rs` → `view/mod.rs`.
- [ ] `hooks.rs` → `presentation/mod.rs`: der `use_*`-Hook wird `use_foo_presentation(&model) -> FooPresentation`. Alle `use_signal`/`use_effect`/`use_memo`/Context-Reads bleiben hier; Rückgabe ist das **signalfreie** `FooPresentation` (`ddd::Presentation`).
- [ ] Signals, die heute im ad-hoc `FooModel` (Render-Struct) stecken, bleiben im Builder; die Presentation trägt den Wert + einen `on_*_change`-Handler statt des Signals. Das ad-hoc `FooModel` → `FooPresentation`.
- [ ] Falls `logic.rs` vorhanden (both-Fall): owner-internes reines Shaping ebenfalls in `presentation/` falten (escaping-Typen → `view/`, Phase-4-Regel).
- [ ] `mod.rs`: Parameter = **ein** Props-Struct `FooModel`. Body: `let p = use_foo_presentation(&props); …` platziert `p`.
- [ ] `rm hooks.rs` (+ `logic.rs` wenn gefaltet).

- [ ] **Batch-Gate + Browser:** pro Subtree/Batch `moon run :ci` → PASS **und** Browser-Verifikation des Verhaltens. Commit `sweep: connected batch N to presentation/`.

---

## Phase 4: Escaping-Type-Subtrees

**Owner:** `collisions_page` (`logic/`-dir), `resolve_page`, `unit_detail`, `burger_menu`, `drag_follower_ghost`, `grid_editor_tile`, `inventory_grid`. `grid_editor` hat ebenfalls ein `logic/`-dir.

**Regel pro Owner:**
- [ ] **Entkommende Contract-Typen** (von Siblings/Kindern importiert) → `view/` bzw. geteiltes View-Modul an der Subtree-Wurzel; sie sind Views. Liste je Owner:
  - `collisions_page`: `CollisionUnitView`, `HotkeyUnitView`, `IslandView`, `UnitPositionUnitView`, `ConflictView`, `ConflictAbilityView`, `IslandAbilityData`, `IslandUnitData`, `UnitIconView`, `HotkeyConflictView`, `UnitPositionConflictView`.
  - `resolve_page`: `MoveView`, `UnresolvedView`, `MoveSection`, `MiniGridPlacement`, `ReasonKind`.
  - `unit_detail`: `UnitCommandGridSlots`, `UnitOverrideTarget`.
  - `burger_menu`: `BurgerMenuRow`.
  - `drag_follower_ghost`: `FollowerPresentation`.
  - `grid_editor_tile`: `EditorTile`.
  - `inventory_grid`: `InventoryDragFollower`, `InventoryDragSource`.
- [ ] **Owner-interne Helfer** (`ResolvedUnit`, `EditorTileChrome`, `BurgerActionHandlers`, `MenuRowBuilder`, `RowDynamics`, …) → `presentation/` (privat).
- [ ] Die `super::super::…logic::X`-Importe der Siblings/Kinder auf den neuen `…view::X`-Pfad umbiegen (löst zugleich einen Großteil der Phase-5-Typgriffe).
- [ ] `logic.rs`/`logic/` des Owners auflösen; die 2 `logic/`-dirs (`collisions_page`, `grid_editor`) Datei für Datei aufteilen.

- [ ] **Gate pro Owner-Subtree:** `moon run :ci` → PASS; für `collisions_page`/`resolve_page`/`grid_editor` zusätzlich Browser (Collisions-, Resolve-Route, Grid-Drag). Commit `sweep: <owner> escaping contracts to view/`.

---

## Phase 5: `super::super::data` Antipattern-Fix

**Files (3 Empty-Panes):**
- `.../details/hotkey_unit_detail/components/empty_hotkey_unit_detail/mod.rs`
- `.../details/island_detail/components/empty_island_detail/mod.rs`
- `.../details/unit_position_detail/components/empty_unit_position_detail/mod.rs`

**Rezept pro Fall (identisch):**
- [ ] Kind nimmt `prompt: String` (`#[props(into)]`) als Prop; `p { {prompt} }`. `use super::super::data::EMPTY_PROMPT` entfernen.
- [ ] Eltern (`hotkey_unit_detail`/`island_detail`/`unit_position_detail`, besitzt `data.rs`) rendert `EmptyHotkeyUnitDetail { prompt: EMPTY_PROMPT }` statt propless.

- [ ] **Gate:** `moon run :ci` → PASS; Browser: leere Detail-Panes zeigen den Prompt. Commit `fix: empty-detail panes take prompt as prop`.

- [ ] **Rest-Typgriffe:** `grep -rn 'super::super' crates/hotkey-editor/src` — verbliebene Griffe auf alte `logic`-Pfade auf `view`-Pfade umbiegen. Legitime own-file/`logic/render`-Fälle + Doc-Links unangetastet lassen.

---

## Phase 6: Mechanischer Datei→Verzeichnis-Sweep

Verbleibende flache Belange in Verzeichnis-Module umziehen. **Rein mechanisch.** Reihenfolge: `style` zuerst, dann `data`, `state`, zuletzt `view` (nur die, die Phase 4 nicht schon angefasst hat).

Mechanik für einen Belang (Beispiel `style.rs`):
```bash
cd /home/clemens/.local/src/warcraft-hotkey-editor/crates/hotkey-editor/src
find . -name style.rs | while read f; do
  d=$(dirname "$f"); mkdir -p "$d/style"; git mv "$f" "$d/style/mod.rs"
done
```
- [ ] In `mod.rs` bleibt die Deklaration `mod X;` unverändert (Rust findet `X/mod.rs`).
- [ ] Interne relative Pfade in `X/mod.rs` prüfen: `super::` bleibt korrekt (Verzeichnis-Modul hat dieselbe Eltern-Ebene wie die frühere Datei).

Invarianten wahren: `style`/`model`/`presentation` bleiben `mod` (privat), nie `pub use`'d; `view` bleibt `mod view;` mit `pub use view::FooView;`.

- [ ] **Batch-Gate pro Belang:** `style` → `moon run :ci` → PASS, Commit `sweep: style.rs to style/mod.rs`. Dann `data`, `state`, `view` je einzeln gaten + committen (getrennte Commits erleichtern Bisect).

---

## Self-Review-Ergebnis (Spec-Abdeckung)

- ddd `Props`→`Model` + `Presentation`-Marker → Phase 0. ✓
- Pipeline View→Model→Presentation / `From<&View>` / Signal-out → Phase 1, 2, 3. ✓
- `props.rs`→`model/mod.rs`, Param = ein Props-Struct, nie View/inline → Phase 0/1/3. ✓
- `logic.rs`→`presentation/` + escaping→`view/` → Phase 2, 4. ✓
- `hooks.rs`→`presentation/` (effektvoller Builder) → Phase 1, 3. ✓
- `super::super::data`-Fix → Phase 5. ✓
- `state.rs` bleibt, nur Datei-Form → Phase 6. ✓
- Datei→Verzeichnis (style/view/data/state) → Phase 6. ✓
- View↔Model-Split unangetastet → Global Constraints + Phase 1 Doku. ✓

---

## Execution Handoff

**Empfehlung:** Subagent-Driven, mit **User-Gate nach Phase 1** (Pilots + COMPONENTS.md absegnen, bevor die Sweeps 500+ Komponenten anfassen). Sweep-Phasen sind rezept-getrieben: ein Subagent pro Batch/Subtree, `moon run :ci` als Gate, Review zwischen Batches.
