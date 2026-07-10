# Component-Model-Konsolidierung — Design

**Datum:** 2026-07-10
**Status:** Design, freigegeben zur Planung

## Problem

Die per-Komponenten-Struktur trägt drei überlappende Konzepte, deren Grenzen
verschwommen sind:

- **`View` / `Props` / `Model`** — drei Namen für **zwei** Rollen, wobei die
  *interne* Rolle doppelt benannt ist. `ddd::View` ist der öffentliche Contract;
  `ddd::Props` ist laut ddd-Doku „a component's private internal model, built from
  its published View"; und daneben existiert ein drittes, ad-hoc benanntes
  `*Model`-Struct (das geformte Render-Struct aus hook/logic). **`Props` und
  `Model` besetzen dieselbe Rolle** (privates internes Modell) unter zwei Namen —
  *das* ist die Redundanz, die konsolidiert wird.

  **NICHT** die Redundanz: dass `View` und `Props` oft feld-identisch sind. Das
  ist **beabsichtigte Entkopplung** — eine Published-Language / API-Fläche, die
  morgen frei divergieren darf (`ddd::View`-Doku: „A View that is byte-identical
  to the Props today is not duplication — it is an API surface, free to diverge
  tomorrow"). Der `Props: From<&View>`-Split ist die Anti-Corruption-Layer und
  bleibt **unangetastet**. `view.rs` und der interne Modell-Typ bleiben zwei
  distinkte Typen. Die Komponente nimmt **immer** den internen Typ als Parameter,
  **nie** die `View` direkt.
- **`hooks.rs` vs. `logic.rs`** — der Name `logic` ist zu generisch und wirkte
  dadurch redundant. Tatsächlich ist `logic.rs` die **reine** Shaping-Ebene
  (effektfrei, beweisbar: 0 Treffer für `use_*`/`use_memo`/`use_signal`/`context`
  über alle 63 Dateien), `hooks.rs` die **effektvolle** (liest Context/Signals,
  verdrahtet Handler). Zwei legitime Ebenen, aber schlecht benannt.
- **`super::super::data`-Griffe** — Kind-Komponenten langen in den Namespace einer
  Eltern-Komponente, statt Daten als Props zu erhalten.

Zusätzlich ist die Datei-Form uneinheitlich: manche Belange sind flache `.rs`-
Dateien, die sich nicht splitten lassen, ohne die Komponente umzubauen.

Ergebnis-Ziel: **drei klar benannte formale Rollen pro Komponente
(`View → Model → Presentation`, je ein ddd-Trait), das Model ausnahmslos aus der
View baubar, mit einem einzigen benannten Presentation-Builder** — und eine
uniforme, splittbare Verzeichnis-Struktur pro Komponente.

## Grundprinzipien (die das Design tragen)

Die Präsentations-Pipeline hat **drei** compiler-sichtbare Stufen, jede ein
ddd-Trait:

```
View ──► Model ──► Presentation
ddd::View  ddd::Model   ddd::Presentation
(Contract) (empf. Props, (signalfrei, render-fertig;
            From<&View>)  der Body platziert es)
```

1. **`Model: From<&View>` ist ausnahmsloses Gesetz.** Jedes `FooModel` wird aus
   einer `FooView` gebaut. Es gibt kein Modell ohne View.
2. **`Presentation` ist die signalfreie, render-fertige Stufe, die der Body
   platziert.** Sie wird aus dem `Model` gebaut — **rein** (`From<&FooModel>`) bei
   pure-shaping Leaves, oder **effektvoll** (Signals/Context) bei connected. Ist
   das `Model` bereits render-fertig (kein Shaping, kein Effekt), entfällt die
   `Presentation` und der Body platziert das `Model` direkt.
3. **Signals leben ausschließlich im Presentation-Builder.** `use_signal`/
   `use_effect`/`use_memo`/Context-Reads passieren nur in `presentation/` (im
   effektvollen Builder). `View`, `Model` **und** `Presentation` tragen
   ausschließlich **Werte + Handler-Callbacks (`EventHandler`)**, niemals
   `Signal<T>`. (Der heutige `KeyPickerModel { open: Signal<bool>, … }` ist genau
   das Anti-Pattern, das dieses Design begradigt: der `Signal` bleibt im
   Presentation-Builder, die Presentation trägt `open: bool` + `on_open_change:
   EventHandler`.)
4. **Zwei Ebenen von „Hook", klar getrennt:**
   - **Generische Accessor-Hooks** (`use_custom_keys_service`, `use_grid_layout`,
     `use_toast`) — die *traditionelle* Hook-Rolle: geben Live-Daten + Context
     zurück, komponenten-agnostisch, wiederverwendbar. Bleiben in
     `services/<domain>/context.rs`. Unverändert.
   - **Der per-Komponenten-Presentation-Builder** (`presentation/`) — kein
     traditioneller Hook; er komponiert die generischen Hooks, besitzt lokale
     Signals, verdrahtet Handler und **baut daraus die `FooPresentation`**.
5. **Die Komponente nimmt immer das `Model` als Parameter, nie die View.**
   Der `#[component] fn Foo(props: FooModel)`-Parameter ist **ein einziger
   Props-Struct** (`#[derive(dioxus::Props)]` + `ddd::Model` + `From<&FooView>`) —
   **nie** die bare `FooView`, **nie** inline-Felder. Ein Aufrufer, der eine
   `FooView` hält, konvertiert an der Grenze via `From<&View>`. Der `View`-Contract
   bleibt ein separater, distinkter Typ.

## Ziel-Anatomie pro Komponente

```
foo/
  mod.rs              # #[component] fn Foo(props: FooModel) — reines RSX. Nimmt IMMER
                      #   EINEN Props-Struct FooModel, NIE die View, NIE inline-Felder.
                      #   Baut via presentation/ die FooPresentation, platziert deren
                      #   Felder. Keine Logik. (Ist das Model render-fertig, platziert
                      #   der Body das Model direkt — keine presentation/.)
  view/mod.rs         # FooView — ddd::View, öffentlicher Contract.
                      #   Werte + Handler-Callbacks, NIE Signals. Splittbar.
  model/mod.rs        # FooModel — ddd::Model (ex-ddd::Props), empfangener Props-Struct,
                      #   From<&FooView>, #[derive(Props)]. Privat, nie re-exportiert.
                      #   Splittbar.
  presentation/mod.rs # optional — FooPresentation (ddd::Presentation) UND ihr Builder.
                      #   Pure leaf: impl From<&FooModel>. Connected: use_foo_presentation
                      #   (besitzt Signals, liest Context, verdrahtet Handler). Absorbiert
                      #   owner-internes ex-logic.rs-Shaping. Splittbar.
  style/mod.rs        # classes!-Consts. Splittbar.
  data/mod.rs         # optional — statischer Content. Splittbar.
  state/mod.rs        # optional — Diskriminanten-Enums + verbliebener Runtime-UI-State.
  components/          # Kind-Komponenten (unverändert).
```

- **Pure passthrough Leaf** (Model render-fertig): keine `presentation/`. `mod.rs`
  platziert die `FooModel`-Felder direkt.
- **Pure-shaping Leaf** (heutige `logic.rs`-only): `presentation/mod.rs` mit
  `impl From<&FooModel> for FooPresentation`; **kein** effektvoller Builder.
- **Connected** (heutige `hooks.rs`): `presentation/mod.rs` mit `use_foo_presentation`
  (Signals/Context) → `FooPresentation`. `mod.rs` ruft den Builder, platziert.

## ddd-Änderung: `Props` → `Model` + neuer `Presentation`-Trait

Im Sibling-Repo `~/.local/src/ddd/crates/ddd` (per `[patch]` gepatcht):

Die drei Präsentations-Traits, jeder mit echtem, compiler-geprüftem Bound:

```rust
pub trait View: Clone + PartialEq {}

pub trait Model: for<'view> From<&'view Self::View> + Clone + PartialEq {
    type View: View;
}

pub trait Presentation {
    type Model: Model;
}
```

- Trait **`Props` → `Model`** umbenennen; `From<&View>`-Bound bleibt, **+ `Clone +
  PartialEq`** (die faktische Dioxus-Props-Anforderung wird so erzwungen statt nur
  übers `#[derive]` implizit vorhanden).
- Doku anpassen („a component's private internal model, built from its published
  View").
- **`View`** bekommt **`Clone + PartialEq`** (Consumer klonen die View in der
  `From<&View>`-Konversion; Dioxus vergleicht sie).
- **Neuer Trait `Presentation`** mit assoziiertem Typ `type Model: Model` — er
  verdrahtet die Pipeline `View → Model → Presentation` auf Typ-Ebene (die Kette
  `Self::Model::View` ist navigierbar), **ohne** einen `From`-Supertrait: die
  Presentation wird bei pure-shaping Leaves via `From<&Model>`, bei connected
  effektvoll (Hooks) gebaut — ein universeller `From`-Bound ginge nicht (der
  effektvolle Bau ist keine reine Konversion; Hooks sind keine Traits). Kein
  `Clone/PartialEq` auf `Presentation` (sie ist kein Dioxus-Props, wird nur
  destrukturiert).
- **Kopplung:** `type Model: Model` verlangt, dass **jede** Komponente mit
  `Presentation` ein `Model` hat. Für imported-contract-Leaves (z.B.
  `race_tab_state`, das `RaceTabBinding` empfängt und heute kein eigenes Model hat)
  heißt das: der importierte Parent-Contract muss die Model-Rolle einnehmen (oder
  das Leaf bekommt ein eigenes Model). Diese Wahl wird am Pilot B festgezurrt.
- Betrifft die ddd-Version/den Tag; da lokal gepatcht. `View`/`Model`/`Presentation`
  haben keine Test-Abdeckung — kein Test bricht.

`Model` behält den `From<&View>`-Bound (nicht reiner Marker): das ist möglich,
**weil** Grundprinzip 3 (Signals nur im Presentation-Builder) dafür sorgt, dass jedes
Modell tatsächlich eine reine Projektion seiner View ist. Der Bound erzwingt
compiler-seitig „kein Modell ohne View".

## `props.rs` → `model/mod.rs` (ein interner Modell-Name, View bleibt separat)

`props.rs` **löst sich NICHT in `view.rs` auf** — der View↔Props-Split ist
Entkopplung und bleibt. Was konsolidiert wird, ist die **doppelte Benennung der
internen Rolle**: heute trägt eine Komponente sowohl `FooProps` (der
Dioxus-Parameter, `From<&FooView>`) als auch ein separates ad-hoc `FooModel` (das
geformte Render-Struct aus hook/logic). Das sind zwei Namen für dieselbe interne
Rolle.

Nach der Umstellung gibt es **ein** internes Modell pro Komponente:

- `props.rs` → `model/mod.rs`; Typ `FooProps` → `FooModel`. Es behält
  `#[derive(dioxus::Props)]` (Dioxus braucht das für den Parameter), implementiert
  `ddd::Model` und `From<&FooView>`. Das ist der empfangene Props-Struct.
- Das bisher separate ad-hoc `FooModel` (Render-Struct) wird die
  **`FooPresentation`** (`presentation/`, `ddd::Presentation`) — die signalfreie,
  render-fertige Stufe. Es verschmilzt **nicht** mit dem Props-`Model`; die beiden
  waren nie dieselbe Rolle (empfangene Props vs. geformte Render-Daten), nur
  gleichnamig. Bei der heutigen `KeyPicker`-Kollision (`KeyPickerProps` **und**
  `KeyPickerModel`) wird `KeyPickerProps` → `KeyPickerModel` (Props) und das alte
  `KeyPickerModel` → `KeyPickerPresentation`.
- **Die Komponente nimmt dieses `FooModel` als Parameter** —
  `#[component] fn Foo(props: FooModel)` —, **nie** die `FooView`, **nie**
  inline-Felder. `view.rs` bleibt der separate, distinkte öffentliche Contract.

Ziel: drei klar benannte formale Rollen pro Komponente (`FooView` / `FooModel` /
`FooPresentation`) statt der dreier vager Namen (View / Props / ad-hoc Model), bei
unverändertem View↔Modell-Split.

## `logic.rs` → `presentation/` — mit sauberem Schnitt für entkommende Typen

`logic.rs` ist reines Shaping — es baut das Render-Struct aus dem Model. Das ist
genau der **reine Presentation-Builder** und faltet nach `presentation/` (als
`impl From<&FooModel> for FooPresentation`). **Aber:** ~18–23 distinkte
`logic`-Typen über 7 Owner **entkommen** heute ihrer Komponente (Siblings/Kinder
importieren sie), und `presentation/` ist wie `model/` per Invariante **privat,
nie re-exportiert**. Alle entkommenden Typen sind **View-/Daten-Contracts** (viele
heißen `*View`).

Fold-Regel:

- **Owner-interne Shaping-Helfer** (nur vom eigenen `mod.rs`/`presentation` genutzt,
  z.B. `ResolvedUnit`, `EditorTileChrome`, `BurgerActionHandlers`,
  `MenuRowBuilder`, `RowDynamics`) → **`presentation/`** (privat). Faltet sauber.
- **Entkommende Contract-Typen** → **`view/`** bzw. ein geteiltes View-Modul an
  der Subtree-Wurzel. Sie *sind* Views; das ist konsistent, keine Ausnahme.
  Betroffene Owner + Typen (nicht erschöpfend, aus der Bestandsaufnahme):
  - `collisions_page` (bereits `logic/`-Verzeichnis): `CollisionUnitView`,
    `HotkeyUnitView`, `IslandView`, `UnitPositionUnitView`, `ConflictView`,
    `ConflictAbilityView`, `IslandAbilityData`, `IslandUnitData`, `UnitIconView`,
    `HotkeyConflictView`, `UnitPositionConflictView`.
  - `resolve_page`: `MoveView`, `UnresolvedView`, `MoveSection`,
    `MiniGridPlacement`, `ReasonKind`.
  - `unit_detail`: `UnitCommandGridSlots`, `UnitOverrideTarget`.
  - `burger_menu`: `BurgerMenuRow`.
  - `drag_follower_ghost`: `FollowerPresentation`.
  - `grid_editor_tile`: `EditorTile`.
  - `inventory_grid`: `InventoryDragFollower`, `InventoryDragSource`.
- **Zwei Owner nutzen bereits ein `logic/`-Verzeichnis** (`collisions_page`,
  `grid_editor`) statt `logic.rs` — Fold entsprechend behandeln (Verzeichnis-
  Inhalt aufteilen in `model/` + `view/`, nicht als Einzeldatei).

## `super::super::data` — Antipattern-Fix

- **3 echte Antipattern-Fälle** (Kind grabbt Wert aus `data.rs` des Elternteils,
  wird propless gerendert): `empty_hotkey_unit_detail`, `empty_island_detail`,
  `empty_unit_position_detail`. Jeweils `EMPTY_PROMPT` als **Prop** vom Eltern
  (der `data.rs` besitzt) durchreichen; `use super::super::data::EMPTY_PROMPT` im
  Kind entfernen.
- **~40 Typ-Hochgriffe** (Wert fließt korrekt als Prop, aber der Typ ist in einer
  Eltern-Komponente definiert): entfallen weitgehend automatisch, sobald die
  entkommenden Contract-Typen (oben) nach `view/` bzw. geteilte View-Module
  wandern. Rest-Griffe im gleichen Zug auf den neuen View-Ort zeigen lassen.

## `state.rs` bleibt (→ `state/mod.rs`)

Alle 24 `state.rs` sind live und importiert; **keine Leiche.** Zwei legitime
Sorten, beide bleiben:

- **Runtime-UI-State / Daten-Contracts** (~10): `OverrideEditTarget`,
  `SystemHotkeysDialogState`, `KeyPickerCell`, `UnitDetailModel`, … — „UI-State ≠
  Domain-State", muss bleiben.
- **Diskriminanten-Enums** (~14), auf die der Parent matcht, um die per-State-
  Komponente zu rendern (`GridTileState`, `SurfaceState`, `SystemSlotState`, …) —
  der nötige Begleiter des per-State-Component-Patterns, nicht dessen Überrest.

Nur die Datei-Form wird uniformiert: `state.rs` → `state/mod.rs`.

## Datei→Verzeichnis-Sweep (uniforme, splittbare Struktur)

Jeder Belang wird ein Verzeichnis-Modul (`X.rs` → `X/mod.rs`), sodass er später
ohne Komponenten-Umbau in Sub-Dateien splitten kann. Die neuen Pfade liegen als
Geschwister neben `components/` in jedem Komponenten-Ordner (parallel, nicht
tiefer verschachtelt).

- `style.rs` (509) → `style/mod.rs`. Alle flach heute, invariant-sauber
  (immer `mod style;` privat, 0 `pub use style::`). Uniform.
- `hooks.rs` (73) — geht als **effektvoller Presentation-Builder** in
  `presentation/mod.rs` auf, nicht in ein `hooks/`. Es gibt nach dem Design
  **keine** per-Komponenten-traditionellen Hooks mehr, daher wäre ein `hooks/`-Dach
  irreführend.
- `logic.rs` (63) + `logic/` (2 Verzeichnisse) → nach `presentation/` (owner-interner
  reiner Presentation-Builder) und `view/` (entkommende Contracts) aufgeteilt, s.o.
- `view.rs` (491) → `view/mod.rs`.
- `data.rs` (34) → `data/mod.rs`.
- `state.rs` (24) → `state/mod.rs`.
- `props.rs` (494) → `model/mod.rs` (`FooProps` → `FooModel`), s.o.

**Invarianten, die die Umstellung wahren muss:**
- `style`, `model` (ex-props) und `presentation` bleiben **privat**, nie re-exportiert.
- `view` bleibt `mod view;` (privat) mit re-exportiertem Typ (`pub use
  view::FooView;`) — das etablierte Published-Language-Muster.
- Die vorhandenen `pub mod`-Ausnahmen (8) und `pub use view::` (≈491) bleiben
  intakt.

## Umfang & Reihenfolge (Grobstufen — Detailplan folgt via writing-plans)

Dieser Refactor ist groß (509 Komponenten) und **vor** dem Import-Order-Tool
angesiedelt. Grobe Stufenfolge:

1. **ddd:** `Props` → `Model` umbenennen + `Presentation`-Marker ergänzen
   (Sibling-Repo), lokal patchen, App kompiliert wieder.
2. **props→model:** `props.rs` → `model/mod.rs`, `FooProps` → `FooModel`
   (behält `#[derive(Props)]`, wird `ddd::Model`). Komponente nimmt einen
   Props-Struct `FooModel`, nie die View, nie inline-Felder. View↔Modell-Split
   bleibt.
3. **Presentation-Builder:** `hooks.rs` → `presentation/mod.rs` (effektvoller
   Builder, Signals hierher); das ad-hoc Render-`FooModel` → `FooPresentation`
   (`ddd::Presentation`).
4. **logic-Fold:** `logic.rs`/`logic/` → `presentation/` (owner-interner reiner
   `From<&Model>`-Builder) + `view/` (entkommende Contracts). Die 2
   `logic/`-Verzeichnis-Owner separat.
5. **super::super::data-Fix:** 3 Antipattern-Prompts als Props; Rest-Typgriffe auf
   neue View-Orte.
6. **Datei→Verzeichnis-Sweep:** `style.rs`/`view.rs`/`data.rs`/`state.rs` →
   `*/mod.rs`.

Jede Stufe hält `moon run :ci` grün, bevor die nächste beginnt.

## Erfolgskriterien

- Drei klar benannte formale Rollen pro Komponente: `FooView` (`ddd::View`),
  `FooModel` (`ddd::Model`, `From<&FooView>`, der Props-Parameter),
  `FooPresentation` (`ddd::Presentation`, render-fertig) — statt View / Props /
  ad-hoc Model.
- Die Komponente nimmt **einen Props-Struct** `FooModel` als Parameter, **nie** die
  `FooView`, **nie** inline-Felder. `view.rs` bleibt ein separater, distinkter
  Contract (View↔Modell-Split unangetastet — Entkopplung, keine getilgte Redundanz).
- **Kein `Signal<T>`** in `view/`, `model/` oder `presentation/`; Signals nur im
  effektvollen Presentation-Builder in `presentation/`.
- ddd exportiert `Model` (nicht `Props`) + `Presentation`; jedes interne Modell ist
  `ddd::Model`, jedes Render-Struct `ddd::Presentation`.
- `logic.rs`/`hooks.rs` existieren nicht mehr; Presentation-Bau (rein oder effektvoll)
  in `presentation/`, geteilte Contracts in `view/`.
- Keine `super::super::data`-Wert-Griffe mehr; die 3 Empty-Panes nehmen `prompt`
  als Prop.
- Jeder Belang ist ein Verzeichnis-Modul (`style/`, `view/`, `model/`,
  `presentation/`, `data/`, `state/`), splittbar ohne Komponenten-Umbau.
- `state.rs`-Typen unverändert erhalten (nur Datei-Form), keine gelöscht.
- `moon run :ci` grün; App im Browser funktional verifiziert (UI-Änderung an
  connected Components wie `KeyPicker`).

## Bewusst NICHT in diesem Refactor

- **Diskriminanten-Enums zu Parent-Wahl auflösen** (die 14 „Kind B"-`state.rs`) —
  wäre ein separates Design mit Call-Site-Rewrites, kein Teil hiervon.
- **Import-Order-Tool** — eigene Spec
  (`2026-07-10-import-order-tool-design.md`), kommt **nach** dieser
  Konsolidierung.
