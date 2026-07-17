# PRODUCT.md

> Dieses Dokument ist Pflichtlektüre für jeden Agenten, der in diesem Repo arbeitet,
> gleichrangig neben `ARCHITECTURE.md`, `COMPONENTS.md`, `RUST_STYLE.md` und `AGENTS.md`.
> Jene vier sagen, WIE Code auszusehen hat. Dieses sagt, WOFÜR er existiert und FÜR WEN.
> Diese Lücke hat real dazu geführt, dass regelkonform am Produkt vorbeigebaut wurde.
>
> Es gilt `docs/PRODUCT_RESEARCH_RULES.md`. Jede Aussage trägt ihre Quellenklasse.
> Vier Klassen existieren, `blizzard-offiziell`, `community`, `repo-code` und
> `auftraggeber`. Wo nichts belegt ist, steht das ausdrücklich da. Wer dieses
> Dokument erweitert, hält sich an dieselben Regeln, insbesondere an das Verbot der
> Quellenwäsche. Ein Forum, das Blizzard zitiert, bleibt `community`.

---

## 1. Was das hier ist

Dieses Produkt ist ein reiner Frontend-Editor in Rust, Dioxus und WebAssembly für die
Datei `CustomKeys.txt` von Warcraft III Reforged. Kein Server, keine Datenbank, kein
Konto, kein Cloud-Speicher. Es wurde in direkter Kooperation mit **Back2Warcraft**
entwickelt, und darum steckt darin echtes Domänen-Expertenwissen, das man sich nicht
ausdenken kann. Das ist die Legitimation dieses Produkts, und es ist der Grund, warum
Entscheidungen, die von außen willkürlich aussehen, meistens keine sind. Wer hier etwas
umbaut, weil es "logischer" wäre, baut mit hoher Wahrscheinlichkeit Fachwissen weg.

**Quellenlage zu diesem Absatz, ehrlich.** Die Kooperation mit Back2Warcraft ist Klasse
`auftraggeber` und **nicht extern belegt**. Belegt ist nur der Kontext drumherum.
Back2Warcraft ist ein englischsprachiges Caster-Team, gegründet am 09.01.2011, mit Sitz
in Deutschland, laut `created=2011-01-09` und `location=Germany` im Wikitext von
[Liquipedia](https://liquipedia.net/warcraft/Back2Warcraft) (Klasse `community`). Und es
gibt einen Präzedenzfall dafür, dass B2W fremde Community-Projekte nicht nur erwähnt,
sondern materiell unterstützt. PCGamesN schreibt wörtlich, "a broadcaster team called
Back2Warcraft has been following the project closely since the beginning, helping spread
the word around the community and sponsoring the Flo servers as well", siehe
[PCGamesN](https://www.pcgamesn.com/warcraft-3-reforged/multiplayer-community-w3champions)
(Diego Arguello, aktualisiert 2023-01-17, Klasse `community`, journalistisch). Das belegt
B2Ws Verhalten gegenüber W3Champions. Es belegt **nicht** die Kooperation mit diesem
Produkt. Ein Indiz, aber kein Beleg, ist der Dateiname `CustomKeys_Neo_QWERTY.txt` im
Template-Katalog.

---

## 2. Das Spiel und die Domäne, so weit nötig

`CLAUDE.md` deckt das größtenteils ab, deshalb hier nur das, was für den Rest des
Dokuments gebraucht wird.

Jede Unit und jedes Gebäude in Warcraft III hat eine Command Card, ein festes 4x3 Raster
aus 12 Buttons. Jeder Button kann auf eine Taste gelegt werden. `CustomKeys.txt` ist die
Datei, die das Spiel dafür liest, adressiert pro Objekt über einen vierstelligen
Rawcode.

Das Format kennt drei Hotkey-Felder und drei Positionsfelder. Die
Community-Dokumentation nennt sie wörtlich, "The following button position entries can be
customized. - Buttonpos - Unbuttonpos - Researchbuttonpos", und definiert die Semantik,
"The position is define by an x,y where x=0 is the leftmost button column, x=3 is the
rightmost button column, y=0 is the topmost button row, and y=2 is the bottommost button
row", siehe
[CustomKeyInfo.txt](https://raw.githubusercontent.com/tlo9/Warcraft-3-Keybinds/master/CustomKeyInfo.txt).

> **ACHTUNG, HERKUNFT UNGEKLÄRT.** Diese Datei liegt im privaten GitHub-Repo einer
> Einzelperson und wurde dort vor Jahren hochgeladen. Nichts belegt, dass sie von Blizzard
> stammt, und der Auftraggeber, der sie gelesen hat, sagt ausdrücklich, dass nichts daran
> nach Blizzard klingt. Die Klasse ist `community`, und selbst das ist großzügig, denn es
> ist nicht einmal eine Aussage der Community, sondern eine Datei unbekannter Herkunft.
> **Sie belegt nur, dass dieser Text existiert, nicht dass er richtig ist, und schon gar
> nicht, dass Blizzard ihn geschrieben hat.** Eine frühere Fassung nannte sie hier
> mutmaßlich Blizzards Datei. Genau diese Formulierung ist der Fehler, an dem der erste
> Recherche-Versuch gestorben ist. Sie pflanzt die Autorität, die sie zu bestreiten
> vorgibt, und ein Leser nimmt die Prosa mit, nicht das Klassenetikett.

Die Feldstruktur ist im Datenbestand exakt nachweisbar. `Abolish Magic` trägt `Hotkey`,
`Unhotkey` und beide Positionen, und `Attribute Bonus` trägt getrenntes `Buttonpos` und
`Researchbuttonpos`. Ein Heldenskill hat also **zwei unabhängige Rasterplätze**, einen im
Lernmenü und einen auf der Command Card. Nachgelesen in
`crates/warcraft-keybinds/fixtures/resolved_default_customkeys.txt` von `warcraft-data`,
Zeilen 7 bis 11 (`[Aadm]`, `Hotkey=B`, `Unhotkey=B`, `Buttonpos=0,2`, `Unbuttonpos=0,2`)
und Zeilen 38 bis 41 (`[Aamk]`, `Buttonpos=1,1`, `Researchhotkey=F`,
`Researchbuttonpos=3,1`).

> **ACHTUNG, PFADE DER DOMÄNEN-CRATE.** `warcraft-keybinds` liegt **nicht in diesem Repo**.
> `Cargo.toml` pinnt es als git-Abhängigkeit auf `github.com/clemenscodes/warcraft-data`,
> Tag `v0.8.0` (Zeilen 24 und 25). Der Arbeitsbaum trägt zusätzlich einen `[patch]`-Block
> auf eine lokale Arbeitskopie, im `Cargo.toml` selbst markiert als "DEVELOPMENT ONLY" und
> "MUST BE DELETED BEFORE THIS WORK IS COMMITTED" (Zeilen 27 bis 34). Alle Zeilenangaben zu
> `warcraft-keybinds` in diesem Dokument beziehen sich auf den gepinnten Stand `v0.8.0`.
> Ein lokaler `/home/...`-Pfad ist für keinen Leser auflösbar und gehört nicht in dieses
> Dokument.

Grid-Layouts sind der Kern der Umbelegungspraxis. Die 12 Rasterpositionen werden auf einen
festen Tastaturblock gelegt, damit die Taste der **Position** entspricht statt einem
Buchstaben, den man sich merken muss.

---

## 3. Warum das Produkt existiert

Alles in diesem Abschnitt ist belegt. Es ist die Begründung des Produkts, und es ist die
Begründung dafür, warum der Kern dort liegt, wo er liegt.

### 3.1 Das Ausgangsproblem ist Ergonomie, nicht Bequemlichkeit

Die Szene begründet Custom Hotkeys ausdrücklich mit APM und Mausweg. Wörtlich, "The more
you move the mouse to click those Command icons, the less your focusing on your Unit Micro.
This guide will help improve increase your APM (Actions Per Minute) accuracy and efficiency
with Customkeys.txt and AHK. This is NOT considered cheating or hacking", siehe
[Liquipedia Custom Hotkeys Guide](https://liquipedia.net/warcraft/Custom_Hotkeys_Guide)
(WTVR @ USEast, zuletzt bearbeitet 2017-05-28, Klasse `community`). Derselbe Guide beziffert
das Problem, "There are over 50 Hotkeys spread across the keyboard in WC3, and with the
Default Hotkeys Setup each race is different".

Eine zweite, unabhängige Stimme bestätigt das mit einem konkreten Beispiel, "the standard
warcraft 3 hotkeys are spread all over the keyboard, for example, with a human worker
'build' is on b and 'lumber mill' is on l", und "The Warcraft 3 default key settings are
randomly spread over the keyboard and different for each race", siehe
[gaming-tools](https://gaming-tools.com/warcraft-3/qwer-customkeys/) (Klasse `community`,
kein Autor und kein Datum auf der Seite, deshalb nur als zweite Stimme zitierbar, nie als
Alleinbeleg).

### 3.2 Kollisionen sind in der Datei nicht lokal auflösbar

Das ist der Punkt, an dem Handarbeit kippt. Fähigkeiten werden von mehreren Units geteilt,
also macht eine Umbelegung an einer Unit eine andere kaputt. Wörtlich, "Kobold Geomancer has
unsolvable collision with his Slow, which is assigned Q as shortcut and his Abolish magic,
which is also assigned to Q. This cant be fixed, because other units use the same ability",
siehe [Hive Workshop](https://www.hiveworkshop.com/threads/customkeys-txt.253829/) (edo494,
2014-06-23, Klasse `community`).

Das ist **keine Einzelmeinung**. Dieselbe Mechanik steht unabhängig in der
Format-Dokumentation, "Also note that many abilities are used with multiple units, so while
a key conflict may not exist with one unit, it may on another", siehe
[CustomKeyInfo.txt](https://raw.githubusercontent.com/tlo9/Warcraft-3-Keybinds/master/CustomKeyInfo.txt)
(Klasse `community`, siehe Warnung oben). Und sie ist im Regressionstest dieses Repos
abgesichert, siehe Abschnitt 3.6.

### 3.3 Die erzwungene Umbelegung kaskadiert, und zwar von Hand

Wer eine geteilte Fähigkeit nicht umbelegen kann, muss eine andere auf eine andere Position
zwingen. Genau dieser Kaskadeneffekt wurde von Hand gerechnet. Wörtlich, "Satyr Soulstealer
has spells that collide(Mana burn is originally at Q position, but his Raise dead is supposed
to be there too with Q shortcut). The Raise dead ability cannot have its shortcut reassigned,
because it is used by other neutrals, like nerubian webspinner with Q position. The solution
for this I used was to force Mana burn to be at W position and it now has W shortcut", siehe
[Hive Workshop](https://www.hiveworkshop.com/threads/customkeys-txt.253829/) (edo494,
2014-06-23, Klasse `community`). Das ist der Kaskadenbegriff der Domänen-Crate, aus der
Praxis von 2014 hergeleitet.

### 3.4 Das Format warnt selbst davor, Positionen anzufassen

Der stärkste Einzelbeleg dafür, warum Buttons ohne Werkzeug zu verschieben gefährlich ist,
steht in der Format-Dokumentation selbst, Zeile 55, wörtlich, "Note that if more than one
button is placed at the same location on a single unit that Warcraft III will attempt to
reposition one of the conflicting buttons, but the priority for repositioning is undefined.
Also not that if Warcraft III is unable to reposition a button that button will not appear in
the command panel, rendering that ability unusable. Also note that many abilities are used
with multiple units, so while a position conflict may not exist with one unit, it may on
another. **Change button positions at your own risk.**", siehe
[CustomKeyInfo.txt](https://raw.githubusercontent.com/tlo9/Warcraft-3-Keybinds/master/CustomKeyInfo.txt)
(Klasse `community`).

Analog für Tasten, wörtlich, "Note that if the same key is assigned to multiple actions on a
single unit, while Warcraft III will still run fine, the result of using that key is not
defined, and only one of the actions will work", ebenda. Eine Kollision ist also nicht
unschön, sie ist undefiniert. Das rechtfertigt die Kollisionsprüfung als grüne Baseline.

### 3.5 Die Standarddatei sagt nicht, wo etwas anfängt

Wer Positionen verschieben will, braucht einen Ausgangszustand, und den liefert das Spiel
nicht mit. Wörtlich, "if you look in your default customkeys.txt file, you will not see the
line 'Buttonpos = ' anywhere. But you can clearly see them in the examples of the link you
provided. This means the default custom keys file is missing the button position information,
which makes it difficult to keep track of moving things around if you don't know where
everything starts", siehe
[us.forums.blizzard.com](https://us.forums.blizzard.com/en/warcraft3/t/buttonpos-isnt-in-default-customkeystxt/17532)
(normalice-1189, 2020-02-01, Klasse `community`, `staff_posts=0` im Thread verifiziert, also
kein Blue Post trotz Blizzard-Domain).

Auch das ist **keine Einzelmeinung**. Vier Jahre später und auf einer anderen Domain derselbe
Befund, "Your positions in the newly generated file are missing, too, i.e. Buttonpos=0,0",
siehe [eu.forums.blizzard.com](https://eu.forums.blizzard.com/en/warcraft3/t/20-hotkeys-feedback-bug-report-missing-shop-items-etc/1069)
(ffFiend-2900, 2024-11-14, Klasse `community`). Genau das begründet die Materialisierungs-
Boot-Strategie dieses Produkts.

### 3.6 Die Rawcodes zu finden ist ein über ein Jahrzehnt ungelöstes Problem

2008 gefragt, "I know how to use CustomKeys.txt, however I've had trouble finding out how to
get the AbilityID's for the button I want to switch" (Kamikazzee, 2008-03-10). 2021
unbeantwortet wiederholt, "I am also having this problem, did you find any solution?"
(dtquangcr, 2021-09-11). Beides siehe
[Hive Workshop](https://www.hiveworkshop.com/threads/how-can-you-find-the-abilityids-for-customkeys-txt.63394/)
(Klasse `community`). Zwei voneinander unabhängige Nutzer, 13 Jahre auseinander, das ist ein
belegt persistenter Schmerzpunkt.

Die Community musste sich dafür ein eigenes Debug-Werkzeug bauen, präparierte CustomKeys mit
allen A-Codes laden, Spiel starten, den Code dort ablesen, wo sonst der Name der Fähigkeit
steht. Wörtlich, "Instead of ability name you will see a skill code", und der Autor merkt an,
dass dieser Thread der "top-most result in Google" für die Suche nach Skill-Ids ist, ebenda
(krow7, 2022-01-21, Klasse `community`).

### 3.7 Der Ausgangszustand ist nicht konfliktfrei

Wörtlich, "there are a few units that have hotkey conflicts by default (such as Chen
Stormstout)", siehe
[jcfieldsdev README](https://raw.githubusercontent.com/jcfieldsdev/warcraft3-hotkey-editor/master/README.md)
(Klasse `community`). Das ist die Aussage eines Tool-Autors über seinen eigenen Datenbestand
und nicht von Blizzard bestätigt. Als Datenaussage belastbar, als Aussage über das Spiel nur
indikativ, und genau so ist sie zu zitieren.

### 3.8 Die Größenordnung, und zwar gemessen

Der Grund für den Aufwand ist die schiere Menge. Die aufgelöste Standard-`CustomKeys.txt`
hat 1066 Abschnitte auf 6678 Zeilen, mit 998 `Buttonpos`, 805 `Hotkey`, 96 `Unhotkey`, 159
`Researchhotkey` und 163 `Researchbuttonpos`. Nachgezählt in
`crates/warcraft-keybinds/fixtures/resolved_default_customkeys.txt` von `warcraft-data`
(gepinnt `v0.8.0`, siehe Pfadwarnung in Abschnitt 2). Wer Unit für Unit verschiebt, hat rund
tausend Objekte vor sich, nicht ein Dutzend.

Unabhängig davon gegengezählt an der ausgelieferten `default.txt` des Konkurrenten, 6089
Zeilen, 1130 Objektabschnitte, darunter 1112 `Buttonpos`, 137 `Researchbuttonpos` und 85
`Unbuttonpos`, also 1334 Rasterpositionsfelder, plus 845 `Hotkey`, 137 `Researchhotkey` und
35 `Unhotkey`, siehe
[default.txt](https://raw.githubusercontent.com/jcfieldsdev/warcraft3-hotkey-editor/master/www/hotkeys/default.txt)
(Klasse `community`).

`Unhotkey` trifft 96 und `Researchhotkey` 159 von 1066 Abschnitten. Die Deutung, dass das
selten genug ist, um es zu vergessen, und oft genug, dass es weh tut, ist eine
**Interpretation und keine Messung**.

> **Was diese Zahlen belegen und was nicht.** Sie belegen die **MENGE**. Sie belegen die
> **ZEIT** nicht. Die Auftraggeber-Angabe, dass das Erstellen eigener CustomKeys ohne Editor
> eine Prozedur über Tage oder Wochen war und selbst dann in zig Konflikten endete, ist
> Klasse `auftraggeber` und extern **nicht belegt**. Die Menge ist die einzige externe Stütze,
> die es dafür gibt.

### 3.9 Die Datei überlebt das Optionsmenü nicht

Nutzer berichten, dass das Spiel `CustomKeys.txt` überschreibt. Wörtlich, "every time I
change any setting (such as sound) it overwrites my CustomKeys.txt file which I created from
the Github hotkey editor" (Xelfire-11383, 2024-12-06). Der Community-Workaround, "if you set
your hotkey file to 'read only', warcraft3 doesnt ovverwrite them anymore. thats the best
workaround atm" (Neo-2529, 2024-12-06). Beides siehe
[us.forums.blizzard.com](https://us.forums.blizzard.com/en/warcraft3/t/201-hotkeys/34180)
(Klasse `community`, der einzige staff-Post im Thread ist ein leerer Kaivax-Eintrag von
2025-03-24 ohne Text, also kein inhaltlicher Blue Post).

Und der Workaround rettet ausgerechnet das nicht, worum es hier geht. Wörtlich, "The
'workaround' does not work for the positions of the hotkeys. This is a big deal for us that
has played with the buttons in a certain way for several years", siehe
[eu.forums.blizzard.com](https://eu.forums.blizzard.com/en/warcraft3/t/20-hotkeys-feedback-bug-report-missing-shop-items-etc/1069)
(Berfner-1408, 2024-11-15, Klasse `community`, einzelner Nutzer, aber deckungsgleich mit
syNtec und Lichkoenig).

**Das ist die externe Begründung dafür, warum dieses Produkt eine eigene, persistente Quelle
der Wahrheit im localStorage hält.** Die Datei im Spielverzeichnis ist kein verlässlicher
Speicher.

### 3.10 Der In-Game-Editor aus Patch 2.0 ersetzt das nicht

Hier ist Präzision Pflicht, denn hier liegt die einzige echte blizzard-offizielle Quelle im
ganzen Bestand, und sie sagt weniger, als man gerne hätte.

**Blizzard sagt, Klasse `blizzard-offiziell`**, wörtlich, "Added in-game hotkey customization
screen in the option menu. Players must have custom keys enabled to access the menu", siehe
[Patch Notes 2.0.0](https://news.blizzard.com/en-us/article/24167122/warcraft-iii-reforged-patch-notes-patch-2-0-0)
(`datePublished` laut JSON-LD der Seite `2024-11-13T18:00:00Z`). **Blizzard sagt dort nichts
über Button-Positionen.** Mehr steht dort nicht, und mehr darf ihm nicht entnommen werden.

**Nutzer berichten, Klasse `community`**, dass dieser Bildschirm beim Speichern die Datei neu
schreibt und alles verwirft, was er selbst nicht kennt. Wörtlich, "Your positions in the
newly generated file are missing, too, i.e. Buttonpos=0,0. (But it worked in 2.0.0 before
saving anything in the options.)", und "All shop-items missing. (healing salve, scroll of
speed etc.) I can not set shop items in the menu", und am 2024-11-21, nicht gespeichert werde
"Anything that is NOT in the current hotkey menu" (ffFiend-2900). Bestätigend im selben
Thread, "All shop-items missing hugely impacts the game" (SirWinsALot-2643, 2024-11-14), siehe
[eu.forums.blizzard.com](https://eu.forums.blizzard.com/en/warcraft3/t/20-hotkeys-feedback-bug-report-missing-shop-items-etc/1069)
(`staff_posts=0` verifiziert, **kein** Blue Post trotz Blizzard-Domain). Der Nachsatz von
ffFiend ist wichtig, der Verlust wird durch das **Speichern im Optionsmenü** ausgelöst, nicht
durch den Patch an sich.

Ein Nutzer rät ausdrücklich ab, wörtlich, "DO NOT edit any hotkeys inside Warcraft 3
options and save because that will replace CustomKeys.txt with the 2.0 patch standard that
misses many hotkeys and forces original grid button positioning", und zum Verschieben von
Buttons, "This is unfortunately not possible in neither Champions nor Warcraft 3 at the
moment. It adds the buttonpos line", siehe
[eu.forums.blizzard.com](https://eu.forums.blizzard.com/en/warcraft3/t/custom-hotkeys-is-finally-an-option-but-its-a-messfix/1210)
(syNtec-21170, 2024-11-27, Klasse `community`, `staff_posts=0`, einzelner Nutzer, deckungsgleich
mit ffFiend, Berfner und Lichkoenig).

Ein vierter Melder auf der us-Domain, wörtlich, "In my text document i changed where the
little icon tiles are located, this is not represented correctly anymore after the patch. […]
the worst thing is that the tiles cant be moved anymore", siehe
[us.forums.blizzard.com](https://us.forums.blizzard.com/en/warcraft3/t/custom-hotkeys-text-doesnt-work-properly-anymore/33212)
(Lichkoenig-2667, 2024-11-13, Klasse `community`, einzelner Melder, der einzige staff-Eintrag
im Thread ist der Auto-Close-Bot). Das bleibt hier nur stehen, weil vier unabhängige Melder
auf zwei Domains denselben Befund liefern, und es ist als **Nutzerbericht** zu formulieren,
nicht als Tatsache über das Spiel.

> **Formulierungsgrenze, verbindlich.** "Blizzard hat einen Hotkey-Bildschirm gebaut" ist
> belegt. "Blizzards Editor kann keine Buttons verschieben" ist **nicht** blizzard-belegt, das
> ist ein übereinstimmender Nutzerbefund. Schreibe niemals "Blizzard bestätigt", wo vier
> Forennutzer berichten, auch nicht auf einer Blizzard-Domain.

### 3.11 Der etablierte Rat lautet, nichts anzupassen

Das ist die vielleicht wichtigste Marktbeobachtung im ganzen Bestand. Die meistgelesenen
Anleitungen beschreiben gar kein Bauen, sondern ein Übernehmen. Fertige Datei laden, in den
`CustomKeyBindings`-Ordner legen, im Optionsmenü aktivieren. Und vom individuellen Anpassen
wird aktiv abgeraten, wörtlich, "I do not recommend doing this, it would be against the
pattern. If everything is simply based on QWER-ASDF-YXCV why would you change something?".
Ältere Tools markiert dieselbe Seite selbst als "Not recommended anymore". Und sie beziffert
ihren eigenen Ablauf, "You should be able to setup everything in under 5 minutes!", siehe
[gaming-tools](https://gaming-tools.com/warcraft-3/hotkeys/) (Klasse `community`, hoch im
Wortlaut, mittel in der Zurechnung, kein Autor und kein Datum).

> **Die 5 Minuten messen das INSTALLIEREN einer fertigen Datei, nicht das BAUEN eines
> Layouts.** Sie widersprechen der Auftraggeber-Angabe von 30 bis 60 Minuten nicht. Wer die
> beiden Zahlen gegeneinander stellt, vergleicht zwei verschiedene Tätigkeiten.

**Genau diese Lücke besetzt das Produkt.** Der etablierte Rat der Szene ist "nimm ein Preset,
passe nichts an". Dieses Werkzeug existiert für die, die anpassen wollen.

### 3.12 Grid ist kein Standard, sondern eine Familie von Geschmäckern

Wörtlich, "Most people know the Grid Align Setup but, I do not like those because it sets the
spells to ZXCV. I fixed it around so the spells are now QWER, Attack/Stop/Hide remain A/S/D and
move is Z (for Zurround!)", siehe
[Liquipedia Custom Hotkeys Guide](https://liquipedia.net/warcraft/Custom_Hotkeys_Guide) (WTVR,
2017-05-28, Klasse `community`). Das erklärt, warum ein einzelnes fertiges Template nicht
genügt, und warum der Template-Katalog dieses Produkts mehrere konkurrierende Ideen führt
statt einer richtigen.

Tastaturlayout-Varianten sind ebenfalls real. `tlo9/Warcraft-3-Keybinds` liefert Default,
QWERTY-Grid und Dvorak-Grid, plus eine community-beigesteuerte QWERTZ-Datei, siehe
[GitHub API](https://api.github.com/repos/tlo9/Warcraft-3-Keybinds) (Klasse `community`).

### 3.13 Eine harte Grenze der Datei, ehrlich benannt

Inventar-Items sind über `CustomKeys.txt` **nicht** belegbar. Die Community löst das
außerhalb der Datei mit AutoHotkey. Wörtlich, "some of the Hotkeys were ignored and left out
from Customkeys.txt so you have to use something like AutoHotkey for Inventory Items", siehe
[Liquipedia Custom Hotkeys Guide](https://liquipedia.net/warcraft/Custom_Hotkeys_Guide) (WTVR,
2017-05-28, Klasse `community`, unabhängig gestützt dadurch, dass gaming-tools für Items
ebenfalls auf AHK verweist).

**Das ist eine Grenze des Formats, kein Produktversäumnis.** Wer sie als Feature-Lücke
meldet, hat die Domäne nicht verstanden. Der System-Hotkeys-Dialog dieses Produkts führt ein
Inventar-Raster, das ist eine andere Sache als die Items im Shop.

Ebenfalls weggefallen ist der eigene Tooltip-Text. Wörtlich, "Warcraft III Reforged does not
support custom tooltip text for all commands (including hero abilities)", siehe
[jcfieldsdev help.html](https://jcfieldsdev.github.io/warcraft3-hotkey-editor/help.html)
(Klasse `community`, Aussage des Konkurrenz-Tool-Autors über Spielverhalten, nicht von
Blizzard bestätigt, kein Datum auf der Seite). Als Scope-Grenze brauchbar, Tooltips müssen
kein Produktziel sein.

### 3.14 Der Beleg aus dem eigenen Code

Dass Handarbeit an dieser Datei fehleranfällig ist, steht auch im eigenen Repo, und zwar als
eigene, behobene Regression. Wörtlich in
`crates/hotkey-editor/e2e/tests/desktop/drag-drop.spec.ts`, Zeilen 52 bis 54 (Klasse
`repo-code`, Pfad und Zeilen verifiziert),

```
// Regression: move_slot required AbilityOff(id) in slot_ids before it would
// co-move an ability's Unbuttonpos.  Regular command cards only contain
// Ability(id) slots, so Unbuttonpos was always left behind after a swap.
```

Und dann die eleganteste Konvergenz im ganzen Bestand. Derselbe Test benutzt den **Kobold
Geomancer**, Rawcode `nkog`, also exakt die Unit, die edo494 2014 auf Hive als unlösbaren
Kollisionsfall benannt hat (Abschnitt 3.2). Wörtlich, ebenda Zeilen 56 bis 59,

```
// After template + cascade on Kobold Geomancer (nkog):
//   ACdm stays at (0,2) and ACsw is cascaded to (1,2), both with co-located
//   Unbuttonpos.  Dragging (0,2) onto (1,2) swaps the pair; both Unbuttonpos
//   values must follow their respective abilities.
```

Der 2014 als unlösbar beschriebene Fall ist in diesem Repo ein grüner Test. Genau so ist das
erzählbar, ohne Übertreibung.

### 3.15 Gibt es 2026 überhaupt noch ein Publikum

Ja, und das ist der einzige harte Beleg dafür. Die w3champions-Ladder läuft im 1v1 auf über
3000 Ladder-Partien pro Tag. Vom 01. bis 16.07.2026 lag der Wert zwischen 3294 und 4046
Partien täglich, nachgerechnet über
[games-per-day](https://website-backend.w3champions.com/api/w3c-stats/games-per-day?from=2026-07-01)
(`gameMode=1`, Klasse `community`). Der 17.07. steht bei 1411 und ist ein angebrochener Tag,
er gehört nicht in die Spanne.

Die Größenordnung der Szene ist fünfstellig pro Season, nicht sechsstellig. Die
MMR-Verteilung für 1v1 Gateway Europe summiert sich in der abgeschlossenen Season 24 auf
19029 Spieler, Season 23 auf 11975, Season 22 auf 11594, die laufende Season 25 steht bei 6980,
siehe
[mmr-distribution](https://website-backend.w3champions.com/api/w3c-stats/mmr-distribution?season=24&gateWay=20&gameMode=1)
(Klasse `community`).

> **Ehrlichkeitspflicht bei dieser Zahl.** Die Summe ist eine **Eigenaggregation der
> veröffentlichten Bucket-Counts**, keine von w3champions publizierte Kennzahl, und sie ist
> auf 1v1 Europa begrenzt. Sie taugt als Erwartungsdämpfer gegen jede sechsstellige
> Marktbehauptung, nicht als Marktzahl.

Die Ladder wird laufend gepflegt und umfasst neben 1v1, 2v2, 4v4 und FFA auch Custom-Map-Ladders
wie Legion TD, Direct Strike, Castle Fight, Survival Chaos, MiniDota, Risk Europe und Warhammer,
mit Map-Dateien mit Datumsstempeln bis 2026-07-10, siehe
[active-modes](https://website-backend.w3champions.com/api/ladder/active-modes) (Klasse
`community`).

Hotkeys sind 2026 ein aktives Onboarding-Thema. Grubby führt sie als Folge 2 von 10 seines
Anfängerguides für 2026, Titel "Interface: Hotkeys and Control Groups - (#2) 10-Part Complete
Beginner's Guide for WC3 in 2026", und Back2Warcraft führt "Hotkeys & Control Groups - Warcraft
III Academy", geprüft über die
[YouTube oEmbed-API](https://www.youtube.com/oembed?url=https://www.youtube.com/watch?v=QGwAUV4bdIw&format=json)
(Klasse `community`).

> **Grenze dieser Quelle.** Belegt sind **ausschließlich Titel und Kanal**. Keine Inhalte,
> keine Abrufzahlen, kein Veröffentlichungsdatum. Zitierbar ist "Hotkeys sind Folge 2 von 10 in
> Grubbys 2026er Anfängerguide". **Nicht** zitierbar ist "die Szene onboardet aktiv".

---

## 4. Zielgruppe

Die Grundregel steht in `PRODUCT_RESEARCH_RULES.md` und ist nicht verhandelbar. **Die
Zielgruppe kennt ihr Handwerk.** Diese Leute tippen seit zwanzig Jahren Rawcodes von Hand in
Textdateien. Jede Aussage, die sie für Anfänger hält, ist falsch, auch wenn sie sich aus
einer echten Quelle herleiten lässt. Wer eine Produktlücke daraus konstruiert, dass der
Editor eine Selbstverständlichkeit nicht erklärt, hat den Fehler des ersten Versuchs
wiederholt.

Die folgenden Segmente sind aus Belegen abgeleitet, nicht erfunden. Sie sind **keine
Personas**, es sind Verhaltensmuster, für die es Quellen gibt.

### Segment A, der Layout-Bauer

**Wer.** Will kein fertiges Preset, sondern ein eigenes Layout, und zwar mit Position **und**
Taste zusammen. Das ist der Kernnutzer dieses Produkts.

**Was er will, belegt.** Ein Spieler formuliert genau das, wörtlich, "Well ok, in a rudimentary
way yes, but that's not fully what I meant. I rather meant 'advanced grid layout'. With
repositioning the buttons of abilites (heroes and non-heroes) to QWER hotkeys AND grid position.
See this program.", siehe
[us.forums.blizzard.com](https://us.forums.blizzard.com/en/warcraft3/t/how-do-i-set-up-hotkeys/28030)
(KraSumLive-2679, 2022-06-18, Klasse `community`, `staff_posts=0`, Stand vor dem In-Game-Editor
aus 2.0.0).

> **Formulierungsgrenze, verbindlich.** Das ist **ein** Spieler, der **einen** Wunsch
> formuliert. Es ist die beste externe Artikulation des Produktkerns, die existiert, aber es
> **belegt** den Produktkern nicht. Zitierbar als "ein Spieler formuliert genau das", niemals
> als "die Zielgruppe fordert".

**Was ihn blockiert.** Die Standarddatei nennt keine Ausgangspositionen (3.5). Kollisionen sind
nicht lokal auflösbar (3.2). Umbelegungen kaskadieren (3.3). Das Format warnt selbst vor
Positionsänderungen (3.4). Und es sind rund tausend Objekte (3.8).

### Segment B, der Preset-Übernehmer

**Wer.** Nimmt eine fertige Datei und ist fertig. Das ist der von der Szene empfohlene Weg.

**Was er will, belegt.** Datei laden, in den `CustomKeyBindings`-Ordner, aktivieren, "You should
be able to setup everything in under 5 minutes!", siehe
[gaming-tools](https://gaming-tools.com/warcraft-3/hotkeys/) (Klasse `community`). Und der
ausdrückliche Rat, nichts anzupassen, ebenda (3.11).

**Was ihn blockiert.** Nichts, solange das Preset passt. Er wird zu Segment A, sobald es das
nicht tut, und die Belege sagen, dass das passiert, denn Grid ist kein Standard, sondern eine
Familie konkurrierender Geschmäcker (3.12), inklusive Tastaturlayout.

**Konsequenz fürs Produkt.** Der Template-Katalog ist die Brücke von B nach A, nicht das Ziel.
Ein Template ist ein Startpunkt.

### Segment C, der Geschädigte aus der 2.0-Ära

**Wer.** Hatte eine funktionierende, handgepflegte Datei und hat sie verloren.

**Was ihn blockiert, belegt.** Das Spiel überschreibt die Datei bei jeder Optionsänderung
(3.9). Der Read-Only-Workaround rettet die Positionen nicht (3.9). Nutzer berichten, dass der
In-Game-Editor beim Speichern alles verwirft, was er nicht kennt (3.10).

**Konsequenz fürs Produkt.** Er braucht eine externe, persistente Quelle der Wahrheit und einen
verlässlichen Re-Export. Genau das ist die localStorage-Architektur.

### Segment D, der Rawcode-Sucher

**Wer.** Weiß genau, was er will, findet aber die vierstellige Adresse nicht.

**Was ihn blockiert, belegt.** 2008 gefragt, 2021 unbeantwortet wiederholt, und die Community hat
sich dafür ein In-Game-Debug-Verfahren gebaut (3.6).

**Konsequenz fürs Produkt.** Die Suche und die Rawcode-Anzeige auf der Karte sind für dieses
Segment kein Komfort, sondern die Lösung eines 13 Jahre alten Problems. Sie sind trotzdem
**nicht** der Produktkern, siehe Abschnitt 8.

### Segment E, der mobile Planer

**Wer.** Plant unterwegs weiter, im Zug, an der Bushaltestelle, im Wartezimmer.

**Quellenlage, ehrlich.** Dieses Segment ist Klasse `auftraggeber` und **extern nicht belegt**.
Es gibt keine Quelle im Bestand, die zeigt, dass Nutzer mobil editieren **wollen**. Belegt ist
ausschließlich die andere Hälfte, nämlich dass sie es beim Konkurrenten technisch **nicht
können**, siehe Abschnitt 6.2. Das ist ein Beleg für einen Differenzierer, nicht für eine
Nachfrage. Wer das Gegenteil behauptet, wäscht die Quelle.

**Konsequenz fürs Produkt.** Mobil ist Auftraggeber-Vorgabe und wird als solche behandelt, also
als echter Use Case und nicht als Zugeständnis. Es ist aber keine Rechercheerkenntnis, und es
ist unehrlich, es als eine zu verkaufen.

---

## 5. Feature-Katalog

Alle Pfade sind repo-relativ ab dem Repo-Wurzelverzeichnis
`/home/clemens/.local/src/warcraft-hotkey-editor`, Klasse `repo-code`. Pfade in
`warcraft-keybinds` beziehen sich auf den gepinnten Stand `v0.8.0`, siehe Pfadwarnung in
Abschnitt 2.

> **Stand dieses Katalogs.** Er beschreibt den Arbeitsbaum auf `feature/mobile-redesign` mit
> unbeendetem Umbau, nicht `main`.
>
> **Korrektur gegenüber der Katalog-Vorlage.** Die Viewport-Erkennung liegt in
> `crates/hotkey-editor/src/components/app/components/shell/components/editor_page/viewport.rs`,
> Zeile 8, `const MOBILE_MEDIA_QUERY: &str = "(max-width: 767.98px)";`. Sie liegt **nicht** in
> `viewport/mod.rs`, wie die Vorlage angab. Verifiziert.

### 5.1 Der Kern

Diese Features sind das Produkt. Alles andere ist Beiwerk.

| Feature | Was es für den Spieler tut | Mobil |
| --- | --- | --- |
| **Command-Card-Editor mit Drag and Drop** | Der Spieler zieht einen Button im 4x3 Raster einer Unit auf eine andere Zelle. Klick wählt aus, Doppelklick öffnet den Key-Picker, Ziehen verschiebt oder tauscht. Auf Touch startet der Zug per Langdruck, Wegziehen bricht ab, während des Zugs ist Scrollen gesperrt. | ja |
| **UnitCommandGrids** | Zeigt alle Raster der Unit nebeneinander, Command Card plus Baumenü, Uprooted-Menü und Research-Menü, falls vorhanden. Exakt derselbe Baustein auf Desktop und Mobil. | ja |
| **Grid-Editor-Varianten** | Drei Ausprägungen desselben Editors, weil das Spiel drei Positionsfelder kennt. Der Spieler merkt nur, dass jedes Menü gleich zu bedienen ist. | ja |
| **Hotkey-Override-Section** | Unter dem Raster steht der angeklickte Button, und der Spieler legt seine Taste fest. Die zweite Hälfte der Kernarbeit, erst Position, dann Taste. | ja |
| **Hotkey-Konflikterkennung beim Setzen** | Die Domäne sagt sofort, ob die Taste auf derselben Command Card schon vergeben ist, und mit welchem Button. | ja |
| **Key-Picker-Dialog** | On-Screen-Tastatur statt Tippen. Belegte, freie und konfligierende Tasten sind unterschiedlich dargestellt. | ja |
| **Move-Blocker-Warnung** | Ist die Zielzelle für den Off-State einer anderen Fähigkeit reserviert, wird der Zug abgelehnt, und ein Toast nennt den Blockierer namentlich plus den Ausweg. | ja |
| **Schalter update_hotkeys_on_move** | Bestimmt, ob beim Verschieben die Taste des Grid-Layouts mitgesetzt wird, also Grid-Denke, oder die alte Taste am Button klebt. | unbekannt |
| **Mobiler Unit-Pager** | Unter 768 px ersetzt ein vertikaler Snap-Pager die gesamte Desktop-Oberfläche, eine Unit pro Bildschirm, über alle Units aller fünf Rassen. | ja |
| **PagerCard** | Icon, Name und Rawcode der Unit, darunter die Raster und die Hotkey-Override-Zeile, pro Rasse eingefärbt. | ja |
| **Grid-Karussell mit Punkten** | Die Raster einer Unit liegen mobil als horizontales Karussell übereinander, Punkte zeigen Anzahl und Position. | ja |
| **Drag-Follower-Overlay** | Während des Ziehens hängt eine Kopie der Kachel am Zeiger, die Zielzelle ist als Ring markiert, Escape bricht ab. | ja |
| **Hotkey-Badge mit Zuständen** | Jede Kachel trägt ihren Buchstaben. Konflikt und passive Fähigkeit sehen anders aus, beides ohne Nachschlagen erkennbar. | ja |
| **Tastaturbedienung des Rasters** | Leertaste und Enter wählen die fokussierte Kachel aus, der Editor ist ohne Maus bedienbar. | nein |

**Wo der Kern lebt.**

- Drag-Mechanik und Konstanten, `crates/hotkey-editor/src/components/app/components/shell/components/editor_page/components/editor_workspace/components/race_theme/components/shared/unit_detail/components/unit_detail_body/components/unit_detail_row/components/shared/grid_editors/shared/grid_editor/presentation/drag_state.rs`, Zeile 6 `TOUCH_CANCEL_THRESHOLD_PIXELS: f64 = 12.0` und Zeile 7 `LONG_PRESS_MS: i32 = 300` (verifiziert). Tastaturbedienung in `.../grid_editor/presentation/mechanics.rs`, Zeilen 22 bis 33.
- Die drei Varianten, `.../grid_editors/` mit `command_grid_editor`, `research_grid_editor` und `alternate_form_grid_editor`.
- Geteilte Bausteine, `crates/hotkey-editor/src/components/app/components/shell/components/editor_page/components/shared/unit_command_grids/mod.rs` und `.../editor_page/components/shared/hotkey_override_section/mod.rs` (beide verifiziert).
- Konflikterkennung, `crates/hotkey-editor/src/services/customkeys/hotkey_override.rs` (`detect_conflict`, verifiziert), Domäne `CustomKeys::find_hotkey_conflict`.
- Mobiler Pager, `.../editor_page/components/mobile_editor/presentation/mod.rs`, Zeile 9 `CARD_WINDOW_BUFFER: usize = 1` (verifiziert), Sortierung über `UnitOrder::rank_of`, also Human, Orc, Nightelf, Undead, Neutral, dann Name.
- Schalter, `crates/hotkey-editor/src/persistence/editor_preferences_persistence.rs`, Schlüssel `warcraft-hotkey-editor.update-hotkeys-on-move` (verifiziert).

### 5.2 Das Beiwerk

Nützlich, teilweise unverzichtbar, aber nicht der Kern.

| Feature | Was es für den Spieler tut | Mobil |
| --- | --- | --- |
| **Undo und Redo** | Jede Änderung ist zurücknehmbar und wiederholbar, die Historie überlebt einen Reload. Ctrl/Cmd+Z, Ctrl/Cmd+Y, Ctrl/Cmd+Shift+Z, unterdrückt in Eingabefeldern. | teilweise |
| **Import einer eigenen CustomKeys.txt** | Bestehende Datei hochladen, sie wird über die Werksbasis gelegt und normalisiert, ein Toast meldet die Zählung. | teilweise |
| **Export** | Lädt das Ergebnis als Datei herunter. Der Info-Dialog nennt den exakten Zielordner und warnt, dass jeder andere Dateiname vom Spiel ignoriert wird. | teilweise |
| **Preview-Dialog** | Zeigt den fertigen Text zum Lesen und Kopieren. Wörtlich der localStorage-Inhalt, nicht neu serialisiert. | teilweise |
| **Templates-Dialog, sieben Presets** | Fertige Layouts als Startpunkt, jedes mit gerenderter Vorschau. Default, Clemens DotA-like in QWERTY, QWERTZ und AZERTY, NEO (Back2Warcraft) in QWERTY, QWERTZ und AZERTY. | teilweise |
| **Grid-Layout-Editor-Dialog** | Legt fest, welcher Buchstabe zu welcher der 12 Positionen gehört, und schreibt ihn auf jede Bindung. Liefert die grüne Baseline, nicht die Feinarbeit. | teilweise |
| **Kollisionsansicht, drei Klassen** | Listet Cross-Unit-Positionsinseln, Positionskollisionen innerhalb einer Unit und Hotkey-Kollisionen innerhalb einer Unit. Jeder Eintrag ist per URL verlinkbar. | teilweise |
| **Kollisionszähler als Badge** | Zeigt jederzeit, wie viele Konflikte offen sind, in jeder Bildschirmbreite. | ja |
| **Resolve-Page mit begründetem Kaskadenplan** | Zeigt jeden geplanten Zug einzeln mit Grund, Fight, Spill, Swap, GapPull. Ungelöste Fälle werden separat gezählt. Erst dann wendet der Spieler an. | teilweise |
| **Resolve-Vorschau ohne Anwenden** | Der Plan wird berechnet, ohne die Datei anzufassen. | teilweise |
| **Carriers-Dialog** | Klärt, welche Units eine Fähigkeit teilen, also woran ein Zug an mehreren Command Cards zieht. | unbekannt |
| **System-Hotkeys-Dialog** | Kontrollgruppen, Heldenauswahl, Inventar-Raster und Restliste. Inventar-Slots per Zeigergeste tauschbar, funktioniert auch mit dem Finger. | teilweise |
| **Suche als Sprungwerkzeug** | Volltextsuche über alle Units aller Rassen, umschaltbar zwischen Unit-Name und Fähigkeitsname, mit Rassen- und Modus-Chips. Reines Navigationsmittel. | ja |
| **Unit-Liste mit Kategorien** | Linke Spalte des Desktop-Editors, nach Kategorie gruppiert, mit Suchfeld und Trägheitsscrollen. | nein |
| **Katalog-Filter** | Sucht nach Unit oder Fähigkeit, blendet Units ohne Fähigkeiten ein, listet jede Tier-Variante einzeln. | nein |
| **Rassen-Tabs, fünf, gleichrangig** | Human, Orc, Night Elf, Undead und Neutral, jeder mit eigenem Banner und eigener Akzentfarbe. Neutral ist kein Sonderfall. | nein |
| **Modus-Tabs, Melee und Campaign** | Grenzt die Unit-Liste ein, beide gleichzeitig möglich. | nein |
| **Unit-Stats-Panel und Beschreibung** | Lebenspunkte, Mana, Angriffs- und Rüstungswerte samt Matchup-Tabelle, Helden-Attribute mit Zuwachs pro Level. | nein |
| **Burger-Menü** | Unterhalb der Desktop-Breite wandern alle zehn Aktionen in eine Schublade. Nichts geht verloren, es liegt zwei Taps tiefer. | ja |
| **Deep-Links** | Rasse, Modus, Unit und Suchbegriff als Query, dazu `/collisions?kind&entry` und `/resolve?entry`. Teilen, Bookmarken, Zurück und Vorwärts funktionieren. | ja |
| **Persistenz in localStorage** | Nichts geht beim Schließen verloren, ohne Konto und ohne Server. | ja |
| **Hilfe-Dialog mit Erstbenutzer-Öffnung** | Beim ersten Besuch geht der Leitfaden von selbst auf, danach nie wieder ungefragt. | teilweise |
| **Toast-Meldungen** | Kurze Rückmeldungen statt stiller Fehler. | ja |
| **Breadcrumbs und Tooltips** | Orientierung auf Kollisions- und Resolve-Seite, Tooltips mit sechs Platzierungen. | ja |
| **Normalisierung und Werksbasis beim Start** | Beim ersten Öffnen ist die Datei die vollständige Standardbelegung, mit aufgelösten Kaskaden und materialisierten Positionen. Der Spieler fängt nie bei null an. | ja |
| **Grid-Layout aus einer Datei ableiten** | Errechnet, welches Buchstabenraster in einer Datei steckt, häufigster Buchstabe je Zelle, QWERTY als Rückfall. | ja |
| **Mini-Grids als Vorschau** | Überall, wo von einer Position die Rede ist, steht ein kleines 4x3 Raster mit markierter Zelle. Der Spieler muss x,y nicht im Kopf übersetzen. | teilweise |

**Ausgewählte Fundorte.**

- Undo, `crates/hotkey-editor/src/services/undo/mod.rs`, `install_keyboard_shortcuts` ab Zeile 179 (verifiziert). Persistenz `crates/hotkey-editor/src/persistence/editor_history_persistence.rs`.
- Routen, `crates/hotkey-editor/src/components/app/route.rs`, Zeilen 7 bis 28 (verifiziert).
- Die fünf localStorage-Schlüssel, alle in `crates/hotkey-editor/src/persistence/` verifiziert, `warcraft-hotkey-editor.custom-keys`, `.grid-layout`, `.undo-history`, `.onboarding-seen`, `.update-hotkeys-on-move`.
- Kaskadengruende `MoveReason`, `crates/warcraft-keybinds/src/cascade/planner.rs`, Zeilen 13 bis 38 (Stand `v0.8.0`).

**Zum Template-Katalog, gemessen.** Ein vollständiges Template ist größer als der Standard.
`CustomKeys_Neo_QWERTY.txt` hat 7122 Zeilen, 1094 Abschnitte, 991 `Hotkey`, 999 `Buttonpos`, 164
`Researchhotkey` und 163 `Researchbuttonpos`. Ein vollständiges Layout berührt praktisch jedes
Objekt im Spiel. Das entspricht exakt dem, was die Community per Pastebin und GitHub-Repo von
Hand macht, siehe QWEASZ, QWEASY und AZEQSW bei
[Liquipedia](https://liquipedia.net/warcraft/Custom_Hotkeys_Guide) und QWERTY, Dvorak und QWERTZ
bei [tlo9](https://api.github.com/repos/tlo9/Warcraft-3-Keybinds) (beide Klasse `community`).

### 5.3 Die Lücken zwischen Desktop und Mobil

Ausdrücklich benannt, denn eine benannte Lücke ist ein Arbeitsauftrag und eine überbrückte
Lücke ist eine Falle.

1. **Unit-Stats-Panel und Unit-Beschreibung fehlen auf Mobil vollständig.** Desktop rendert
   `UnitDetail` mit `UnitStatsPanel` und `UnitDescription`. Die mobile `PagerCard` rendert nur
   `PagerCardHeader`, `UnitCommandGrids`, `GridCarouselDots` und `HotkeyOverrideSection`
   (verifiziert im Quelltext). Kein Ersatzweg im Code.
2. **Rassen-Tabs und Modus-Tabs existieren auf Mobil nicht.** `EditorPage` kehrt unter 768 px vor
   dem Rendern von `EditorTabsBar` zurück, verifiziert in
   `.../editor_page/mod.rs` (`if is_mobile { return rsx! { MobileEditor {} } }`). `MobileEditor`
   listet stattdessen alle Units aller fünf Rassen in einer einzigen Kette. Der Spieler kann
   mobil **nicht** auf eine Rasse oder auf Melee eingrenzen, außer indirekt über die Chips im
   Such-Dialog.
3. **Unit-Liste und Katalog-Filter sind mobil nur im Such-Dialog erreichbar**, nicht als
   ständige Navigation.
4. **Toter Code auf dieser Branch.** `.../unit_list/style/mod.rs` und
   `.../unit_list/components/mobile_category_tabs/style/mod.rs` pflegen `mobile:`-
   Überschreibungen, aber unter 768 px wird `UnitList` nie gemountet. Diese `mobile:`-Styles sind
   unerreichbar, nur die `tablet:`-Varianten sind live. **Zu klären, ob das Absicht des
   laufenden Umbaus ist.**
5. **Der Grid-Layout-Editor-Dialog ist auf Touch nur halb bedienbar.** `LayoutTile` nutzt HTML5
   `draggable: "true"` mit `ondragstart` und `ondrop`, die einzige Stelle im Repo mit HTML5-Drag.
   Alles andere ist zeigerbasiert und damit touchfähig. Auf Mobil bleibt Tap und Key-Picker, was
   funktional reicht, aber der Tausch-Weg fehlt.
6. **Der Grid-Layout-Button im Header ist erst ab dem laptop-Band sichtbar**, darunter nur über
   das Burger-Menü.
7. **Die gesamte Aktionsleiste ist unterhalb des desktop-Bands per CSS ausgeblendet.** Kein
   Funktionsverlust, aber jede Aktion kostet einen Tap mehr. Das betrifft auch Undo und Redo, die
   im mobilen Kernablauf am häufigsten gebraucht werden, und Tastaturkürzel gibt es auf dem
   Handy nicht.
8. **Die Tastaturbedienung des Rasters hat mobil keine Entsprechung.**
9. **Kollisions- und Resolve-Seite sind mobil nur dünn ausgelegt.** 14 von 63 Style-Dateien unter
   `collisions_page` und 8 von 44 unter `resolve_page` tragen überhaupt eine `mobile:`-
   Überschreibung, gegenüber 82 von 163 unter `editor_page`. Die Seiten laufen dort, sind aber
   nicht durchgestaltet. Das ist eine grep-Zählung über die `style/mod.rs`-Dateien und **kein
   Urteil über die tatsächliche Darstellung**, das müsste im Browser geprüft werden.
10. **E2E-Abdeckung für Mobil ist ein einziger Spec**, `crates/hotkey-editor/e2e/tests/mobile/search-dialog.spec.ts`,
    gegen 37 Specs unter `tests/desktop/` (nachgezählt, 1 zu 37). Der mobile Pager, das
    Grid-Karussell und der Langdruck-Drag sind ungetestet, **obwohl der Langdruck-Drag der Kern
    auf Mobil ist**.
11. **Der Hilfe-Dialog beim Erstbesuch ist unklar.** Der `HelpButton` ist mobil per CSS
    ausgeblendet, mountet aber trotzdem und öffnet den `HelpDialog` bei `!onboarding_seen`, und
    `BurgerHelpItem` hält einen zweiten `HelpDialog`. Ob mobil genau ein Dialog erscheint,
    **lässt sich aus dem Code allein nicht sicher sagen** und müsste im Browser geprüft werden.

---

## 6. Konkurrenzanalyse

### 6.1 jcfieldsdev/warcraft3-hotkey-editor, der Vorgänger

**Die Korrektur zuerst, denn sie ist die wichtigste Aussage dieses Abschnitts.**

Die Auftraggeber-Prämisse lautet, der Vorgänger sei veraltet und habe bei Patches nie mitziehen
können. **Der zweite Teil ist so nicht belegbar und wäre gegenüber einer fachkundigen
Zielgruppe angreifbar.** Er hat Patch 2.0.2 nachgezogen, Commit "changes for patch 2.0.2" vom
2025-06-05, und ist danach stillgefallen. Letzter Commit 2025-06-11 "updated to allow numbers and
function keys as hotkeys", also rund 13 Monate ohne Codeänderung. Das Repo ist **nicht**
archiviert, hat 24 Sterne und 4 offene Issues. Der Commit vom 2024-11-19 "fixed bug with moving
researchbutton after button (or vice versa)" belegt außerdem, dass er `Buttonpos` **und**
`Researchbuttonpos` beherrschte. Er ist kein Strohmann. Verifiziert über die
[GitHub API](https://api.github.com/repos/jcfieldsdev/warcraft3-hotkey-editor/commits) (Klasse
`community`, `pushed_at 2025-06-11T13:28:41Z`, `created_at 2023-12-14`, `archived false`).

**Belastbar ist**, Stillstand seit rund 13 Monaten. **Nicht belastbar ist**, "konnte bei Patches
nie mitziehen". Wer diese These im Marketing oder in einer Produktentscheidung benutzt, benutzt
etwas Widerlegtes.

**Er ist die Standardempfehlung der Szene und lebt.** Wörtlich, "Use the Hotkey Editor from
jcfields. With predefined sets (QWER grid layout)" (KraSumLive-2679, 2022-06-17) und "'wc3 hotkey
editor' https://jcfields.gitlab.io/warcraft3-hotkey-editor/" (wc3tutor-1926, 2022-06-18), siehe
[us.forums.blizzard.com](https://us.forums.blizzard.com/en/warcraft3/t/how-do-i-set-up-hotkeys/28030)
(Klasse `community`, `staff_posts=0`). Die Live-Instanz antwortet mit HTTP 200. **Er ist der
Maßstab, an dem die Zielgruppe misst, nicht ein toter Strohmann. Er ist inhaltlich
stehengeblieben, nicht offline.**

**Was er löst, und zwar seit Jahren.** Kollisionserkennung und Drag-and-Drop mit Swap-Verhalten.
Wörtlich, "If there is a hotkey conflict, the hotkeys of the conflicting commands are highlighted
in red. Conflicts are automatically detected as you edit hotkeys. Additionally, units with
conflicting commands are highlighted in red in the list of units", und "Buttons can be
repositioned by dragging and dropping them. If dropped onto an empty space, the button is
relocated there. If dropped onto another button, both buttons swap positions", siehe
[README](https://raw.githubusercontent.com/jcfieldsdev/warcraft3-hotkey-editor/master/README.md)
(Klasse `community`).

> **Die wichtigste Konsequenz für jede Produktkommunikation.** Kollisionserkennung und
> Drag-and-Drop taugen **nicht** als Aufhänger. Sie sind seit Jahren Stand der Technik. Das deckt
> sich exakt mit der Auftraggeber-Prämisse, dass die Kollisionsauflösung nur eine grüne
> Baseline liefert.

**Wo er einen Vorsprung hat, ehrlich benannt.** Seine Datenbasis umfasst alle Units und Gebäude
aus Standard-Multiplayer **und den offiziellen Kampagnen**, wörtlich, "Includes all units and
structures from standard multiplayer and the official campaigns", mit Rassen-Icons am linken Rand,
getrenntem Melee und Campaign, und den vier Kampagnen-Rassen "Blood Elves, Draenei, Demons, and
Naga" unter Human, Orc, Undead und Night Elf, ebenda. **Das ist ein Umfangs-Vorsprung, kein
Defizit, und es ist neutral zu zitieren, nicht gegen ihn zu drehen.**

**Woran er scheitert.**

- **Er modelliert das Spielverhalten nicht exakt, und sagt das selbst.** Wörtlich, "The icon
  order algorithm may not be exactly the same as the game in cases where there are button position
  conflicts (which, by default, includes several creeps)", ebenda, Abschnitt Limitations. Eine
  Selbsteinräumung ist die stärkste Form eines Konkurrenzbelegs, und sie trifft exakt die
  Cascade- und Kollisionslogik, die dieses Produkt in eine getestete Domänen-Crate ausgelagert
  hat.
- **Sein Kernversprechen versagt stillschweigend.** Issue #2, erstellt 2024-08-18, seit rund 23
  Monaten offen, wörtlich, "'Research Improved Bows' and 'Research Marksmanship' don't save a
  button position entry in 'CustomKeys.txt'. Therefore, they won't change from their default
  positions even after drag+dropping in the webapp. Manually adding a Buttonpos=x,y entry to the
  CustomKeys.txt file works as expected", siehe
  [GitHub API](https://api.github.com/repos/jcfieldsdev/warcraft3-hotkey-editor/issues?state=all)
  (Klasse `community`). Das trifft den Produktkern direkt.
- **Falsch geschriebene Rawcodes brechen Keybinds stillschweigend.** Issue #5, erstellt 2025-08-30,
  offen, wörtlich, "I have noticed some skills aren't configured correctly, such as dark ranger's
  black arrow ([anba] vs [ANba]) and Death Knight's Animate dead ([auan] vs [AUan])", ebenda. Der
  Fehler ist im ausgelieferten Datenbestand reproduzierbar, Zeile 1894 `[auan]` und Zeile 3135
  `[anba]` in
  [default.txt](https://raw.githubusercontent.com/jcfieldsdev/warcraft3-hotkey-editor/master/www/hotkeys/default.txt).
  Er fällt in die Stillstandsphase nach dem letzten Commit. **Das ist die Fehlerklasse, die eine
  generierte Datenbasis ausschließt und eine handgepflegte nicht.**
- **Toggle-Zustände fehlen.** Issue #6, erstellt 2025-08-30, offen, wörtlich, "Some toggle
  actions, like Button/Unburrow, Bear form/Human form, Defend stance/Normal stance, etc. can be set
  to different positions and/or keybinds. This is super useful when a control group has units in
  both forms, e.g. druid bears with no mana, and human form druids to rejuvenate", ebenda. **Das
  ist exakt die `Unbuttonpos`-Domäne, in der dieses Repo eine eigene Regression hatte und einen
  Test dagegen hält (3.14). Der Konkurrent hat die Lücke offen, dieses Produkt hat sie
  geschlossen und abgesichert.**
- **Veraltete Daten kommen reaktiv herein, nicht proaktiv gepflegt.** Issue #3, erstellt
  2025-03-11, geschlossen, "Add Wand of Negation to Tomb of Relics" und "Patch 1.36 added wand of
  negation", ebenda. **Das belegt den Pflege-Mechanismus, einzeln, von Hand, nutzergetrieben. Es
  belegt die stärkere These "konnte bei Patches nie mitziehen" nicht.**
- **Seine Datenkette hängt an fremden Projekten.** Wörtlich, "Used RivSoft's Warcraft III data
  viewer for data and assets and WTii's unit tester map for testing", und "Includes QWEASZ hotkey
  set by wtvr", siehe
  [README](https://raw.githubusercontent.com/jcfieldsdev/warcraft3-hotkey-editor/master/README.md),
  Abschnitt Acknowledgments. **Vorsichtig deuten.** Das belegt Fremdbezug, es belegt **nicht**,
  dass der Fremdbezug schlechter ist. Interessanter Nebenbefund, der einzige belegte strukturierte
  Testansatz der Szene ist WTiis Unit-Tester-Map, also ein Custom-Map-Umweg und kein Spielfeature.

### 6.2 Der belastbarste Differenzierer, Mobil

Der Konkurrent ist auf Mobil **strukturell** unbrauchbar, nicht nur unschön. Seine `index.html`
enthält exakt drei Meta-Tags, charset, title und description, und ein grep auf `name="viewport"`
liefert null Treffer. In
[www/scripts/editor.js](https://raw.githubusercontent.com/jcfieldsdev/warcraft3-hotkey-editor/master/www/scripts/editor.js)
liefert `grep -cE 'touchstart|touchmove|pointerdown|pointermove'` den Wert 0, während
`dragstart`, `dragover` und `drop` vorhanden sind (Klasse `community`, eigenständig nachgeprüft).
Damit ist die Kernaktion Buttonverschieben auf Touchgeräten **nicht auslösbar**.

> **Formulierungsgrenze, verbindlich.** Belegt ist die **technische Unmöglichkeit**. **Nicht**
> belegt ist, dass Nutzer mobil editieren wollen. Das ist ein Beleg für einen Differenzierer, nie
> ein Beleg für Nachfrage. Siehe Segment E.

### 6.3 tlo9/Warcraft-3-Keybinds

Kein Editor, sondern ein Satz fertiger Dateien zum Umbenennen und Hineinkopieren, `CustomKeys-Sample.txt`,
`CustomKeys-QWERTY-Grid.txt`, `CustomKeys-Dvorak-Grid.txt`, mit der Anleitung "Rename one of the
files ... place it into your CustomKeyBindings folder" und "Read 'CustomKeyInfo.txt' for
customization info". Anpassung pro Unit ist nicht vorgesehen. Das Repo ist praktisch tot und lebt
von Fremdbeiträgen, `created_at 2020-01-29`, `pushed_at 2025-05-07T14:57:45Z`, 9 Sterne, 0 offene
Issues, letzter Push ist ein Merge eines fremden Pull Requests, letzter inhaltlicher
Autoren-Commit 2024-05-04. Verifiziert über die
[GitHub API](https://api.github.com/repos/tlo9/Warcraft-3-Keybinds) (Klasse `community`).

**Was wir daraus lernen.** Es belegt die Alternative "fertige Datei übernehmen" und dass sie ohne
Editor-Ebene auskommt. Und die Dvorak-Datei belegt zusammen mit den QWERTZ- und AZERTY-Varianten
im eigenen Katalog, dass Tastaturlayout-Varianten ein reales Thema sind.

### 6.4 Blizzards In-Game-Editor

Siehe 3.10. Zusammengefasst. Blizzard hat den Bildschirm gebaut, das ist belegt. Alles Weitere ist
übereinstimmender Nutzerbefund, kein Blizzard-Wort. Er ist die relevanteste Konkurrenz, weil er
mitgeliefert wird, und er ist gleichzeitig der Grund, warum Segment C existiert.

### 6.5 Was im Bestand ausdrücklich NICHT belegt ist

Diese Aussagen wurden geprüft und **widerlegt oder nicht belegt**. Sie dürfen **nicht** benutzt
werden, auch wenn sie plausibel klingen.

- Der offizielle In-Game-Editor kann die Buttons nicht in der Command Card verschieben, und
  Spieler fordern genau das. **Nicht in dieser Form belegt.**
- Patches ändern Rawcodes, Beispiel Shaman Purge von `aprg` auf `apg2`. **Widerlegt.**
- Reforged hat 3 oder 4 Codes geändert. **Widerlegt.**
- Blizzard hat mit 2.0.3 eine Konflikt-Anzeige eingebaut, und Reforged wird bis 2026 gepatcht,
  Version 2.0.4. **Als blizzard-offiziell widerlegt.**
- Die Zielgruppe lehnt das Editieren von Textdateien explizit ab. **Widerlegt.** Das widerspricht
  außerdem direkt der Regel, dass die Zielgruppe ihr Handwerk kennt.
- `CustomKeys.txt` greift in Custom Games gar nicht. **Widerlegt.**
- Der Vorgänger hat keinerlei Persistenz zwischen Sitzungen. **Widerlegt.**
- AucT Hotkeys, Warkeys und die W3Champions-Launcher-Hotkeys als Konkurrenz. **Widerlegt.**
- Der Begriff "command card" stammt aus einem In-Game-Tooltip. **Widerlegt.**
- Lokalisierung kollidiert zusätzlich, Beispiel spanisches D für Stop gegen Dismember.
  **Widerlegt.**

---

## 7. User Stories

Format, "Als **Segment** will ich **Ziel**, damit **Nutzen**". Segmente sind die aus Abschnitt 4.

### 7.1 Stories, die die heutigen Features abdecken

1. Als **Layout-Bauer** will ich einen Button im 4x3 Raster einer Unit per Drag auf eine andere
   Zelle ziehen, damit die Taste der Position entspricht und ich mir keinen Buchstaben merken
   muss.
2. Als **Layout-Bauer** will ich beim Verschieben eines Toggle-Buttons die `Unbuttonpos`
   automatisch mitziehen, damit der Off-State nicht zurückbleibt und die Fähigkeit unbenutzbar
   wird (3.4, 3.14).
3. Als **Layout-Bauer** will ich alle Raster einer Unit nebeneinander sehen, Command Card,
   Baumenü, Uprooted und Research, damit ich eine Unit in einem Durchgang fertigstelle.
4. Als **Layout-Bauer** will ich nach dem Verschieben direkt die Taste setzen, damit Position und
   Hotkey zusammen entstehen und nicht in zwei Durchläufen.
5. Als **Layout-Bauer** will ich beim Setzen einer Taste sofort erfahren, ob sie auf dieser
   Command Card schon vergeben ist und an wen, damit ich nicht blind in einen undefinierten
   Zustand laufe (3.4).
6. Als **Layout-Bauer** will ich entscheiden können, ob die Grid-Taste beim Verschieben mitgesetzt
   wird oder die alte Taste am Button klebt, damit ich sowohl Grid-Denke als auch
   Muskelgedächtnis bedienen kann.
7. Als **Layout-Bauer** will ich einen abgelehnten Zug samt namentlich genanntem Blockierer und
   Ausweg erklärt bekommen, damit ich nicht raten muss, warum nichts passiert.
8. Als **Layout-Bauer** will ich jederzeit sehen, welche Units eine Fähigkeit teilen, damit ich
   verstehe, an welchen anderen Command Cards mein Zug zieht (3.2).
9. Als **Layout-Bauer** will ich jeden geplanten Kaskadenzug vor dem Anwenden einzeln und mit
   Grund sehen, Fight, Spill, Swap oder GapPull, damit ich die Auflösung prüfe statt sie zu
   erleiden (3.3).
10. Als **Layout-Bauer** will ich jede Änderung zurücknehmen können und die Historie über einen
    Reload hinweg behalten, damit ein Fehlgriff in einer 30- bis 60-minütigen Sitzung nicht die
    Sitzung kostet.
11. Als **Preset-Übernehmer** will ich aus sieben fertigen Templates mit gerenderter Vorschau
    wählen, damit ich einen Startpunkt habe, der zu meiner Tastatur und meinem Geschmack passt
    (3.12).
12. Als **Preset-Übernehmer** will ich ein Buchstabenraster einmal festlegen und auf alles
    anwenden, damit ich eine grüne Baseline bekomme, von der aus ich verfeinere.
13. Als **Geschädigter** will ich meine bestehende Datei hochladen und normalisiert
    zurückbekommen, damit jahrelange Handarbeit nicht verloren ist (3.9).
14. Als **Geschädigter** will ich, dass mein Stand ohne Konto und ohne Server erhalten bleibt,
    damit mir kein Optionsmenü meine Arbeit überschreibt (3.9).
15. Als **Geschädigter** will ich beim Export den exakten Zielordner und die Warnung zum
    Dateinamen genannt bekommen, damit die Datei vom Spiel auch gelesen wird.
16. Als **Geschädigter** will ich den fertigen Text ansehen und kopieren können, ohne
    herunterzuladen, damit ich prüfen kann, was tatsächlich herauskommt.
17. Als **Rawcode-Sucher** will ich den vierstelligen Rawcode direkt auf der Unit-Karte sehen,
    damit ich ihn nicht über ein In-Game-Debug-Verfahren ablesen muss (3.6).
18. Als **Rawcode-Sucher** will ich über alle Units aller Rassen volltextsuchen, wahlweise nach
    Unit-Name oder Fähigkeitsname, damit ich zu der Stelle springe, die ich meine.
19. Als **Layout-Bauer** will ich jederzeit sehen, wie viele Konflikte noch offen sind, ohne die
    Seite zu wechseln, damit ich weiß, ob ich fertig bin.
20. Als **Layout-Bauer** will ich Kollisionen nach ihren drei Arten getrennt sehen,
    Cross-Unit-Positionsinseln, Positionskollisionen und Hotkey-Kollisionen, damit ich weiß,
    welche ich lokal lösen kann und welche nicht (3.2).
21. Als **Layout-Bauer** will ich eine Stelle im Editor per URL teilen und bookmarken, damit ich
    eine Frage an die Community mit einem Link stellen kann.
22. Als **Layout-Bauer** will ich beim ersten Öffnen die vollständige Standardbelegung mit
    materialisierten Positionen vorfinden, damit ich weiß, wo alles startet (3.5).
23. Als **Layout-Bauer** will ich bei jeder Positionsangabe ein kleines 4x3 Raster mit markierter
    Zelle sehen, damit ich x,y nicht im Kopf übersetzen muss.
24. Als **Layout-Bauer** will ich Neutral als vollwertigen Rassen-Tab, damit Tavernenhelden,
    Söldner und Creeps kein Sonderfall sind.
25. Als **mobiler Planer** will ich mich per Wisch von Unit zu Unit durcharbeiten, eine Unit pro
    Bildschirm, damit ich unterwegs Fortschritt mache.
26. Als **mobiler Planer** will ich einen Button per Langdruck aufnehmen und ziehen, damit der
    Kern des Produkts auf dem Handy überhaupt auslösbar ist (6.2).
27. Als **mobiler Planer** will ich die mehreren Raster einer Unit als Karussell mit Punkten
    durchblättern, damit auf einem schmalen Bildschirm nichts verlorengeht.
28. Als **mobiler Planer** will ich alle Aktionen im Burger-Menü finden, damit mir mobil keine
    Funktion fehlt.

### 7.2 Neue Stories, noch nicht umgesetzt, aus der Recherche zwingend folgend

Jede trägt ihre Herleitung. **Keine davon ist eine Zusage, alle sind Vorschläge.**

29. Als **mobiler Planer** will ich Undo und Redo mit einem Tap erreichen statt über das
    Burger-Menü, damit der häufigste Handgriff im Kernablauf nicht der teuerste ist. **Herleitung**,
    Lücke 7 in 5.3, plus die Tatsache, dass es mobil keine Tastaturkürzel gibt.
30. Als **mobiler Planer** will ich auf eine Rasse oder auf Melee eingrenzen können, damit ich mich
    nicht durch alle Units aller fünf Rassen wische, um an Undead zu kommen. **Herleitung**, Lücke 2
    in 5.3.
31. Als **Layout-Bauer** will ich, dass der mobile Langdruck-Drag durch einen E2E-Test abgesichert
    ist, damit der Kern auf Mobil nicht stillschweigend bricht. **Herleitung**, Lücke 10 in 5.3, 1
    Spec gegen 37.
32. Als **mobiler Planer** will ich die Buchstaben im Grid-Layout-Editor auch per Finger tauschen,
    damit das letzte HTML5-Drag im Repo mich nicht wie den Konkurrenten aussperrt. **Herleitung**,
    Lücke 5 in 5.3 plus 6.2. Es wäre unschön, exakt den Fehler zu behalten, den wir dem
    Konkurrenten vorhalten.
33. Als **Geschädigter** will ich beim Export an den Read-Only-Schritt erinnert werden, damit das
    Spiel mir die exportierte Datei nicht beim nächsten Lautstärkeregler überschreibt.
    **Herleitung**, 3.9, Xelfire und Neo. **Vorsicht**, der Workaround rettet die Positionen laut
    Berfner nicht, also darf der Hinweis nichts versprechen, was er nicht hält.
34. Als **Geschädigter** will ich vor dem In-Game-Optionsmenü gewarnt werden, damit ich meine
    Positionen nicht durch einen Klick auf Speichern verliere. **Herleitung**, 3.10, ffFiend und
    syNtec. **Vorsicht**, das ist ein Nutzerbefund und darf **niemals** als Blizzard-Aussage
    formuliert werden.
35. Als **Layout-Bauer** will ich, dass klar ist, warum Shop-Items und Inventar-Items hier nicht
    belegbar sind, damit ich es nicht als Bug melde und nicht stundenlang danach suche.
    **Herleitung**, 3.13, WTVR. **Vorsicht**, das ist eine Formatgrenze, und die Erklärung muss so
    knapp sein, dass sie die Zielgruppe nicht für Anfänger hält.
36. Als **Layout-Bauer** will ich meine Layout-Datei als Link teilen können, damit ich sie in
    einem Forum oder auf Discord weiterreiche, so wie die Szene es heute per Pastebin und
    GitHub-Repo macht. **Herleitung**, 3.12 und 5.2, Liquipedia und tlo9. **Achtung, harte
    Architekturgrenze.** Das Produkt hat keinen Server, siehe `ARCHITECTURE.md`. Ein Weg dorthin
    müsste ohne Server auskommen, und ob das bei rund 7000 Zeilen sinnvoll geht, ist offen. **Diese
    Story ist ein Vorschlag mit ungeklärter Machbarkeit, keine Zusage.**
37. Als **Layout-Bauer** will ich Kampagnen-Units bearbeiten können, so wie der Konkurrent es
    anbietet. **Herleitung**, 6.1, sein belegter Umfangs-Vorsprung. **Ungeklärt**, ob dieses Produkt
    diese Daten hat. Der Katalog führt Modus-Tabs mit Melee und Campaign, ob die Datenbasis
    vollständig ist, ist aus dem Code nicht belegt und müsste geprüft werden.
38. Als **mobiler Planer** will ich auf der Kollisions- und Resolve-Seite eine durchgestaltete
    Ansicht, damit die grüne Baseline auch unterwegs erreichbar ist. **Herleitung**, Lücke 9 in
    5.3. **Vorsicht**, die Zählung ist ein grep über Style-Dateien und **kein** Urteil über die
    tatsächliche Darstellung. Vor dieser Story steht eine Prüfung im Browser.
39. Als **Layout-Bauer** will ich eine Unit als fertig markieren können, damit ich in einer 30-
    bis 60-minütigen Sitzung über rund tausend Objekte den Überblick behalte. **Herleitung**, 3.8
    gemessen, plus die Auftraggeber-Angabe zur Sitzungsdauer. **Klasse gemischt**, die Menge ist
    belegt, die Sitzungsdauer ist `auftraggeber`, das Feature ist eine Ableitung und **kein
    Nutzerwunsch aus einer Quelle**.
40. Als **mobiler Planer** will ich die Stats der Unit auch mobil einsehen können. **Herleitung**,
    Lücke 1 in 5.3. **Ausdrückliche Gegenrede**, das ist Beiwerk, nicht Kern, und die Karte ist
    knapp. Diese Story ist gelistet, weil die Lücke real ist, **nicht** weil sie priorisiert
    gehört.

---

## 8. Was das für jede Arbeit am Produkt bedeutet

Das hier ist der Teil, den ein Agent gelesen haben muss, bevor er irgendetwas anfasst.

### 8.1 Der Kern ist das Ziehen der Buttons, Unit für Unit

Klasse `auftraggeber`, und die belegte Lage stützt es von allen Seiten. Jede Unit muss einzeln
angepasst werden, deshalb ist der Command-Card-Editor das Produkt. **Die Kollisionsauflösung ist
nicht der Kern**, sie liefert nur eine grüne Baseline, und der Konkurrent hat sie seit Jahren
(6.1). **Die Suche ist nicht der Kern**, sie ist ein Sprungwerkzeug.

**Was daraus folgt.** Eine Änderung, die den Drag-Pfad langsamer, wackliger oder unzuverlässiger
macht, ist ein Rückschritt, auch wenn sie jede Regel in `COMPONENTS.md` einhält. Eine Änderung,
die einen Nebenschauplatz poliert und den Drag-Pfad anfasst, ohne ihn zu prüfen, ist ein Risiko.
Genau diese Lücke, Regelkonformität ohne Produktverständnis, ist der Grund, warum es dieses
Dokument gibt.

### 8.2 Eine Sitzung dauert 30 bis 60 Minuten

Klasse `auftraggeber`, im Code **nirgends messbar oder hinterlegt**, und extern unbelegt. Die
5-Minuten-Angabe von gaming-tools misst etwas anderes, nämlich das Installieren einer fertigen
Datei (3.11).

**Was daraus folgt.** Der Editor ist kein Werkzeug für einen Handgriff, er ist eine
Arbeitsumgebung für eine dreiviertel Stunde konzentrierter Wiederholung. Deshalb ist Undo nicht
Beiwerk, sondern Sicherheitsnetz. Deshalb ist Persistenz nicht Komfort, sondern Voraussetzung.
Deshalb kostet jede zusätzliche Interaktion pro Unit hochgerechnet auf rund tausend Objekte (3.8)
real Zeit. Und deshalb ist ein Datenverlust an dieser Stelle kein kleiner Bug.

### 8.3 Mobil ist echter Use Case, kein Zugeständnis

Klasse `auftraggeber`. Extern belegt ist **nur** die Gegenseite, nämlich dass der Konkurrent es
strukturell nicht kann, kein viewport-Meta-Tag und null Touch- oder Pointer-Handler (6.2).

**Was daraus folgt.** Der mobile Pfad ist gleichwertig zu entwickeln und gleichwertig zu testen.
Der aktuelle Zustand ist es nicht, ein einziger E2E-Spec gegen 37 auf Desktop, und ausgerechnet der
Langdruck-Drag ist ungetestet, obwohl er mobil der Kern ist (5.3, Lücke 10). Wer mobil anfasst,
prüft im Browser, denn mehrere Lücken in 5.3 sind aus dem Code allein **nicht** entscheidbar.

**Und die ehrliche Gegenprobe.** Wer Mobil gegenüber Dritten begründet, benutzt den
Konkurrenz-Differenzierer, nicht eine erfundene Nachfrage. "Der Konkurrent kann es technisch nicht"
ist belegt. "Nutzer wollen es" ist es nicht.

### 8.4 Navigation ist nicht die Hauptfunktion, aber das ist keine Ausrede

Suche, Unit-Liste, Rassen-Tabs und Deep-Links sind Sprungwerkzeuge. Sie sind nicht der Kern.

**Und trotzdem.** Bei rund tausend Objekten (3.8) ist schlechte Navigation ein Zeitfresser, der
direkt von der Sitzung abgeht. Für Segment D ist die Rawcode-Anzeige die Lösung eines belegt 13
Jahre alten Problems (3.6). "Ist ja nur Navigation" rechtfertigt keinen schlechten Zustand, es
rechtfertigt nur eine niedrigere Priorität gegenüber dem Drag-Pfad.

### 8.5 Unterstelle der Zielgruppe niemals Unwissen

Sie tippt seit zwanzig Jahren Rawcodes von Hand. Sie hat den Custom-Keys-Schalter längst umgelegt.
Sie kennt `CustomKeyBindings`. Jede Erklärung, die sie für Anfänger hält, ist falsch, auch wenn
sie sich aus einer echten Quelle herleiten lässt. Das ist keine Stilfrage, das ist der zweite
dokumentierte Fehler des verbrannten ersten Versuchs (`PRODUCT_RESEARCH_RULES.md`).

### 8.6 Grenzen des Formats sind keine Bugs

Inventar- und Shop-Items sind über `CustomKeys.txt` nicht belegbar, die Community löst das mit
AutoHotkey (3.13). Eigener Tooltip-Text existiert in Reforged nicht mehr (3.13). Wer so etwas als
Feature-Lücke einplant, plant gegen das Format.

### 8.7 Wer dieses Dokument fortschreibt

Es gelten `PRODUCT_RESEARCH_RULES.md` unverändert. Ohne abrufbare Quelle ist eine Aussage
ungültig. Jede Aussage trägt ihre Klasse. **Ein Forum auf einer Blizzard-Domain ist `community`,
nicht `blizzard-offiziell`**, und in genau diesem Bestand ist das viermal der Fall,
`staff_posts=0`. Die einzige echte blizzard-offizielle Quelle hier sind die Patch Notes zu 2.0.0,
und sie sagen nichts über Button-Positionen. Eine ehrliche Lücke ist immer besser als ein
unbelegter Satz.

---

## 9. Offene Lücken, gesammelt

Benannt, nicht überbrückt.

**Auftraggeber-Prämissen ohne externen Beleg.**

- Die Kooperation mit Back2Warcraft. Belegt sind nur B2Ws Existenz und ein Präzedenzfall der
  Projektförderung bei W3Champions (1).
- Tage bis Wochen Handarbeit ohne Editor, endend in zig Konflikten. Belegt ist nur die **Menge**,
  rund tausend Objekte, nicht die **Zeit** (3.8).
- 30 bis 60 Minuten für ein neues Template. Im Code nirgends messbar oder hinterlegt (8.2).
- Mobil als Nutzerwunsch. Belegt ist nur, dass der Konkurrent es technisch nicht kann (8.3).

**Eine Auftraggeber-Prämisse ist eingeschränkt.**

- "jcfieldsdev konnte bei Patches nie mitziehen" ist **nicht belastbar**. Belastbar ist nur
  Stillstand seit rund 13 Monaten nach einem nachgezogenen Patch 2.0.2 (6.1).

**Aus dem Code nicht entscheidbar, Prüfung im Browser nötig.**

- Ob auf Mobil genau ein Hilfe-Dialog erscheint (5.3, Lücke 11).
- Ob die Kollisions- und Resolve-Seite mobil tatsächlich schlecht aussehen. Die 14-von-63- und
  8-von-44-Zählung ist ein grep, kein Urteil (5.3, Lücke 9).
- Ob die `mobile:`-Styles unter `unit_list` Absicht des laufenden Umbaus oder toter Code sind (5.3,
  Lücke 4).
- Ob `update_hotkeys_on_move` und der Carriers-Dialog mobil erreichbar sind. Im Katalog steht
  "unbekannt".

**Datenlage.**

- Ob dieses Produkt Kampagnen-Units vollständig führt, ist aus dem Code nicht belegt (7.2, Story
  37).
- `warcraft-keybinds` liegt nicht in diesem Repo, und der Arbeitsbaum trägt einen `[patch]`-Block
  auf eine lokale Arbeitskopie, im `Cargo.toml` selbst als "DEVELOPMENT ONLY" markiert. Alle
  Domänen-Angaben beziehen sich auf `v0.8.0` (2).
- Dieser Katalog beschreibt `feature/mobile-redesign` mit unbeendetem Umbau, nicht `main` (5).

**Nicht recherchierbar gewesen.**

- Ob der Vorgänger tatsächlich der Vorgänger dieses Produkts ist. Im gesamten Repo, `Cargo.toml`,
  `docs/`, `README.md` und `CLAUDE.md`, findet sich dazu nichts. Klasse `auftraggeber`.
