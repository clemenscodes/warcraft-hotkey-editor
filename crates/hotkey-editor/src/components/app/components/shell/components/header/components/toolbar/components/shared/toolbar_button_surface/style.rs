use super::state::SurfaceState;
use tw_macro::tw;
// The single source of truth for how a gold toolbar button looks. `classes!` holds the
// chrome shared by every variant — the box, the resting gradient, the hairline border,
// the radius, and the focus ring — drawn in `cqi` off the container the parent hands it,
// so the whole button scales as one drawing. The border is tuned per band to read as a
// ~1px hairline: 2.8cqi on the compact 36px phone/tablet box, and 1.25cqi on the
// laptop-and-up box. The per-variant text color, resting border/glow, and hover treatment
// are layered on top by `states!`.

classes! {
    base: tw![
        "inline-flex",
        "items-center",
        "justify-center",
        "size-full",
        "p-0",
        "border-[1.25cqi]",
        "border-warcraft-gold-border",
        "rounded-[15cqi]",
        "cursor-pointer",
        "bg-panel-gold-resting",
        "transition-[border-color,color,background,box-shadow]",
        "focus:outline-none",
        "kb-focus:border-white",
        "kb-focus:text-white",
        "kb-focus:[--focus-color:var(--color-warcraft-highlight)]",
        "kb-focus:shadow-focus",
    ],
    mobile: tw!["mobile:border-[2.8cqi]"],
    tablet: tw!["tablet:border-[2.8cqi]"],
}

states! {
    SurfaceState,
    Interactive => tw![
        "text-warcraft-text-secondary",
        "hover:border-warcraft-gold",
        "hover:text-warcraft-gold",
        "hover:bg-panel-gold-active",
        "hover:shadow-glow",
    ],
    Attention => tw![
        "text-warcraft-gold",
        "hover:border-warcraft-gold",
        "hover:text-warcraft-gold",
        "hover:bg-panel-gold-active",
        "hover:shadow-glow",
    ],
    Clear => tw![
        "border-warcraft-gold",
        "text-warcraft-gold",
        "shadow-glow-soft",
        "hover:bg-panel-gold-active",
        "hover:shadow-glow",
    ],
}
