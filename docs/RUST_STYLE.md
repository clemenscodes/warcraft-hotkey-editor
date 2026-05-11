# Rust style rules

These are the Rust coding rules for this project. They are **non-negotiable**
and must never require reminding. Read this file before writing any Rust
code in this repository.

These sit alongside `docs/ARCHITECTURE.md`. Architecture rules dictate
*where* code lives; these dictate *how* it is written. Both must hold.

> Before writing any Rust code: read the relevant source files and apply
> every rule below without exception.

---

## Full semantic names everywhere

Every identifier — types, structs, enums, fields, variables, parameters,
locals, functions — must carry its complete semantic meaning. No
abbreviations, no single letters, no shortened forms. This applies equally
to type-level names and value-level names.

```rust
// WRONG
struct KV { ... }          // abbreviation for "key-value"
struct AbilBuf { ... }     // abbreviation for "ability buffer"
let s = player.get_stats();
let buff = ability.duration;
let hp = hero.life;

// CORRECT
struct KeyValue { ... }
struct AbilityBuffer { ... }
let player_stats = player.get_stats();
let buff_duration = ability.duration;
let hero_life = hero.life;
```

## No section header comments

Do not write comments whose only purpose is to label a block of code (e.g.
`// === Rendering ===`, `// Helpers`, `// Setup`). If a file has grown to
need sections, split it into separate files instead.

## No tuples

Never use tuples in any form: no plain tuples `(u32, u32)`, no tuple
structs `struct Pair(u32, u32)`, no newtype wrappers `struct Meters(f32)`.
Always define a named struct with named fields.

```rust
// WRONG
fn screen_position() -> (f32, f32) { ... }
struct Offset(i32, i32);

// CORRECT
struct ScreenPosition { x: f32, y: f32 }
struct Offset { horizontal: i32, vertical: i32 }
```

## No `print*` functions

A function whose name starts with `print` is a design mistake. If a type
has a human-readable representation, implement `std::fmt::Display` for it.
Call `println!("{}", value)` at the call site.

```rust
// WRONG
fn print_hero_stats(stats: &HeroStats) { println!("{} {}", stats.strength, stats.agility); }

// CORRECT
impl fmt::Display for HeroStats {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.strength, self.agility)
    }
}
```

## Private struct fields with explicit accessors

Struct fields are private by default. Expose read access through a getter
method. Expose write access through a setter method or a builder. Do not
make fields `pub` to skip writing accessors.

```rust
// WRONG
pub struct HeroState {
    pub current_life: f32,
    pub maximum_life: f32,
}

// CORRECT
pub struct HeroState {
    current_life: f32,
    maximum_life: f32,
}

impl HeroState {
    pub fn current_life(&self) -> f32 { self.current_life }
    pub fn maximum_life(&self) -> f32 { self.maximum_life }
    pub fn set_current_life(&mut self, current_life: f32) { self.current_life = current_life; }
}
```

## Assign structs to a variable before passing them

Never construct a struct inline as a function argument. Always assign it
to a named variable first.

```rust
// WRONG
renderer.draw(HeroPortrait { hero_id: 42, scale: 1.0 });

// CORRECT
let hero_portrait = HeroPortrait { hero_id: 42, scale: 1.0 };
renderer.draw(hero_portrait);
```

## No evaluated expressions as arguments

If passing an argument requires evaluating an expression (a method call,
field access, conversion, etc.), assign it to a named variable first. A
function call should only ever receive plain variables as arguments —
never inline expressions. This applies regardless of whether the expression
fits on one line.

```rust
// WRONG
let kern_value = scaled_font.kern(
    font.glyph_id(left_glyph.character),
    font.glyph_id(right_glyph.character),
);

// CORRECT
let left_glyph_id = font.glyph_id(left_glyph.character);
let right_glyph_id = font.glyph_id(right_glyph.character);
let kern_value = scaled_font.kern(left_glyph_id, right_glyph_id);
```

## No inline numeric type suffixes

Never attach a type suffix to a numeric literal (e.g. `0u8`, `1u32`,
`2.0f32`). If the compiler cannot infer the type, annotate the binding
instead.

```rust
// WRONG
let count = 0u32;
let scale = 2.0f32;
let buffer = vec![0u8; size];

// CORRECT
let count: u32 = 0;
let scale: f32 = 2.0;
let buffer: Vec<u8> = vec![0; size];
```

## Prefer struct composition over field copying

When constructing a struct and all fields from another struct are passed
through verbatim, do not copy the individual fields — embed the source
struct as a named sub-field instead.

```rust
// WRONG
let atlas_meta = FontAtlasMeta {
    ascent: metrics.ascent,
    descent: metrics.descent,
    line_height: metrics.line_height,
    glyphs: glyph_metadata,
    kerning,
};

// CORRECT — embed metrics as a sub-struct
struct FontAtlasMeta {
    metrics: FontMetrics,
    glyphs: Vec<GlyphMetadata>,
    kerning: KerningTable,
}

let atlas_meta = FontAtlasMeta {
    metrics,
    glyphs: glyph_metadata,
    kerning,
};
```

## No `verb_noun` free functions

A free function named `verb_noun` (e.g. `render_hero`, `parse_ability`,
`load_icon`) is a design mistake. The noun should be a struct and the verb
should be a method on it.

```rust
// WRONG
fn render_hero(hero: &Hero, canvas: &mut Canvas) { ... }
fn parse_ability(raw: &str) -> Ability { ... }
fn load_icon(path: &Path) -> Icon { ... }

// CORRECT
impl Hero {
    fn render(&self, canvas: &mut Canvas) { ... }
}

impl Ability {
    fn parse(raw: &str) -> Ability { ... }
}

impl Icon {
    fn load(path: &Path) -> Icon { ... }
}
```

## No `as` casts outside `From`/`TryFrom` impls

Never use `as` to cast between numeric types in ordinary code. Use `From`,
`Into`, or `TryFrom` trait conversions instead. The only place `as` is
acceptable is inside the body of a `From` or `TryFrom` impl, where a
lossless or explicitly-checked cast is the implementation of the
conversion.

```rust
// WRONG
let address = base + offset as usize;
let byte = value as u8;

// CORRECT
let address = base + usize::from(offset);
let byte = u8::try_from(value).expect("value fits in u8");

// ACCEPTABLE — inside a From impl body
impl From<RawAddress> for usize {
    fn from(raw: RawAddress) -> usize { raw.0 as usize }
}
```
