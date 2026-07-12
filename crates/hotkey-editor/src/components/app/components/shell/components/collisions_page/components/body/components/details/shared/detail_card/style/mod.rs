use tw_macro::tw;
// The shared collision-detail card surface: the bordered dark panel that fills its slot and
// clips its content. The filled and empty panes compose this one surface and each supply their
// own body region (the filled list, or the centred muted prompt), so the surface look lives
// here once. The gap between a body's parts and any centring/min-height belongs to the body
// region, not the surface.

classes! {
    base: tw![
        "flex",
        "flex-col",
        "self-stretch",
        "w-full",
        "min-w-0",
        "max-h-full",
        "py-4",
        "px-5",
        "border",
        "border-warcraft-blue-deep",
        "rounded-container",
        "bg-panel-dark",
        "overflow-hidden",
    ],
}
