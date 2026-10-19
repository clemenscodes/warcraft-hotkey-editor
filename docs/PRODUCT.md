# The Product

> This document describes **why** this repository exists and **for whom**. It is the
> onboarding read for new developers and agents, on equal footing with `ARCHITECTURE.md`,
> `COMPONENTS.md`, `RUST_STYLE.md`, `AGENTS.md` and `DOMAIN.md`, which govern the HOW of
> the code. This file holds the finished picture of the product itself.

---

## 1. What the product is

A pure frontend editor in Rust, Dioxus and WebAssembly for the file `CustomKeys.txt` from
Warcraft III Reforged. No server, no database, no account, no cloud storage. The user opens
the page, builds a key layout, downloads the file and drops it into the game.

Two things define it. First, it was built in cooperation with **Back2Warcraft**, one of the
best known Warcraft III caster teams, and therefore carries real domain knowledge that cannot
be invented. Second, it feeds its game data **automatically from Blizzard's CASC archive**
instead of maintaining it by hand. Together these are the reason that decisions which look
arbitrary from the outside usually are not. Anyone who rebuilds something because it would be
"more logical" is very likely tearing out domain knowledge.

---

## 2. Why the product exists

The starting point is ergonomics, not convenience. Warcraft III is extremely key intensive at
over 200 actions per minute, and the default hotkeys are scattered across the whole keyboard
with no scheme, and they differ per race. In League of Legends the abilities sit on QWER, in
Warcraft they are scattered wildly, and on top of that the abilities themselves are scattered
across the command grid. Anyone who wants to improve their micro has to remap all of that.

The game offers no adequate way to do this. Its built in hotkey editor only changes keys,
**not positions**. Its "Grid" option overwrites the default positions with QWER-ASDF-YXCV, but
leaves the conflicts between units in place and does not let you adjust positions. That leaves
only the "custom" option, and for that you have to touch and understand the text file yourself.

And that text file is nearly impossible to tame by hand. Four reasons.

- **Collisions are not locally solvable.** Hotkey and position are encoded at the **ability
  level**, not per unit. Every ability carries its own object id, and when several units share
  the same ability, their position and key are inevitably tied together. Take Abolish Magic.
  The Dryad carries it as `Aadm`, which belongs only to her and is freely configurable. The
  same spell on the mercenary Forest Troll Shadow Priest is `ACdm`, shared by ten other neutral
  units, and there is even a third variant `ACd2`. A remap on one unit therefore breaks another.
- **The forced remap cascades.** Anyone who cannot move a shared ability has to push another
  one off its position, and that propagates. The community used to compute those cascades by
  hand.
- **There is no starting point.** The default `CustomKeys.txt` contains no `Buttonpos` lines at
  all, so you cannot even see where anything starts.
- **Finding the rawcodes is a problem unsolved for over a decade.** The four character address
  of an ability is nowhere convenient, and the community had to build itself an in game debug
  procedure to read it.

The file holds around one thousand objects. A collision free configuration by hand is
therefore not merely tedious, it is impossible without an algorithm built for exactly this
problem.

On top of that, the game does not reliably keep the file. It overwrites `CustomKeys.txt` on
option changes, the read only workaround fails to save exactly the positions, and the in game
screen shipped with patch 2.0 discards on save anything it does not know. That is the reason
this product keeps its own persistent source of truth in localStorage.

And the scene so far advises the opposite of customizing. The most read guides say take a
finished preset and change nothing. That is exactly the gap the product fills. It is built for
the people who want to customize. Grid is not a standard here but a family of competing tastes,
including keyboard layout variants, which is why the template catalog carries several competing
ideas rather than one correct one.

---

## 3. The domain in brief

You cannot reason about hotkeys without the game they belong to. The domain is not "five races
and some keys", it is Warcraft III, and none of it is stated in the code.

**The game.** Warcraft III (Reign of Chaos plus its expansion The Frozen Throne, re-released as
Reforged) is a real-time strategy game. You pick a race, build a base, harvest three resources
(gold, lumber, food/upkeep), train units and RPG-like Heroes that level up and learn abilities,
research upgrades, and fight. The map is also full of creeps, neutral hostile units guarding
gold mines and neutral buildings.

**The command card.** Select any unit or building and the UI shows its command card, a fixed
4x3 grid of twelve buttons for everything it can do, Move, Stop, Hold, Attack, its spells and
abilities, a worker's build menu, a building's train/research menu. Different races have
different units and buildings, hence different command cards, which is why the editor is
organized by race.

**`CustomKeys.txt`** is the file the game reads to override those keys and positions, addressed
per object through a four character rawcode. The format knows three hotkey fields, `Hotkey`,
`Unhotkey` (cancel/unlearn), `Researchhotkey` (a hero learning an ability), and three position
fields, `Buttonpos`, `Unbuttonpos`, `Researchbuttonpos`, each as `x,y` in the grid with x from
0 leftmost to 3 rightmost and y from 0 top to 2 bottom. A hero skill therefore has two
independent grid slots, one in the learn menu and one on the command card.

**Grid layouts** are the core of the remapping practice. The twelve grid positions are mapped
onto a fixed keyboard block (for example `QWER / ASDF / ZXCV`), so that the key matches the
**position** rather than a per-ability letter you have to memorize.

**The five races, and how `Race` is modeled.** Four races are selectable at the start of a
match, Human, Orc, Night Elf, Undead. The fifth, Neutral, cannot be picked at the start, yet
everyone plays it anyway, tavern Heroes bought in-game by any race, hireable mercenaries, plus
creeps and neutral buildings, and those need custom hotkeys just like the rest. So in this
editor Neutral is a full, equal race alongside the four, five first-class race tabs, none
second-class. The race set is therefore closed, compile-time, and game-defined, exactly these
five, each once, never runtime data fetched through a service or an aggregate, and never a
`[Race; N]` that could hold duplicates or the wrong count.

---

## 4. The CASC advantage

This is the unique selling point. On every patch Blizzard adjusts the configuration files in
its CASC archive, which hold the entire game data. This product extracts and parses them
through the data layer `clemenscodes/warcraft-data` and therefore picks up **every** change
automatically, including ones that appear in no patch notes. A new patch can be extracted and
shipped again within minutes.

A user notices this because the displayed data is always accurate and never drifts away from
the game. A hand maintained tool drifts inevitably. Concretely, the Death Knight ultimate
Animate Dead carries the id `AUa2` today, the old one was `AUan`. A tool that still assigns
`AUan` writes a line onto a stale id, and in game the ultimate then takes no hotkey and no
position update. Exactly this class of error is on file as an open issue for the predecessor
(section 8).

The data layer did not originate for this editor but for a different project, a live in game
overlay for Warcraft III. It made the editor half finished from the start. The actual core work
was the cascade resolution algorithm.

---

## 5. Back2Warcraft

The cooperation began when Back2Warcraft talked in a stream about the poor state of hotkeys in
Warcraft III, and the editor was built in response. B2W reached out over Discord, received
preview links and helped in many places. They showed how the domain behavior of individual
units actually works, which units carry which fields, and what the old competitor did well, and
they tested the editor for bugs throughout.

In the process, knowledge surfaced that the CASC data does not directly yield. On the
**Phoenix** the unit carries the active ability `apxf`, which is not shown in game, while only
the passive `Ahpe` appears, so the active variant has to disappear from the command card. On
the **tree units**, in one run Uproot stayed on R instead of V and Switch stayed on V instead
of C. Without Back2Warcraft these edge cases would not have surfaced and many bugs would have
stayed undetected.

B2W also promotes the product publicly. The subdomain
[hotkeys.back2warcraft.com](https://hotkeys.back2warcraft.com/) forwards to the editor, and the
[B2W tools page](https://back2warcraft.com/tools) lists it as "Clemens' Hotkey Editor" alongside
W3Champions.

---

## 6. Target audience

**Practically every Warcraft player is relevant, across every mode.** Almost everyone plays
with conflicts, they are just used to it. This concerns not only 1v1, but every game mode, the
campaign and all custom games. A 4v4 player benefits just as much as a ladder player.

**Realistic adoption is narrower than relevance.** Most players have trained their muscle memory
so hard over the years that a conflict free switch is barely an option anymore. The actual
addressees are therefore two groups. Those who are starting or learning today, and those who
finally want to eliminate the conflicts to raise their game to a higher level. Even the non
switchers are not excluded, the app is built for them too, and a friction free UX might move
them to try after all.

**What the audience already knows, and what you must never explain to them.** They have
experience with the game and know the game mechanics. They know which ability sits at which
place on which unit. They know all game settings and the CustomKeys syntax partially, but not
fully. Any explanation that treats these people as beginners is wrong. They have been typing
rawcodes into text files by hand for twenty years.

The concrete usage patterns that have evidence behind them are five.

- **Segment A, the Layout Builder.** Wants no finished preset but a layout of their own, with
  position **and** key together. This is the core user. They are blocked by the missing starting
  positions, the collisions that are not locally solvable, the cascades and the sheer volume.
- **Segment B, the Preset Adopter.** Takes a finished file and is done. They become Segment A as
  soon as the preset does not fit, and that happens, because Grid is a family of tastes. The
  template catalog is the bridge from B to A, not the goal.
- **Segment C, the Casualty of the 2.0 era.** Had a working, hand maintained file and lost it to
  the options menu. They need an external, persistent source of truth and a reliable re export,
  exactly the localStorage architecture.
- **Segment D, the Rawcode Seeker.** Knows what they want but cannot find the four character
  address. For them the search and the rawcode display on the card are the solution to a problem
  over a decade old.
- **Segment E, the Mobile Planner.** Keeps working on the go, on the train, at the bus stop, in
  the waiting room. Mobile is a real, equal use case (section 10).

The only people not meant are non Warcraft players who, even with the tool, have no interest in
adjusting their CustomKeys. No one else.

---

## 7. Feature catalog

All paths are repo relative. The domain crate `warcraft-keybinds` does not live in this repo,
it is pinned as a git dependency. This catalog describes the state on the branch
`feature/mobile-redesign`, whose UI rework is not yet finished.

### 7.1 The core

These features are the product. Everything else is supporting cast.

| Feature | What it does for the player | Mobile |
| --- | --- | --- |
| **Command card editor with drag and drop** | Drag a button in a unit's 4x3 grid onto another cell. Click selects, double click opens the key picker, drag moves or swaps. On touch the drag starts on long press, pulling away cancels, scrolling is locked during the drag. | yes |
| **UnitCommandGrids** | Shows all of a unit's grids side by side, command card plus build menu, uprooted menu and research menu. The same building block on desktop and mobile. | yes |
| **Grid editor variants** | Three flavors of the same editor, because the game knows three position fields. Every menu is operated the same way. | yes |
| **Hotkey override section** | Below the grid sits the clicked button, and the player sets its key. The second half of the core work, position first, then key. | yes |
| **Hotkey conflict detection on set** | Says immediately whether the key is already taken on the same command card, and by which button. | yes |
| **Key picker dialog** | On screen keyboard instead of typing. Taken, free and conflicting keys look different. | yes |
| **Move blocker warning** | If the target cell is reserved for another ability's off state, the move is rejected, and a toast names the blocker and the way out. | yes |
| **update_hotkeys_on_move toggle** | Decides whether the grid key is set along on a move, or the old key stays stuck to the button. | partial |
| **Mobile unit pager** | Below 768 px a vertical snap pager replaces the whole interface, one unit per screen, across all units of all five races. | yes |
| **PagerCard** | Icon, name and rawcode of the unit, below it the grids and the hotkey override row, tinted per race. | yes |
| **Grid carousel with dots** | A unit's several grids sit stacked as a horizontal carousel on mobile, dots show count and position. | yes |
| **Drag follower overlay** | During the drag a copy of the tile hangs on the pointer, the target cell is marked with a ring, Escape cancels. | yes |
| **Hotkey badge with states** | Every tile carries its letter. Conflict and passive ability look different, without lookup. | yes |
| **Keyboard operation of the grid** | Space and Enter select the focused tile, the editor is usable without a mouse. | no |

**Where the core lives.**

- Drag mechanics and constants in `.../grid_editors/shared/grid_editor/presentation/drag_state.rs`
  (`TOUCH_CANCEL_THRESHOLD_PIXELS = 12.0`, `LONG_PRESS_MS = 300`), keyboard operation in
  `.../grid_editor/presentation/mechanics.rs`.
- The three variants under `.../grid_editors/`, `command_grid_editor`, `research_grid_editor`,
  `alternate_form_grid_editor`.
- Shared building blocks `.../editor_page/components/shared/unit_command_grids/` and
  `.../editor_page/components/shared/hotkey_override_section/`.
- Conflict detection in `crates/hotkey-editor/src/services/customkeys/hotkey_override.rs`,
  domain `CustomKeys::find_hotkey_conflict`.
- Mobile pager in `.../editor_page/components/mobile_editor/`, ordering Human, Orc, Nightelf,
  Undead, Neutral, then name.

### 7.2 The supporting cast

Useful, in part indispensable, but not the core.

| Feature | What it does for the player | Mobile |
| --- | --- | --- |
| **Undo and redo** | Every change is reversible and survives a reload. Keyboard shortcuts on desktop. | partial |
| **Import of your own CustomKeys.txt** | Upload an existing file, it is overlaid on the factory base and normalized. | partial |
| **Export** | Downloads the result as a file. The info dialog names the exact target folder and warns about differing file names. | partial |
| **Preview dialog** | Shows the finished text to read and copy, verbatim the localStorage content. | partial |
| **Templates dialog, seven presets** | Finished layouts as a starting point with preview. Default, Clemens DotA like in QWERTY, QWERTZ, AZERTY, NEO (Back2Warcraft) in QWERTY, QWERTZ, AZERTY. | partial |
| **Grid layout editor** | Sets which letter belongs to which of the twelve positions, and writes it onto every binding. The green baseline. | partial |
| **Collisions view, three classes** | Cross unit position islands, position collisions within a unit, hotkey collisions within a unit. Every entry is linkable. | partial |
| **Collision counter as a badge** | Shows at all times how many conflicts are open. | yes |
| **Resolve page with cascade plan** | Shows every planned move individually with a reason, Fight, Spill, Swap, GapPull. Only then does the player apply. | partial |
| **Carriers dialog** | Clarifies which units share an ability, that is, what a move pulls on across several command cards. | partial |
| **System hotkeys dialog** | Control groups, hero selection, inventory grid and the remaining list. Inventory slots swappable by pointer gesture. | partial |
| **Search as a jump tool** | Full text search across all units of all races, switchable between unit and ability name, with race and mode chips. Purely a navigation tool. | yes |
| **Unit list with categories** | Left column of the desktop editor, grouped by category, with a search field. | no |
| **Race tabs, five equal** | Human, Orc, Night Elf, Undead, Neutral, each with its own banner and accent. | no |
| **Mode tabs, Melee and Campaign** | Narrows the unit list, both possible at once. | no |
| **Unit stats panel and description** | Life, mana, attack, armor with matchup table, hero attributes with per level growth. | no |
| **Burger menu** | Below the desktop width all actions move into a drawer. | yes |
| **Deep links** | Race, mode, unit and search term as query, plus `/collisions` and `/resolve`. Sharing and bookmarking work. | yes |
| **Persistence in localStorage** | Nothing is lost on close, without account and without server. | yes |
| **Help dialog with first visit opening** | On the first visit the guide opens by itself, never again unprompted afterwards. Mobile too. | yes |
| **Normalization and factory base at start** | On first open, the full default assignment with resolved cascades and materialized positions. | yes |
| **Deriving a grid layout from a file** | Computes which letter grid a file contains, most frequent letter per cell, QWERTY as fallback. | yes |
| **Mini grids as preview** | Everywhere a position is mentioned, a small 4x3 grid with the marked cell. | partial |

### 7.3 The mobile state and the open work

The mandate is **functional parity**, mobile should do everything desktop does. The mobile flow
is complete, edit, download, mail the file to yourself, retrieve it on the PC. Once the WASM is
on the device from the CDN, the app makes no more network requests. The onboarding also appears
on mobile and must appear exactly once on the first visit.

The rework is not finished, and the following is open on the way to parity. These are tasks, not
design decisions for a thinner mobile path.

- Unit stats panel and unit description are still missing on mobile.
- Race and mode filters are reachable on mobile only through the search dialog, not as permanent
  navigation. The mobile editor currently lists all units of all races in a single chain.
- The grid layout editor still uses HTML5 drag in one place and is operable on touch only via tap
  and key picker, the swap path by finger is missing.
- Undo and redo sit two taps deep in the burger menu on mobile, although they are used most often
  in the core flow and there are no keyboard shortcuts on mobile.
- E2E coverage for mobile is thin, a single spec against many on desktop, and of all things the
  long press drag, the core on mobile, is untested.
- A PWA manifest is missing, so that the app also starts without a network once it was loaded. A
  good addition, not a painful gap.

---

## 8. Competition

**jcfieldsdev/warcraft3-hotkey-editor, the predecessor.** It is the scene's standard
recommendation and it is alive, though stalled in content for about a year. It has solved
collision detection and drag and drop with swap for years, that is state of the art and
therefore does **not** work as a selling point. Its scope advantage is real, it also carries the
official campaigns with their four special races. Where it fails is documented in its own open
issues. Some researches save no position line and therefore never change. Misspelled rawcodes
like `auan` instead of `AUan` break keybinds silently, exactly the class of error a generated
data base rules out and a hand maintained one does not. And toggle states, the `Unbuttonpos`
domain, are missing entirely, while this repo had a dedicated regression for it and holds a test
against it.

**The most solid differentiator is mobile.** The predecessor has no viewport meta tag and not a
single touch or pointer handler, only HTML5 drag. The core action, moving a button, is simply
not triggerable on touch devices.

**tlo9/Warcraft-3-Keybinds** is not an editor but a set of finished files to copy in. It attests
the alternative of adopting a finished preset, and through its Dvorak and QWERTZ files, that
keyboard layout variants are a real topic.

**Blizzard's in game editor** from patch 2.0 is the most relevant competition, because it ships
with the game. It can change keys but not move button positions, and on save it discards what it
does not know. It is at the same time the reason Segment C exists.

---

## 9. User stories

### 9.1 Covered today

1. As a Layout Builder I want to drag a button onto another cell, so that the key matches the
   position.
2. As a Layout Builder I want the `Unbuttonpos` to move along when I move a toggle button, so the
   off state does not stay behind.
3. As a Layout Builder I want to see all grids of a unit side by side, so I finish it in one pass.
4. As a Layout Builder I want to set the key right after moving.
5. As a Layout Builder I want to learn immediately whether a key is already taken on this command
   card and by whom.
6. As a Layout Builder I want to decide whether the grid key is set along on a move.
7. As a Layout Builder I want a rejected move explained with the blocker and the way out.
8. As a Layout Builder I want to see which units share an ability.
9. As a Layout Builder I want to see every planned cascade move individually and with a reason
   before applying.
10. As a Layout Builder I want to undo every change and keep the history across a reload.
11. As a Preset Adopter I want to choose from seven templates with preview.
12. As a Preset Adopter I want to set a letter grid once and apply it to everything.
13. As a Casualty I want to upload my existing file and get it back normalized.
14. As a Casualty I want my state preserved without account and server.
15. As a Casualty I want the exact target folder and the file name warning on export.
16. As a Casualty I want to view and copy the finished text without downloading.
17. As a Rawcode Seeker I want to see the rawcode directly on the unit card.
18. As a Rawcode Seeker I want to full text search across all units, by unit or ability name.
19. As a Layout Builder I want to see at all times how many conflicts remain open.
20. As a Layout Builder I want to see collisions separated by their three kinds.
21. As a Layout Builder I want to share and bookmark a spot in the editor by URL.
22. As a Layout Builder I want to find the full default assignment on first open.
23. As a Layout Builder I want a mini grid with the marked cell at every position reference.
24. As a Layout Builder I want Neutral as a full race tab.
25. As a Mobile Planner I want to swipe from unit to unit.
26. As a Mobile Planner I want to pick up and drag a button by long press.
27. As a Mobile Planner I want to page through a unit's grids as a carousel.
28. As a Mobile Planner I want to find all actions in the burger menu.

### 9.2 Roadmap, not yet implemented

- Undo and redo on mobile with one tap instead of through the burger menu.
- Being able to narrow to a race or to Melee on mobile.
- The mobile long press drag covered by an E2E test.
- Swapping the letters in the grid layout editor by finger as well.
- Being reminded of the read only step on export.
- Being warned before the in game options menu, which can discard positions.
- Getting a brief explanation why shop and inventory items are not bindable via CustomKeys.
- Being able to mark a unit as done, to keep track across around a thousand objects.
- Collisions and resolve page fully designed for mobile.
- Being able to view a unit's stats on mobile too. Lower priority, this is supporting cast.

Sharing a layout as a link would be desirable, but it runs into the serverless architecture and
is open in feasibility.

---

## 10. What this means for any work on the product

**The core is dragging the buttons, unit by unit.** Every unit has to be adjusted individually,
which is why the command card editor is the product. The user looks over every unit, drags
buttons, and the editor shows immediately where that creates new collisions. The typical flow is
default template, then apply grid, then per unit fine work. Anyone who only wants the green
baseline is done in under a minute. The actual job takes 30 to 60 minutes of focused repetition
across hundreds of units, roughly ten seconds per unit in the flow, either just for your own
race, for all races you play, or strictly for every unit there is.

From this it follows hard that a change which makes the drag path slower, shakier or less
reliable is a regression, even if it obeys every rule in `COMPONENTS.md`. And because the session
is long, undo is not supporting cast but a safety net, and persistence is not convenience but a
precondition.

**The technical moat is the cascade algorithm.** For the user the core is the dragging, for the
engineering it is the algorithm that automatically resolves the cascades from shared object ids.
Mind the distinction from mere collision **detection**, which the predecessor has had for years,
that is only the green baseline. The automatic **resolution** is what overwhelmed everyone before
us. It lives deliberately in the tested domain crate, not in the renderer. Anyone touching
cascade or resolver logic touches this moat, with tests.

**Mobile is equal.** The mobile path is to be developed and tested on equal footing, the goal is
full parity (section 7.3). The current state is not there yet, and anyone touching mobile checks
in the browser.

**Navigation is not the main function, but that is no excuse.** Search, unit list and tabs are
jump tools. With a thousand objects, poor navigation is nonetheless a time sink that comes
straight off the session.

**Never assume ignorance in the audience.** They know the game, the custom keys switch and the
`CustomKeyBindings` folder. Any explanation that treats them as beginners is wrong.

**Limits of the format are not bugs.** Shop and inventory items are not bindable via
`CustomKeys.txt`, the community solves that with AutoHotkey. Custom tooltip text no longer exists
in Reforged. Anyone planning that as a feature gap is planning against the format.

**How success is measured.** Not by a user count, but by people spreading the editor of their own
accord, because it simply works for them. The measure is a user statement like the one from Reel,
"This tool fixed a years long small error I had in my hotkeys. Its all done with a single button
press", not an advertising headline.

**The biggest weakness today is the unfinished UI rework.** Whoever prioritizes, prioritizes its
completion, not new side shows.
