use super::state::CollisionState;
use tw_macro::tw;

// The button surface fills its host container and draws itself as a cqi-scaled drawing:
// every interior layout length resolves against the host box (the host is the query
// context). The border is tuned per band to read as a ~1px hairline: 2.8cqi on the 36px
// compact phone/tablet box, and 1.25cqi on the laptop-and-up box (a viewport clamp) where
// it renders about a pixel across the band and thickens gently toward 4K; radius, icon and
// badge scale uniformly with the box. The gold glow and focus ring are the shared design
// tokens (`shadow-glow-soft` at rest when clear, `shadow-glow` on hover, `shadow-focus` on
// focus) that every gold button wears — not a per-button reinvention.

classes! {
    base: tw![
        "relative",
        "flex",
        "items-center",
        "justify-center",
        "size-full",
        "p-0",
        "rounded-[15cqi]",
        "border-[1.25cqi]",
        "border-warcraft-gold-border",
        "bg-panel-gold-resting",
        "cursor-pointer",
        "transition-[border-color,color,background,box-shadow]",
        "focus:outline-none",
        "focus-visible:border-white",
        "focus-visible:text-white",
        "focus-visible:[--focus-color:var(--color-warcraft-highlight)]",
        "focus-visible:shadow-focus",
    ],
    mobile: tw!["mobile:border-[2.8cqi]"],
    tablet: tw!["tablet:border-[2.8cqi]"],
}

states! {
    CollisionState,
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
