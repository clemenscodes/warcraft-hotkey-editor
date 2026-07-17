# Auftrag: der komplette mobile Search Dialog, insbesondere die Buttons

Dieser Auftrag liegt absichtlich als Datei im Repo und nicht im Prompt eines
Subagenten, damit der Nutzer jedes Wort lesen und korrigieren kann.

Der Nutzer hat die aktuelle UI mehrfach und in den schaerfsten Worten
abgelehnt. Woertlich: "du hast sowas von NULL PLAN von UI", "absolut
widerliche und ekelhafte UIs", "das ist zum kotzen", "keine einzige
componente ist so abnehmbar", "das verstoesst gegen so ziemlich jedes
designgesetz". Der bisherige Bearbeiter hat es dreimal verschlimmbessert.
Die Kritik ist berechtigt und der Ausgangspunkt, nicht der Ton, den es zu
relativieren gilt.

## Schritt 1, zuerst und ohne Code: die Bestandsaufnahme

Sieh dir den Ist-Zustand an, mobiler Viewport, 390px breit:

`/home/clemens/.config/claude/image-cache/d52f9eb3-586b-451e-92fe-db1e86a4d0e5/32.png`

Von oben nach unten steht dort: ein Titel "SEARCH" mit einem X zum Schliessen,
danach fuenf Rassen-Buttons (HUMAN, ORC, ELF, UNDEAD, NEUTRAL), jeder auf einer
eigenen Reihe ueber die volle Breite, jeder mit einer **anderen** Rahmenfarbe
(blau, rot, tuerkis, lila, gold), danach eine Reihe mit vier Toggles, die
"MELEE", "CAM...", "PLAI...", "ALL T..." lesen, also drei von vier
abgeschnitten, und zuletzt ein Suchfeld mit einem "UNIT" Dropdown darin.

Liefere **zuerst** eine schonungslose Kritik, nicht diplomatisch und nicht
abgemildert. Benenne jedes gebrochene Designgesetz mit dem konkreten Beleg im
Screenshot.

## Die harten Anforderungen des Nutzers

Vorgaben, keine Vorschlaege. Alle woertlich vom Nutzer.

1. "von mir aus packen wir die rassen in die erste reihe, das ist okay. aber
   es darf nichts abschneiden." Die fuenf Rassen kommen in **eine** Reihe, und
   **nichts** darf abgeschnitten werden. Aktuell ist beides verletzt.
2. "in die zweite reihe haben wir dann die toggles, melee campaign, all
   variants, no abilities." Die vier Toggles teilen sich die zweite Reihe.
3. "dann haben wir ne races row, row mit toggles, und drunter muss die suche
   rein." Also Rassen, Toggles, Suche, in dieser Reihenfolge.
4. "man sucht nicht erst und konfiguriert danach was man suchen will." Die
   Filter konfigurieren zuerst, die Suche ist der letzte Schritt.
5. "die aehnlichen Buttons Unit/Ability sind komplett verschiedene stellen"
   und "melee campaign sind grundlos einfach KOMPLETT andere buttons". Der
   Nutzer verabscheut es, wenn gleichartige Bedienelemente wie verschiedene
   Buttons aussehen. Es braucht **einen** konsistenten Button-Look.
6. Verhaeltnismaessigkeit, woertlich: "das komplette unitfilterquery struct
   hat ja theoretisch echt nur ne sehr kleine anzahl an bytes und information.
   die invarianten davon koennte man sehr effizient in relativ wenig bits
   angeben. dafuer, dass das so wenige bits information sind, wird hier schon
   wieder 50% des mobilen viewports verballert. das ist einfach kein
   verhaeltnis." Der gesamte Filterzustand sind rund elf Bit, naemlich fuenf
   Rassen, zwei Modus-Flags, zwei Katalog-Flags. Dafuer geht das halbe Telefon
   drauf.
7. Die Labels sind bewusst umbenannt, weil die alten gelogen haben.
   "Plain units" bedeutet, dass auch Units gelistet werden, die keine eigenen
   Abilities haben, sondern nur Standardkommandos wie Move und Attack.
   "All tiers" bedeutet, dass jede Stufe einzeln gelistet wird statt nur der
   staerksten, auf die Aenderungen sonst nach unten durchgemerged werden. Der
   Nutzer nannte das alte "no abilities" woertlich "komplett missverstaendlich".
   Diese Bedeutung darf nicht wieder verloren gehen.

## Randbedingungen, nicht verhandelbar

- Dioxus (Rust) mit Tailwind v4. Der mobile Viewport ist 390px breit.
- Lies **vollstaendig**, von oben bis unten: `docs/COMPONENTS.md`,
  `docs/RUST_STYLE.md`, `docs/ARCHITECTURE.md`, `docs/AGENTS.md`. Ein Hook
  namens `full-spec-gate.sh` blockiert jede Aktion, bis die Specs im Kontext
  sind, und gibt sie dabei selbst aus. Das ist der Boden, nicht das Ziel.
- **`CLAUDE.md` ist an einer Stelle veraltet, das hat der Nutzer bestaetigt.**
  Wo `CLAUDE.md` und `COMPONENTS.md` sich widersprechen, gewinnt
  `COMPONENTS.md`. Konkret: `CLAUDE.md` verlangt noch "child props via
  `From<&ParentProps>`" und Spread. `COMPONENTS.md` erklaert genau diese Regel
  fuer **geloescht** und Code danach fuer **falsch**. Es gilt: Props sind
  privat, `mod model;` wird nie re-exportiert, und ein Parent uebergibt Daten,
  indem er das Kind benennt und dessen Felder einzeln mit Domain-Werten setzt.
  Kein `..spread`, kein `impl From<&ParentProps>`, kein Props-Typ als Feld
  eines anderen Props-Typs.
- Weitere Kernregeln aus `COMPONENTS.md`: Verzeichnis gleich Component gleich
  CSS-Klasse. Reine RSX-Bodies ohne Logik. Der Render-Baum **ist** der
  Verzeichnisbaum, ein `use super::` fuer ein gerendertes Kind beweist einen
  Verstoss. Sich ausschliessende Looks sind eigene Components, niemals eine
  Zustandstabelle. Einen Look uebernimmt man ausschliesslich, indem man die
  Component komponiert, die ihn besitzt, niemals durch geteilte Styles. Eine
  Component besitzt ihren **Look**, ihr Parent besitzt ihre **Groesse**, und
  eine Component schreibt sich nie selbst Breite oder Hoehe. Sechs disjunkte
  Responsive-Baender, die nichts vererben, `BASE` traegt die gemeinsame
  Wahrheit. Kein `clamp()`. `children: Element` ist verboten.
- Die Rassenfarbe ist ausdruecklich erlaubt: `COMPONENTS.md` nennt die fuenf
  Rassen als **die** sanktionierte Ausnahme, naemlich eine Domaenen-Farbachse
  ueber eine `--race-color` Custom Property. Die Frage ist also nicht, ob die
  Farbe erlaubt ist, sondern ob fuenf verschiedene Rahmenfarben nebeneinander
  gutes Design sind. Beantworte das.
- Design-Tokens stehen in `crates/hotkey-editor/tailwind.css`. Lies sie und
  schlage ausschliesslich Tokens vor, die dort wirklich definiert sind.
  Achtung, `--color-panel-dark` und `--color-panel-toast` existieren **nicht**,
  sie sind tote Referenzen. Die echte deckende Flaeche ist
  `--color-warcraft-bg-base`. Eine Klasse in einer `style/mod.rs` beweist gar
  nichts, gegengeprueft wird im gebauten CSS unter
  `crates/hotkey-editor/assets/tailwind.css`. Vorsicht beim Grep, dort sind
  Punkte, Klammern und Prozentzeichen escaped, `w-4.5` steht als `w-4\.5`.
  Suche mit `grep -F`, sonst schliesst du faelschlich auf eine fehlende Klasse.

## Wo der Code liegt

Der Dialog:
`crates/hotkey-editor/src/components/app/components/shell/components/header/components/toolbar/components/toolbar_actions/components/shared/dialogs/search_dialog/components/search_dialog_body/`

Darin `mod.rs` und `components/search_dialog_filters/` mit den Unterbaeumen
`race_chip_row`, `race_chip` und `mode_chip_row`. Der geteilte Button liegt
unter `shell/components/editor_page/components/shared/toggle_button/`. Lies von
all diesen die `style/mod.rs`, dort steckt der Schaden.

## Drei bereits belegte Befunde

- Der `ToggleButton` traegt `flex-1`, `@container`, `uppercase`,
  `tracking-caps`, `whitespace-nowrap`, `overflow-hidden` und `text-ellipsis`.
  Grossbuchstaben mit Sperrung kosten sehr viel Breite, und `text-ellipsis` ist
  genau der Mechanismus, der die Labels abschneidet, statt das Layout ehrlich
  scheitern zu lassen.
- Der `ToggleButton` liegt unter `editor_page/components/shared/`, wird aber
  auch aus `header/` und aus `shell/components/shared/` gerendert. Der naechste
  gemeinsame Elternteil ist `shell/components/`, dort muesste er nach der
  Shared-Leaf-Regel liegen. Echter Verstoss, darf mitkorrigiert werden.
- Der bisherige Bearbeiter hat den Reihen abwechselnd `grid-cols-5` und
  `flex-wrap` gegeben und die Schrift auf `text-xs` gedrueckt, um Ueberlauf zu
  kaschieren. Beides hat es verschlimmert. Der Nutzer hatte fuer ein
  verwandtes Problem am Key-Picker bereits die Loesung diktiert: breiter machen
  und auf mehr Reihen verteilen, nicht schrumpfen.

## Was zu liefern ist

1. Ein unverbluemtes Urteil ueber den Ist-Zustand, das Schlimmste zuerst.
2. Eine nummerierte Liste jedes Designfehlers, je mit gebrochenem Gesetz und
   Beleg im Screenshot.
3. Der konkrete Vorschlag: ein ASCII-Mockup bei 390px, pro Reihe die exakten
   Tailwind-Klassen ausschliesslich mit real existierenden Tokens, und die
   Begruendung, wie er die harten Anforderungen erfuellt, also eine Rassenreihe,
   nichts abgeschnitten, Toggle-Reihe, Suche zuletzt, verhaeltnismaessig zu elf
   Bit Information.
4. Welche vorhandenen Components komponiert werden und was wirklich neu muss.

## Verbote

Kein Dev-Server, der Nutzer startet ihn selbst. Niemals eine Warte-, Sleep-
oder Polling-Schleife, das hat schon einmal sechs Stunden gekostet und ist per
Hook gesperrt. Zum Kompilieren ausschliesslich `moon run :check --force`, das
`--force` ist zwingend, weil ein lokaler Cargo-`[patch]` sonst falsche gruene
Ergebnisse liefert. Der Gate ist `moon run :ci`, niemals ein nackter `cargo`
Aufruf.
