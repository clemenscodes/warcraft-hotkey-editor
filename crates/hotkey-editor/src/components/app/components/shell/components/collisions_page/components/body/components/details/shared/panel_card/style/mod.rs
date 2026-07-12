use tw_macro::tw;
// The shared conflict-panel card surface: the tinted, bordered panel both the hotkey/unit-
// position conflict panel and the island conflict panel compose. Each supplies its own body
// region (the caption over its clash layouts, or the affected unit over its ability row), so
// the surface look lives here once. The gap between a body's parts and their centring belongs
// to the body region, not the surface.

classes! {
    base: tw![
        "flex",
        "flex-col",
        "py-6",
        "px-4",
        "min-w-0",
        "border",
        "border-warcraft-blue-deep",
        "rounded-panel",
        "bg-warcraft-bg-mid/45",
    ],
}
