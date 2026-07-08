use super::state::FramedIconStyle;
use tw_macro::tw;

// The frame's constant look: it fills the box its parent hands it, clips its image to
// the rounded corners, and carries the constant blue border. The per-variant overlays
// below choose the radius token, the hover glow (a gold border plus soft glow when an
// ancestor `.group` that is not `:disabled` is hovered), and the empty-placeholder
// panel fill.

classes! {
    base: tw![
        "size-full",
        "overflow-hidden",
        "border",
        "border-warcraft-blue",
    ],
}

states! {
    FramedIconStyle,
    TilePlain => tw!["rounded-tile"],
    TileGlow => tw![
        "rounded-tile",
        "group-[:not(:disabled):hover]:border-warcraft-gold",
        "group-[:not(:disabled):hover]:shadow-glow-soft",
    ],
    ControlPlain => tw!["rounded-control"],
    CardGlow => tw![
        "rounded-card",
        "group-[:not(:disabled):hover]:border-warcraft-gold",
        "group-[:not(:disabled):hover]:shadow-glow-soft",
    ],
    Placeholder => tw![
        "rounded-hairline",
        "bg-warcraft-bg-panel/70",
    ],
}
